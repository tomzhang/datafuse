// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::any::Any;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use common_planners::PlanNode;
use common_streams::SendableDataBlockStream;

use crate::api::rpc::FlightClient;
use crate::pipelines::processors::{EmptyProcessor, IProcessor};
use crate::sessions::FuseQueryContextRef;

pub struct RemoteTransform {
    job_id: String,
    remote_addr: String,
    pub ctx: FuseQueryContextRef,
    pub plan: PlanNode,
    input: Arc<dyn IProcessor>,
}

impl RemoteTransform {
    pub fn try_create(
        ctx: FuseQueryContextRef,
        job_id: String,
        remote_addr: String,
        plan: PlanNode,
    ) -> Result<Self> {
        Ok(Self {
            job_id,
            remote_addr,
            ctx,
            plan,
            input: Arc::new(EmptyProcessor::create()),
        })
    }
}

#[async_trait]
impl IProcessor for RemoteTransform {
    fn name(&self) -> &str {
        "RemoteTransform"
    }

    fn connect_to(&mut self, input: Arc<dyn IProcessor>) -> Result<()> {
        self.input = input;
        Ok(())
    }

    fn inputs(&self) -> Vec<Arc<dyn IProcessor>> {
        vec![self.input.clone()]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn execute(&self) -> Result<SendableDataBlockStream> {
        let mut client = FlightClient::try_create(self.remote_addr.clone()).await?;
        Ok(Box::pin(
            client
                .execute_remote_plan_action(self.job_id.clone(), &self.plan)
                .await?,
        ))
    }
}
