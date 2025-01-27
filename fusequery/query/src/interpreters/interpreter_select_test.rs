// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_select_interpreter() -> anyhow::Result<()> {
    use common_datavalues::*;
    use common_planners::*;
    use futures::stream::StreamExt;

    use crate::interpreters::*;
    use crate::sql::*;

    let ctx =
        crate::tests::try_create_context()?.with_id("cf6db5fe-7595-4d85-97ee-71f051b21cbe")?;

    if let PlanNode::Select(plan) = PlanParser::create(ctx.clone())
        .build_from_sql("select number from numbers_mt(10) where (number+2)<2")?
    {
        let executor = SelectInterpreter::try_create(ctx.clone(), plan)?;
        assert_eq!(executor.name(), "SelectInterpreter");

        let mut stream = executor.execute().await?;
        while let Some(_block) = stream.next().await {}
    } else {
        assert!(false)
    }

    if let PlanNode::Select(plan) =
        PlanParser::create(ctx.clone()).build_from_sql("select 1 + 1, 2 + 2, 3 * 3, 4 * 4")?
    {
        let executor = SelectInterpreter::try_create(ctx.clone(), plan)?;
        assert_eq!(executor.name(), "SelectInterpreter");

        let mut stream = executor.execute().await?;
        if let Some(block) = stream.next().await {
            let block = block?;
            assert_eq!(1, block.num_rows());
            assert_eq!(4, block.num_columns());

            let sc = block.schema().clone();
            let types: Vec<&DataType> = sc.fields().iter().map(|f| f.data_type()).collect();
            assert_eq!(
                vec![
                    &DataType::UInt16,
                    &DataType::UInt16,
                    &DataType::UInt16,
                    &DataType::UInt16
                ],
                types
            );
        }
        while let Some(_block) = stream.next().await {}
    }

    Ok(())
}
