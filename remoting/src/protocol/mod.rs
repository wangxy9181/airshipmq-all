use crate::protocol::remoting_command::{BrokerRegisterRequest, CommandType, RemotingCommand};
use crate::protocol::remoting_command::remoting_command::Command;

mod remoting_command;

impl RemotingCommand {
    pub fn new_broker_register_request(broker_name: impl Into<String>,
                                       broker_addr: impl Into<String>,
                                       cluster_name: impl Into<String>,
                                       ha_server_addr: impl Into<String>,
                                       broker_id: i64,
                                       heartbeat_timeout_mills: i64,
                                       enable_acting_master: bool) -> Self {
        let version = common::version();
        Self {
            version,
            command_type: CommandType::Request as i32,
            command: Some(Command::BrokerRegisterRequest(
                BrokerRegisterRequest {
                    broker_name: broker_name.into(),
                    broker_addr: broker_addr.into(),
                    cluster_name: cluster_name.into(),
                    ha_server_addr: ha_server_addr.into(),
                    broker_id,
                    heartbeat_timeout_mills,
                    enable_acting_master
                }
            )),
            code: Default::default(),
            remark: Default::default(),

        }
    }
}