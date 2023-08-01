#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResponseCode {
    Success = 0,
    SystemError = 1,
    SystemBusy = 2,
    RequestCommandNotSupport = 3,
}
impl ResponseCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResponseCode::Success => "Success",
            ResponseCode::SystemError => "SystemError",
            ResponseCode::SystemBusy => "SystemBusy",
            ResponseCode::RequestCommandNotSupport => "RequestCommandNotSupport",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Success" => Some(Self::Success),
            "SystemError" => Some(Self::SystemError),
            "SystemBusy" => Some(Self::SystemBusy),
            "RequestCommandNotSupport" => Some(Self::RequestCommandNotSupport),
            _ => None,
        }
    }
}
/// Broker 注册请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerRegisterRequest {
    #[prost(string, tag = "1")]
    pub broker_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub broker_addr: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub broker_id: i64,
}
/// Broker 注册返回
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerRegisterResponse {
    #[prost(enumeration = "ResponseCode", tag = "1")]
    pub response_code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// 在网络之间传输的命令
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemotingCommand {
    /// 命令ID
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// 版本
    #[prost(int32, tag = "2")]
    pub version: i32,
    /// 类型
    #[prost(enumeration = "CommandType", tag = "3")]
    pub command_type: i32,
    /// 命令
    #[prost(oneof = "remoting_command::Command", tags = "4, 5")]
    pub command: ::core::option::Option<remoting_command::Command>,
}
/// Nested message and enum types in `RemotingCommand`.
pub mod remoting_command {
    /// 命令
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        #[prost(message, tag = "4")]
        BrokerRegisterRequest(super::BrokerRegisterRequest),
        #[prost(message, tag = "5")]
        BrokerRegisterResponse(super::BrokerRegisterRequest),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommandType {
    Request = 0,
    Response = 1,
}
impl CommandType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CommandType::Request => "Request",
            CommandType::Response => "Response",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Request" => Some(Self::Request),
            "Response" => Some(Self::Response),
            _ => None,
        }
    }
}
