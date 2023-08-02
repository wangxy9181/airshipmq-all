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
    /// code
    #[prost(enumeration = "CommandCode", tag = "4")]
    pub command_code: i32,
    /// remark
    #[prost(string, tag = "5")]
    pub remark: ::prost::alloc::string::String,
    /// 数据
    #[prost(oneof = "remoting_command::Data", tags = "6")]
    pub data: ::core::option::Option<remoting_command::Data>,
}
/// Nested message and enum types in `RemotingCommand`.
pub mod remoting_command {
    /// 数据
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "6")]
        BrokerRegisterRequest(super::BrokerRegisterRequest),
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommandCode {
    Success = 0,
    SystemError = 1,
    SystemBusy = 2,
    RequestCommandNotSupport = 3,
}
impl CommandCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CommandCode::Success => "Success",
            CommandCode::SystemError => "SystemError",
            CommandCode::SystemBusy => "SystemBusy",
            CommandCode::RequestCommandNotSupport => "RequestCommandNotSupport",
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
