// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::collections::VecDeque;
use std::sync::Arc;

use anyhow::Result;
use common_datavalues::DataValue;
use common_infallible::RwLock;
use common_planners::{Partition, Partitions, Statistics};
use uuid::Uuid;

use crate::clusters::{Cluster, ClusterRef};
use crate::datasources::{DataSource, IDataSource, ITable, ITableFunction};
use crate::sessions::Settings;

#[derive(Clone)]
pub struct FuseQueryContext {
    uuid: Arc<RwLock<String>>,
    settings: Settings,
    cluster: Arc<RwLock<ClusterRef>>,
    datasource: Arc<RwLock<Box<dyn IDataSource>>>,
    statistics: Arc<RwLock<Statistics>>,
    partition_queue: Arc<RwLock<VecDeque<Partition>>>,
}

pub type FuseQueryContextRef = Arc<FuseQueryContext>;

impl FuseQueryContext {
    pub fn try_create() -> Result<FuseQueryContextRef> {
        let settings = Settings::create();
        let ctx = FuseQueryContext {
            uuid: Arc::new(RwLock::new(Uuid::new_v4().to_string())),
            settings,
            cluster: Arc::new(RwLock::new(Cluster::empty())),
            datasource: Arc::new(RwLock::new(Box::new(DataSource::try_create()?))),
            statistics: Arc::new(RwLock::new(Statistics::default())),
            partition_queue: Arc::new(RwLock::new(VecDeque::new())),
        };

        ctx.initial_settings()?;
        Ok(Arc::new(ctx))
    }

    pub fn with_cluster(&self, cluster: ClusterRef) -> Result<FuseQueryContextRef> {
        *self.cluster.write() = cluster;
        Ok(Arc::new(self.clone()))
    }

    pub fn with_id(&self, uuid: &str) -> Result<FuseQueryContextRef> {
        *self.uuid.write() = uuid.to_string();
        Ok(Arc::new(self.clone()))
    }

    // ctx.reset will reset the necessary variables in the session
    pub fn reset(&self) -> Result<()> {
        self.statistics.write().clear();
        self.partition_queue.write().clear();
        Ok(())
    }

    // Steal n partitions from the partition pool by the pipeline worker.
    // This also can steal the partitions from distributed node.
    pub fn try_get_partitions(&self, num: usize) -> Result<Partitions> {
        let mut partitions = vec![];
        for _ in 0..num {
            match self.partition_queue.write().pop_back() {
                None => break,
                Some(partition) => {
                    partitions.push(partition);
                }
            }
        }
        Ok(partitions)
    }

    // Update the context partition pool from the pipeline builder.
    pub fn try_set_partitions(&self, partitions: Partitions) -> Result<()> {
        for part in partitions {
            self.partition_queue.write().push_back(part);
        }
        Ok(())
    }

    pub fn try_get_statistics(&self) -> Result<Statistics> {
        let statistics = self.statistics.read();
        Ok(Statistics {
            read_rows: statistics.read_rows,
            read_bytes: statistics.read_bytes,
        })
    }

    pub fn try_set_statistics(&self, val: &Statistics) -> Result<()> {
        *self.statistics.write() = val.clone();
        Ok(())
    }

    pub fn try_get_cluster(&self) -> Result<ClusterRef> {
        let cluster = self.cluster.read();
        Ok(cluster.clone())
    }

    pub fn get_datasource(&self) -> Arc<RwLock<Box<dyn IDataSource>>> {
        self.datasource.clone()
    }

    pub fn get_table(&self, db_name: &str, table_name: &str) -> Result<Arc<dyn ITable>> {
        self.datasource.read().get_table(db_name, table_name)
    }

    pub fn get_table_function(&self, function_name: &str) -> Result<Arc<dyn ITableFunction>> {
        self.datasource.read().get_table_function(function_name)
    }

    pub fn get_settings(&self) -> Result<Vec<DataValue>> {
        self.settings.get_settings()
    }

    pub fn get_id(&self) -> Result<String> {
        Ok(self.uuid.as_ref().read().clone())
    }

    apply_macros! { apply_getter_setter_settings, apply_initial_settings, apply_update_settings,
        ("max_threads", u64, num_cpus::get() as u64, "The maximum number of threads to execute the request. By default, it is determined automatically.".to_string()),
        ("max_block_size", u64, 10000, "Maximum block size for reading".to_string()),
        ("default_db", String, "default".to_string(), "the default database for current session".to_string())
    }
}

impl std::fmt::Debug for FuseQueryContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.settings)
    }
}
