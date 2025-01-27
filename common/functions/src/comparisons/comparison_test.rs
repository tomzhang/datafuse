// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

#[test]
fn test_comparison_function() -> anyhow::Result<()> {
    use std::sync::Arc;

    use common_datablocks::*;
    use common_datavalues::*;
    use pretty_assertions::assert_eq;

    use crate::comparisons::*;
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

    let schema = Arc::new(DataSchema::new(vec![
        DataField::new("a", DataType::Int64, false),
        DataField::new("b", DataType::Int64, false),
    ]));

    let field_a = ColumnFunction::try_create("a").unwrap();
    let field_b = ColumnFunction::try_create("b").unwrap();

    let tests = vec![
        Test {
            name: "eq-passed",
            display: "a = b",
            nullable: false,
            func: ComparisonEqFunction::try_create_func(&[field_a.clone(), field_b.clone()])?,
            block: DataBlock::create(
                schema.clone(),
                vec![
                    Arc::new(Int64Array::from(vec![4, 3, 2, 4])),
                    Arc::new(Int64Array::from(vec![1, 2, 3, 4])),
                ],
            ),
            expect: Arc::new(BooleanArray::from(vec![false, false, false, true])),
            error: "",
        },
        Test {
            name: "gt-passed",
            display: "a > b",
            nullable: false,
            func: ComparisonGtFunction::try_create_func(&[field_a.clone(), field_b.clone()])?,
            block: DataBlock::create(
                schema.clone(),
                vec![
                    Arc::new(Int64Array::from(vec![4, 3, 2, 4])),
                    Arc::new(Int64Array::from(vec![1, 2, 3, 4])),
                ],
            ),
            expect: Arc::new(BooleanArray::from(vec![true, true, false, false])),
            error: "",
        },
        Test {
            name: "gt-eq-passed",
            display: "a >= b",
            nullable: false,
            func: ComparisonGtEqFunction::try_create_func(&[field_a.clone(), field_b.clone()])?,
            block: DataBlock::create(
                schema.clone(),
                vec![
                    Arc::new(Int64Array::from(vec![4, 3, 2, 4])),
                    Arc::new(Int64Array::from(vec![1, 2, 3, 4])),
                ],
            ),
            expect: Arc::new(BooleanArray::from(vec![true, true, false, true])),
            error: "",
        },
        Test {
            name: "lt-passed",
            display: "a < b",
            nullable: false,
            func: ComparisonLtFunction::try_create_func(&[field_a.clone(), field_b.clone()])?,
            block: DataBlock::create(
                schema.clone(),
                vec![
                    Arc::new(Int64Array::from(vec![4, 3, 2, 4])),
                    Arc::new(Int64Array::from(vec![1, 2, 3, 4])),
                ],
            ),
            expect: Arc::new(BooleanArray::from(vec![false, false, true, false])),
            error: "",
        },
        Test {
            name: "lt-eq-passed",
            display: "a <= b",
            nullable: false,
            func: ComparisonLtEqFunction::try_create_func(&[field_a.clone(), field_b.clone()])?,
            block: DataBlock::create(
                schema.clone(),
                vec![
                    Arc::new(Int64Array::from(vec![4, 3, 2, 4])),
                    Arc::new(Int64Array::from(vec![1, 2, 3, 4])),
                ],
            ),
            expect: Arc::new(BooleanArray::from(vec![false, false, true, true])),
            error: "",
        },
        Test {
            name: "not-eq-passed",
            display: "a != b",
            nullable: false,
            func: ComparisonNotEqFunction::try_create_func(&[field_a.clone(), field_b.clone()])?,
            block: DataBlock::create(
                schema.clone(),
                vec![
                    Arc::new(Int64Array::from(vec![4, 3, 2, 4])),
                    Arc::new(Int64Array::from(vec![1, 2, 3, 4])),
                ],
            ),
            expect: Arc::new(BooleanArray::from(vec![true, true, true, false])),
            error: "",
        },
    ];

    for t in tests {
        let func = t.func;
        if let Err(e) = func.eval(&t.block) {
            assert_eq!(t.error, e.to_string());
        }
        func.eval(&t.block)?;

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
