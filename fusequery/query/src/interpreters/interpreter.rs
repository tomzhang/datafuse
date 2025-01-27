// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use anyhow::Result;
use async_trait::async_trait;
use common_streams::SendableDataBlockStream;

#[async_trait]
pub trait IInterpreter: Sync + Send {
    fn name(&self) -> &str;
    async fn execute(&self) -> Result<SendableDataBlockStream>;
}
