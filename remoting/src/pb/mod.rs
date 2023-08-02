mod abi;
mod broker;

pub use abi::*;


impl RemotingCommand {

    pub fn command_not_support(command_id: u64) -> Self {

        Self {
            id: command_id,
            version: common::version(),
            command_type: CommandType::Response as i32,
            command_code: CommandCode::RequestCommandNotSupport as i32,
            remark: CommandCode::RequestCommandNotSupport.as_str_name().into(),
            data: None
        }
    }
}