// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::sync::Arc;

use anyhow::{bail, Result};

use crate::{BooleanArray, DataArrayRef, DataColumnarValue, DataValueLogicOperator};

pub fn data_array_logic_op(
    op: DataValueLogicOperator,
    left: &DataColumnarValue,
    right: &DataColumnarValue,
) -> Result<DataArrayRef> {
    match (left, right) {
        (DataColumnarValue::Array(left_array), DataColumnarValue::Array(right_array)) => match op {
            DataValueLogicOperator::And => {
                array_boolean_op!(left_array, right_array, and, BooleanArray)
            }
            DataValueLogicOperator::Or => {
                array_boolean_op!(left_array, right_array, or, BooleanArray)
            }
        },
        _ => bail!(
            "DataValue Error: Cannot do data_array {}, left:{:?}, right:{:?}",
            op,
            left.data_type(),
            right.data_type()
        ),
    }
}
