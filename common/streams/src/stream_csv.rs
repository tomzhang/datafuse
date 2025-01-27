// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::convert::TryInto;
use std::fs::File;
use std::task::{Context, Poll};

use anyhow::Result;
use common_arrow::arrow::csv;
use common_datablocks::DataBlock;
use common_datavalues::DataSchemaRef;
use csv as csv_crate;
use futures::stream::Stream;

pub struct CsvStream {
    reader: csv_crate::Reader<File>,
}

impl CsvStream {
    pub fn try_create(schema: DataSchemaRef, r: File) -> Result<Self> {
        let reader = csv::Reader::new(r, schema, false, None, 1024, None, None);
        Ok(CsvStream { reader })
    }
}

impl Stream for CsvStream {
    type Item = Result<DataBlock>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let batch = self.reader.next();
        match batch {
            Some(Ok(batch)) => {
                let block = batch.try_into();
                match block {
                    Ok(block) => Poll::Ready(Some(Ok(block))),
                    _ => Poll::Ready(None),
                }
            }
            _ => Poll::Ready(None),
        }
    }
}
