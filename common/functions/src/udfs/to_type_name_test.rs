// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

#[test]
fn test_to_type_name_function() -> anyhow::Result<()> {
    use std::sync::Arc;

    use common_datablocks::*;
    use common_datavalues::*;
    use pretty_assertions::assert_eq;

    use crate::udfs::*;
    use crate::*;

    #[allow(dead_code)]
    struct Test {
        name: &'static str,
        display: &'static str,
        nullable: bool,
        block: DataBlock,
        expect: DataArrayRef,
        error: &'static str,
        func: Box<dyn IFunction>,
    }

    let schema = Arc::new(DataSchema::new(vec![DataField::new(
        "a",
        DataType::Boolean,
        false,
    )]));

    let field_a = ColumnFunction::try_create("a").unwrap();

    let tests = vec![Test {
        name: "to_type_name-example-passed",
        display: "toTypeName(a)",
        nullable: false,
        func: ToTypeNameFunction::try_create(&[field_a.clone()])?,
        block: DataBlock::create(
            schema.clone(),
            vec![Arc::new(BooleanArray::from(vec![true, true, true, false]))],
        ),
        expect: Arc::new(StringArray::from(vec![
            "Boolean", "Boolean", "Boolean", "Boolean",
        ])),
        error: "",
    }];

    for t in tests {
        let func = t.func;
        if let Err(e) = func.eval(&t.block) {
            assert_eq!(t.error, e.to_string());
        }

        // Display check.
        let expect_display = t.display.to_string();
        let actual_display = format!("{}", func);
        assert_eq!(expect_display, actual_display);

        // Nullable check.
        let expect_null = t.nullable;
        let actual_null = func.nullable(t.block.schema())?;
        assert_eq!(expect_null, actual_null);

        let ref v = func.eval(&t.block)?;
        // Type check.
        let expect_type = func.return_type(t.block.schema())?;
        let actual_type = v.data_type();
        assert_eq!(expect_type, actual_type);
        assert_eq!(v.to_array(t.block.num_rows())?.as_ref(), t.expect.as_ref());
    }
    Ok(())
}
