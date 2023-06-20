/// Broker 注册请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerRegisterRequest {
    #[prost(string, tag = "1")]
    pub broker_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub broker_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub cluster_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub ha_server_addr: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub broker_id: i64,
    #[prost(int64, tag = "6")]
    pub heartbeat_timeout_mills: i64,
    #[prost(bool, tag = "7")]
    pub enable_acting_master: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerRegisterResponse {}
/// 在网络之间传输的命令
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemotingCommand {
    /// 版本
    #[prost(int32, tag = "1")]
    pub version: i32,
    /// 类型
    #[prost(enumeration = "CommandType", tag = "2")]
    pub command_type: i32,
    /// code
    #[prost(int32, tag = "3")]
    pub code: i32,
    /// remark
    #[prost(string, tag = "4")]
    pub remark: ::prost::alloc::string::String,
    /// 命令
    #[prost(oneof = "remoting_command::Command", tags = "5, 6")]
    pub command: ::core::option::Option<remoting_command::Command>,
}
/// Nested message and enum types in `RemotingCommand`.
pub mod remoting_command {
    /// 命令
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        #[prost(message, tag = "5")]
        BrokerRegisterRequest(super::BrokerRegisterRequest),
        #[prost(message, tag = "6")]
        BrokerRegisterResponse(super::BrokerRegisterResponse),
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