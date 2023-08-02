use crate::pb::abi::{BrokerRegisterRequest, CommandType, RemotingCommand};
use crate::pb::CommandCode;
use crate::pb::remoting_command::Data;

impl RemotingCommand {
    /// 创建 broker 注册请求
    pub fn new_broker_register_request(broker_name: impl Into<String>,
        broker_addr: impl Into<String>, broker_id: i64) -> Self {
        let version = common::version();
        let id = common::get_command_id();
        Self {
            id,
            version,
            command_type: CommandType::Request as i32,
            command_code: CommandCode::Success as i32,
            remark: String::default(),
            data: Some(
                Data::BrokerRegisterRequest(
                    BrokerRegisterRequest {
                        broker_id,
                        broker_name: broker_name.into(),
                        broker_addr: broker_addr.into(),
                    }
                )
            )
        }
    }

    /// broker 注册成功
    pub fn broker_register_success(command_id: u64) -> Self {
        Self {
            id: command_id,
            version: common::version(),
            command_type: CommandType::Response as i32,
            command_code: CommandCode::Success as i32,
            remark: String::default(),
            data: None
        }
    }

    /// broker 注册失败
    pub fn broker_register_fail(command_id: u64, command_code: CommandCode,
        remark: impl Into<String>) -> Self {
        Self {
            id: command_id,
            version: common::version(),
            command_type: CommandType::Response as i32,
            command_code: command_code as i32,
            remark: remark.into(),
            data: None
        }
    }

}