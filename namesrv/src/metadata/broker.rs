use std::collections::{HashMap, HashSet};
use chrono::Local;
use remoting::RemotingError;

#[derive(Debug, PartialEq, Default)]
pub struct BrokerInfo {
    /// key: brokerName，
    broker_addr_table: HashMap<String, BrokerData>,
    /// key: cluster name, value: broker name
    cluster_addr_table: HashMap<String, HashSet<String>>,
    broker_live_table: HashMap<BrokerAddrInfo, BrokerLiveInfo>,
}

#[derive(Debug, PartialEq)]
pub struct BrokerData {
    cluster_name: String,
    broker_name: String,
    /// key: broker id，value: 单实例 broker 地址
    broker_addrs: HashMap<i64, String>,
    zone_name: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct BrokerLiveInfo {
    last_update_timestamp: i64,
    heartbeat_timeout_mills: i64,
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct BrokerAddrInfo {
    cluster_name: String,
    broker_addr: String,
}

impl BrokerInfo {
    /// 创建一个默认的 BrokerInfo
    pub fn new() -> Self {
        Self::default()
    }
    /// 存储 broker info
    pub fn store_broker_info(&mut self,
                             cluster_name: String,
                             broker_name: String,
                             broker_addr: String,
                             broker_id: i64,
                             zone_name: Option<String>,
                             heartbeat_timeout_mills: i64) {
        // broer addr table
        let broker_data = BrokerData::new(cluster_name.clone(),
                                          broker_name.clone(),
                                          broker_addr, broker_id, zone_name);
        self.broker_addr_table.insert(broker_name.clone(), broker_data);
        // cluster addr table
        self.cluster_addr_table.entry(cluster_name.clone())
            .or_insert(HashSet::new())
            .insert(broker_name.clone());
        // broker live table
        let broker_addr_info = BrokerAddrInfo::new(cluster_name.clone(), broker_name.clone());
        let broker_live_info = BrokerLiveInfo::new(heartbeat_timeout_mills);
        self.broker_live_table.insert(broker_addr_info, broker_live_info);
    }
}

impl BrokerData {
    pub fn new(cluster_name: String,
               broker_name: String,
               broker_addr: String,
               broker_id: i64,
               zone_name: Option<String>) -> Self {
        let mut broker_data = Self {
            cluster_name,
            broker_name,
            broker_addrs: HashMap::new(),
            zone_name,
        };
        broker_data.broker_addrs.insert(broker_id, broker_addr.into());
        broker_data
    }
}

impl BrokerAddrInfo {
    /// 创建 broker addr info
    pub fn new(cluster_name: String, broker_addr: String) -> Self {
        Self {
            cluster_name,
            broker_addr,
        }
    }
}

impl BrokerLiveInfo {
    /// 创建一个 broker live info
    pub fn new(heartbeat_timeout_mills: i64) -> Self {
        Self {
            last_update_timestamp: Local::now().timestamp_millis(),
            heartbeat_timeout_mills,
        }
    }
}

#[cfg(test)]
mod tests {

    use anyhow::Result;
    use super::*;

    #[tokio::test]
    async fn store_broker_info_should_work() -> Result<()> {
        let cluster_name = "cluster_name";
        let broker_name = "broker_name";
        let broker_id = 1i64;
        let broker_addr = "broker_addr";

        let mut broker_info = BrokerInfo::new();
        broker_info.store_broker_info(cluster_name.to_string(),
                                      broker_name.to_string(),
                                      broker_addr.to_string(),
                                      broker_id,
                                      None,
                                      3000);

        assert_eq!(broker_info.broker_addr_table.len(), 1);

        Ok(())
    }
}