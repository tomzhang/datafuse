// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::any::Any;

use anyhow::{bail, Result};
use async_trait::async_trait;
use common_datavalues::DataSchemaRef;
use common_planners::{PlanNode, ReadDataSourcePlan, TableOptions};
use common_streams::SendableDataBlockStream;

use crate::datasources::ITable;
use crate::sessions::FuseQueryContextRef;

#[allow(dead_code)]
pub struct RemoteTable {
    db: String,
    name: String,
    schema: DataSchemaRef,
}

impl RemoteTable {
    #[allow(dead_code)]
    pub fn try_create(
        db: String,
        name: String,
        schema: DataSchemaRef,
        _options: TableOptions,
    ) -> Result<Box<dyn ITable>> {
        let table = Self { db, name, schema };
        Ok(Box::new(table))
    }
}

#[async_trait]
impl ITable for RemoteTable {
    fn name(&self) -> &str {
        &self.name
    }

    fn engine(&self) -> &str {
        "remote"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn schema(&self) -> Result<DataSchemaRef> {
        Ok(self.schema.clone())
    }

    fn read_plan(
        &self,
        _ctx: FuseQueryContextRef,
        _push_down_plan: PlanNode,
    ) -> Result<ReadDataSourcePlan> {
        bail!("RemoteTable read_plan not yet implemented")
    }

    async fn read(&self, _ctx: FuseQueryContextRef) -> Result<SendableDataBlockStream> {
        bail!("RemoteTable read not yet implemented")
    }
}
