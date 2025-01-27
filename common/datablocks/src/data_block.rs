// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::convert::TryInto;
use std::sync::Arc;

use anyhow::Result;
use common_arrow::arrow;
use common_arrow::arrow::record_batch::RecordBatch;
use common_datavalues::{DataArrayRef, DataSchema, DataSchemaRef};

#[derive(Debug, Clone)]
pub struct DataBlock {
    schema: DataSchemaRef,
    columns: Vec<DataArrayRef>,
}

impl DataBlock {
    pub fn create(schema: DataSchemaRef, columns: Vec<DataArrayRef>) -> Self {
        DataBlock { schema, columns }
    }

    pub fn empty() -> Self {
        DataBlock {
            schema: Arc::new(DataSchema::empty()),
            columns: vec![],
        }
    }

    pub fn empty_with_schema(schema: DataSchemaRef) -> Self {
        let mut columns = vec![];
        for f in schema.fields().iter() {
            columns.push(arrow::array::new_empty_array(f.data_type()))
        }
        DataBlock { schema, columns }
    }

    pub fn is_empty(&self) -> bool {
        self.num_columns() == 0 || self.num_rows() == 0
    }

    pub fn schema(&self) -> &DataSchemaRef {
        &self.schema
    }

    pub fn num_rows(&self) -> usize {
        self.columns[0].data().len()
    }

    pub fn num_columns(&self) -> usize {
        self.columns.len()
    }

    pub fn column(&self, index: usize) -> &DataArrayRef {
        &self.columns[index]
    }

    pub fn column_by_name(&self, name: &str) -> Result<&DataArrayRef> {
        if name == "*" {
            Ok(&self.columns[0])
        } else {
            let idx = self.schema.index_of(name)?;
            Ok(&self.columns[idx])
        }
    }
}

impl TryInto<arrow::record_batch::RecordBatch> for DataBlock {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<RecordBatch, Self::Error> {
        Ok(arrow::record_batch::RecordBatch::try_new(
            self.schema.clone(),
            self.columns.clone(),
        )?)
    }
}

impl TryInto<DataBlock> for arrow::record_batch::RecordBatch {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<DataBlock, Self::Error> {
        Ok(DataBlock::create(self.schema(), Vec::from(self.columns())))
    }
}
