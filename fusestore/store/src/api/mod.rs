// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

// The api module only used for internal communication, such as GRPC between cluster and the managed HTTP REST API.

pub mod rpc;
mod rpc_service;

pub use rpc_service::RpcService;
