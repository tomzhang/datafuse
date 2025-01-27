// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

#[test]
fn test_rewriter_plan() -> anyhow::Result<()> {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[allow(dead_code)]
    struct RewriteTest {
        name: &'static str,
        exprs: Vec<ExpressionPlan>,
        expect_str: &'static str,
        error_msg: &'static str,
    }

    let tests = vec![
        RewriteTest{
            name : "Cyclic",
            exprs: vec![
                    Box::new(ExpressionPlan::Function {
                        op: "plus".to_string(),
                        args: vec![
                            lit(1i32),
                            col("z")
                        ],
                    }).alias("x"),
                    Box::new(ExpressionPlan::Function {
                        op: "plus".to_string(),
                        args: vec![
                            lit(1i32),
                            col("x")
                        ],
                    }).alias("y"),
                    Box::new(ExpressionPlan::Function {
                        op: "plus".to_string(),
                        args: vec![
                            lit(1i32),
                            col("y")
                        ],
                    }).alias("z"),
            ],
            expect_str: "",
            error_msg : "Planner Error: Cyclic aliases: x",
        },

        RewriteTest{
            name : "Duplicate aliases",
            exprs: vec![
                    Box::new(ExpressionPlan::Function {
                        op: "plus".to_string(),
                        args: vec![
                            lit(1i32),
                            col("z")
                        ],
                    }).alias("x"),
                    Box::new(ExpressionPlan::Function {
                        op: "plus".to_string(),
                        args: vec![
                            lit(1i32),
                            col("y")
                        ],
                    }).alias("x"),
            ],
            expect_str: "",
            error_msg : "Planner Error: Different expressions with the same alias x",
        },

        RewriteTest{
            name: "normal",
            exprs: vec![
                col("x"),
                Box::new(ExpressionPlan::Function {
                        op: "add".to_string(),
                        args: vec![
                            lit(1i32),
                            col("x")
                        ],
                    }).alias("y"),
                ExpressionPlan::Function {
                    op: "multiply".to_string(),
                    args: vec![
                        col("y"),
                        col("y"),
                    ],
                },
            ],
            expect_str: "[x, add([1, x]) as y, multiply([add([1, x]), add([1, x])])]",
            error_msg: "",
        },

        RewriteTest{
            name: "normal2",
            exprs: vec![
                    Box::new(ExpressionPlan::Function {
                        op: "add".to_string(),
                        args: vec![
                            lit(1i32),
                            lit(1i64),
                        ],
                    }).alias("x"),
                    Box::new(ExpressionPlan::Function {
                        op: "add".to_string(),
                        args: vec![
                            lit(1i32),
                            col("x")
                        ],
                    }).alias("y"),
                ExpressionPlan::Function {
                    op: "multiply".to_string(),
                    args: vec![
                        col("x"),
                        col("y")
                    ],
                },
            ],
            expect_str: "[add([1, 1]) as x, add([1, add([1, 1])]) as y, multiply([add([1, 1]), add([1, add([1, 1])])])]",
            error_msg: "",
        },
        RewriteTest{
            name: "x+1->x",
            exprs: vec![
                Box::new(ExpressionPlan::Function {
                    op: "add".to_string(),
                    args: vec![
                        col("x"),
                        lit(1i64),
                    ],
                }).alias("x"),
                Box::new(ExpressionPlan::Function {
                    op: "add".to_string(),
                    args: vec![
                        lit(1i32),
                        col("x")
                    ],
                }).alias("y"),
                ExpressionPlan::Function {
                    op: "multiply".to_string(),
                    args: vec![
                        col("x"),
                        col("y")
                    ],
                },
            ],
            expect_str: "[add([x, 1]) as x, add([1, add([x, 1])]) as y, multiply([add([x, 1]), add([1, add([x, 1])])])]",
            error_msg: "",
        },
    ];

    for t in tests {
        let result = PlanRewriter::exprs_extract_aliases(t.exprs);
        match &result {
            Ok(v) => assert_eq!(t.expect_str, format!("{:?}", v), "in test_case {}", t.name),
            Err(e) => assert_eq!(t.error_msg, e.to_string(), "in test_case {}", t.name),
        }
    }

    Ok(())
}
