// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::fmt;

use anyhow::{ensure, Result};
use common_datablocks::DataBlock;
use common_datavalues::{
    self as datavalues, DataColumnarValue, DataSchema, DataType, DataValueLogicOperator,
};

use crate::logics::{LogicAndFunction, LogicOrFunction};
use crate::{FactoryFuncRef, IFunction};

#[derive(Clone)]
pub struct LogicFunction {
    depth: usize,
    op: DataValueLogicOperator,
    left: Box<dyn IFunction>,
    right: Box<dyn IFunction>,
    saved: Option<DataColumnarValue>,
}

impl LogicFunction {
    pub fn register(map: FactoryFuncRef) -> Result<()> {
        let mut map = map.write();
        map.insert("and", LogicAndFunction::try_create_func);
        map.insert("or", LogicOrFunction::try_create_func);
        Ok(())
    }

    pub fn try_create_func(
        op: DataValueLogicOperator,
        args: &[Box<dyn IFunction>],
    ) -> Result<Box<dyn IFunction>> {
        ensure!(
            args.len() == 2,
            "Function Error: Logic function {} args length must be 2",
            op
        );

        Ok(Box::new(LogicFunction {
            depth: 0,
            op,
            left: args[0].clone(),
            right: args[1].clone(),
            saved: None,
        }))
    }
}

impl IFunction for LogicFunction {
    fn return_type(&self, _input_schema: &DataSchema) -> Result<DataType> {
        Ok(DataType::Boolean)
    }

    fn nullable(&self, _input_schema: &DataSchema) -> Result<bool> {
        Ok(false)
    }

    fn eval(&self, block: &DataBlock) -> Result<DataColumnarValue> {
        Ok(DataColumnarValue::Array(datavalues::data_array_logic_op(
            self.op.clone(),
            &self.left.eval(block)?,
            &self.right.eval(block)?,
        )?))
    }

    fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
    }

    fn is_aggregator(&self) -> bool {
        self.left.is_aggregator() || self.right.is_aggregator()
    }
}

impl fmt::Display for LogicFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.op, self.right)
    }
}
