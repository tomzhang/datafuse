// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use anyhow::{bail, Result};

use crate::{DataValue, DataValueAggregateOperator};

pub fn data_value_aggregate_op(
    op: DataValueAggregateOperator,
    left: DataValue,
    right: DataValue,
) -> Result<DataValue> {
    Ok(match (&left, &right) {
        (DataValue::Null, _) => right,
        (_, DataValue::Null) => left,
        (DataValue::Int8(lhs), DataValue::Int8(rhs)) => match op {
            DataValueAggregateOperator::Min => typed_data_value_min_max!(lhs, rhs, Int8, min),
            DataValueAggregateOperator::Max => typed_data_value_min_max!(lhs, rhs, Int8, max),
            DataValueAggregateOperator::Sum => typed_data_value_add!(lhs, rhs, Int8, i8),
            DataValueAggregateOperator::Count => DataValue::UInt64(Some(1)),
            _ => {
                bail!(format!(
                    "DataValue Error: Unsupported data_value_{} for data type: left:{:?}, right:{:?}",
                    op,
                    left.data_type(),
                    right.data_type()
                ));
            }
        },
        (DataValue::Int16(lhs), DataValue::Int16(rhs)) => match op {
            DataValueAggregateOperator::Min => typed_data_value_min_max!(lhs, rhs, Int16, min),
            DataValueAggregateOperator::Max => typed_data_value_min_max!(lhs, rhs, Int16, max),
            DataValueAggregateOperator::Sum => typed_data_value_add!(lhs, rhs, Int16, i16),
            DataValueAggregateOperator::Count => DataValue::UInt64(Some(1)),
            _ => {
                bail!(format!(
                    "DataValue Error: Unsupported data_value_{} for data type: left:{:?}, right:{:?}",
                    op,
                    left.data_type(),
                    right.data_type()
                ));
            }
        },
        (DataValue::Int32(lhs), DataValue::Int32(rhs)) => match op {
            DataValueAggregateOperator::Min => typed_data_value_min_max!(lhs, rhs, Int32, min),
            DataValueAggregateOperator::Max => typed_data_value_min_max!(lhs, rhs, Int32, max),
            DataValueAggregateOperator::Sum => typed_data_value_add!(lhs, rhs, Int32, i32),
            DataValueAggregateOperator::Count => DataValue::UInt64(Some(1)),
            _ => {
                bail!(format!(
                    "DataValue Error: Unsupported data_value_{} for data type: left:{:?}, right:{:?}",
                    op,
                    left.data_type(),
                    right.data_type()
                ));
            }
        },
        (DataValue::Int64(lhs), DataValue::Int64(rhs)) => match op {
            DataValueAggregateOperator::Min => typed_data_value_min_max!(lhs, rhs, Int64, min),
            DataValueAggregateOperator::Max => typed_data_value_min_max!(lhs, rhs, Int64, max),
            DataValueAggregateOperator::Sum => typed_data_value_add!(lhs, rhs, Int64, i64),
            DataValueAggregateOperator::Count => DataValue::UInt64(Some(1)),
            _ => {
                bail!(format!(
                    "DataValue Error: Unsupported data_value_{} for data type: left:{:?}, right:{:?}",
                    op,
                    left.data_type(),
                    right.data_type()
                ));
            }
        },
        (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => match op {
            DataValueAggregateOperator::Min => typed_data_value_min_max!(lhs, rhs, UInt8, min),
            DataValueAggregateOperator::Max => typed_data_value_min_max!(lhs, rhs, UInt8, max),
            DataValueAggregateOperator::Sum => typed_data_value_add!(lhs, rhs, UInt8, u8),
            DataValueAggregateOperator::Count => DataValue::UInt64(Some(1)),
            _ => {
                bail!(format!(
                    "DataValue Error: Unsupported data_value_{} for data type: left:{:?}, right:{:?}",
                    op,
                    left.data_type(),
                    right.data_type()
                ));
            }
        },
        (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => match op {
            DataValueAggregateOperator::Min => typed_data_value_min_max!(lhs, rhs, UInt16, min),
            DataValueAggregateOperator::Max => typed_data_value_min_max!(lhs, rhs, UInt16, max),
            DataValueAggregateOperator::Sum => typed_data_value_add!(lhs, rhs, UInt16, u16),
            DataValueAggregateOperator::Count => DataValue::UInt64(Some(1)),
            _ => {
                bail!(format!(
                    "DataValue Error: Unsupported data_value_{} for data type: left:{:?}, right:{:?}",
                    op,
                    left.data_type(),
                    right.data_type()
                ));
            }
        },
        (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => match op {
            DataValueAggregateOperator::Min => typed_data_value_min_max!(lhs, rhs, UInt32, min),
            DataValueAggregateOperator::Max => typed_data_value_min_max!(lhs, rhs, UInt32, max),
            DataValueAggregateOperator::Sum => typed_data_value_add!(lhs, rhs, UInt32, u32),
            DataValueAggregateOperator::Count => DataValue::UInt64(Some(1)),
            _ => {
                bail!(format!(
                    "DataValue Error: Unsupported data_value_{} for data type: left:{:?}, right:{:?}",
                    op,
                    left.data_type(),
                    right.data_type()
                ));
            }
        },
        (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => match op {
            DataValueAggregateOperator::Min => typed_data_value_min_max!(lhs, rhs, UInt64, min),
            DataValueAggregateOperator::Max => typed_data_value_min_max!(lhs, rhs, UInt64, max),
            DataValueAggregateOperator::Sum => typed_data_value_add!(lhs, rhs, UInt64, u64),
            DataValueAggregateOperator::Count => DataValue::UInt64(Some(1)),
            _ => {
                bail!(format!(
                    "DataValue Error: Unsupported data_value_{} for data type: left:{:?}, right:{:?}",
                    op,
                    left.data_type(),
                    right.data_type()
                ));
            }
        },
        (DataValue::Float32(lhs), DataValue::Float32(rhs)) => match op {
            DataValueAggregateOperator::Min => typed_data_value_min_max!(lhs, rhs, Float32, min),
            DataValueAggregateOperator::Max => typed_data_value_min_max!(lhs, rhs, Float32, max),
            DataValueAggregateOperator::Sum => typed_data_value_add!(lhs, rhs, Float32, f32),
            DataValueAggregateOperator::Count => DataValue::UInt64(Some(1)),
            _ => {
                bail!(format!(
                    "DataValue Error: Unsupported data_value_{} for data type: left:{:?}, right:{:?}",
                    op,
                    left.data_type(),
                    right.data_type()
                ));
            }
        },
        (DataValue::Float64(lhs), DataValue::Float64(rhs)) => match op {
            DataValueAggregateOperator::Min => typed_data_value_min_max!(lhs, rhs, Float64, min),
            DataValueAggregateOperator::Max => typed_data_value_min_max!(lhs, rhs, Float64, max),
            DataValueAggregateOperator::Sum => typed_data_value_add!(lhs, rhs, Float64, f64),
            DataValueAggregateOperator::Count => DataValue::UInt64(Some(1)),
            _ => {
                bail!(format!(
                    "DataValue Error: Unsupported data_value_{} for data type: left:{:?}, right:{:?}",
                    op,
                    left.data_type(),
                    right.data_type()
                ));
            }
        },
        (DataValue::String(lhs), DataValue::String(rhs)) => match op {
            DataValueAggregateOperator::Min => {
                typed_data_value_min_max_string!(lhs, rhs, String, min)
            }
            DataValueAggregateOperator::Max => {
                typed_data_value_min_max_string!(lhs, rhs, String, max)
            }
            _ => {
                bail!(format!(
                    "DataValue Error: Unsupported data_value_{} for data type: left:{:?}, right:{:?}",
                    op,
                    left.data_type(),
                    right.data_type()
                ));
            }
        },
        _ => {
            bail!(format!(
                "DataValue Error: Unsupported data_value_{} for data type: left:{:?}, right:{:?}",
                op,
                left.data_type(),
                right.data_type()
            ));
        }
    })
}
