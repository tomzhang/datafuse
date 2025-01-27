// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use common_planners::CreateTablePlan;
use common_streams::{DataBlockStream, SendableDataBlockStream};

use crate::interpreters::IInterpreter;
use crate::sessions::FuseQueryContextRef;

pub struct CreateTableInterpreter {
    ctx: FuseQueryContextRef,
    plan: CreateTablePlan,
}

impl CreateTableInterpreter {
    pub fn try_create(
        ctx: FuseQueryContextRef,
        plan: CreateTablePlan,
    ) -> Result<Arc<dyn IInterpreter>> {
        Ok(Arc::new(CreateTableInterpreter { ctx, plan }))
    }
}

#[async_trait]
impl IInterpreter for CreateTableInterpreter {
    fn name(&self) -> &str {
        "CreateInterpreter"
    }

    async fn execute(&self) -> Result<SendableDataBlockStream> {
        let datasource = self.ctx.get_datasource();
        let database = datasource.read().get_database(self.plan.db.as_str())?;
        database.create_table(self.plan.clone())?;

        Ok(Box::pin(DataBlockStream::create(
            self.plan.schema.clone(),
            None,
            vec![],
        )))
    }
}
