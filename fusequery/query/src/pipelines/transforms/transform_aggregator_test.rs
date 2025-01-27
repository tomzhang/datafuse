// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_transform_aggregator() -> anyhow::Result<()> {
    use std::sync::Arc;

    use common_datavalues::*;
    use common_planners::{self, *};
    use futures::stream::StreamExt;
    use pretty_assertions::assert_eq;

    use crate::pipelines::processors::*;
    use crate::pipelines::transforms::*;

    let ctx = crate::tests::try_create_context()?;
    let test_source = crate::tests::NumberTestData::create(ctx.clone());

    let aggr_exprs = vec![add(sum(col("number")), lit(2u64))];

    let aggr_partial = PlanBuilder::create(test_source.number_schema_for_test()?)
        .aggregate_partial(aggr_exprs.clone(), vec![])?
        .build()?;
    let aggr_final = PlanBuilder::create(test_source.number_schema_for_test()?)
        .aggregate_final(aggr_exprs.clone(), vec![])?
        .build()?;

    let mut pipeline = Pipeline::create();
    let a = test_source.number_source_transform_for_test(16)?;
    pipeline.add_source(Arc::new(a))?;
    pipeline.add_simple_transform(|| {
        Ok(Box::new(AggregatorPartialTransform::try_create(
            aggr_partial.schema(),
            aggr_exprs.clone(),
        )?))
    })?;
    pipeline.merge_processor()?;
    pipeline.add_simple_transform(|| {
        Ok(Box::new(AggregatorFinalTransform::try_create(
            aggr_final.schema(),
            aggr_exprs.clone(),
        )?))
    })?;

    let mut stream = pipeline.execute().await?;
    while let Some(v) = stream.next().await {
        let v = v?;
        if v.num_rows() > 0 {
            let actual = v.column(0).as_any().downcast_ref::<UInt64Array>().unwrap();
            let expect = &UInt64Array::from(vec![122]);
            assert_eq!(expect.clone(), actual.clone());
        }
    }
    Ok(())
}
