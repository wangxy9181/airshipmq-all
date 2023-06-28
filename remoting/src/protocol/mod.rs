use std::sync::atomic::{AtomicU64, Ordering};
pub use remoting_command::*;
pub use remoting_command::remoting_command::*;

mod remoting_command;

static COMMAND_ID: AtomicU64 = AtomicU64::new(1);

impl RemotingCommand {
    /// 创建错误 command
    pub fn new_error_command(response_code: ResponseCode, remark: impl Into<String>) -> Self {
        let version = common::version();
        Self {
            version,
            command_type: CommandType::Response as i32,
            response_code: response_code as i32,
            remark: remark.into(),
            ..Default::default()
        }
    }
    /// 创建成功返回
    pub fn new_success_response() -> Self {
        let version = common::version();
        Self {
            version,
            command_type: CommandType::Response as i32,
            response_code: ResponseCode::Success as i32,
            ..Default::default()
        }
    }

    /// 创建错误 command
    pub fn new_error_command_with_id(response_code: ResponseCode, remark: impl Into<String>,
                                     id: u64) -> Self {
        let version = common::version();
        Self {
            id,
            version,
            command_type: CommandType::Response as i32,
            response_code: response_code as i32,
            remark: remark.into(),
            ..Default::default()
        }
    }
    /// 创建成功返回
    pub fn new_success_response_with_id(id: u64) -> Self {
        let version = common::version();
        Self {
            id,
            version,
            command_type: CommandType::Response as i32,
            response_code: ResponseCode::Success as i32,
            ..Default::default()
        }
    }

    /// 创建 broker 注册请求
    pub fn new_broker_register_request(broker_name: impl Into<String>,
                                       broker_addr: impl Into<String>,
                                       cluster_name: impl Into<String>,
                                       ha_server_addr: impl Into<String>,
                                       broker_id: i64,
                                       heartbeat_timeout_mills: i64,
                                       enable_acting_master: bool) -> Self {
        let version = common::version();
        Self {
            id: COMMAND_ID.fetch_add(1, Ordering::Acquire),
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
            response_code: Default::default(),
            remark: Default::default(),
        }
    }

}