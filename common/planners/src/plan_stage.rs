// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::sync::Arc;

use anyhow::Result;
use common_datavalues::DataSchemaRef;

use crate::PlanNode;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum StageState {
    Normal,
    Through,
    SortMerge,
    GroupByMerge,
    AggregatorMerge,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct StagePlan {
    pub uuid: String,
    pub id: usize,
    pub state: StageState,
    pub input: Arc<PlanNode>,
}

impl StagePlan {
    pub fn schema(&self) -> DataSchemaRef {
        self.input.schema()
    }

    pub fn input(&self) -> Arc<PlanNode> {
        self.input.clone()
    }

    pub fn set_input(&mut self, input: &PlanNode) -> Result<()> {
        self.input = Arc::new(input.clone());
        Ok(())
    }
}
