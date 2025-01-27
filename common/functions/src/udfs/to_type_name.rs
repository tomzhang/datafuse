// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::fmt;

use anyhow::{ensure, Result};
use common_datablocks::DataBlock;
use common_datavalues::{DataColumnarValue, DataSchema, DataType, DataValue};

use crate::IFunction;

#[derive(Clone)]
pub struct ToTypeNameFunction {
    arg: Box<dyn IFunction>,
}

impl ToTypeNameFunction {
    pub fn try_create(args: &[Box<dyn IFunction>]) -> Result<Box<dyn IFunction>> {
        ensure!(
            args.len() == 1,
            "The argument size of function database must be one",
        );

        Ok(Box::new(ToTypeNameFunction {
            arg: args[0].clone(),
        }))
    }
}

impl IFunction for ToTypeNameFunction {
    fn return_type(&self, _input_schema: &DataSchema) -> Result<DataType> {
        Ok(DataType::Utf8)
    }

    fn nullable(&self, _input_schema: &DataSchema) -> Result<bool> {
        Ok(false)
    }

    fn eval(&self, block: &DataBlock) -> Result<DataColumnarValue> {
        let type_name = format!("{}", self.arg.return_type(block.schema())?);
        Ok(DataColumnarValue::Scalar(DataValue::String(Some(
            type_name,
        ))))
    }
}

impl fmt::Display for ToTypeNameFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "toTypeName({})", self.arg)
    }
}
