#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingReq {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushReq {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullReq {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    /// int64, not uint64
    #[prost(int64, tag = "2")]
    pub offset: i64,
    #[prost(uint32, tag = "3")]
    pub limit: u32,
    #[prost(uint32, tag = "13")]
    pub conn0_ref: u32,
    #[prost(uint32, tag = "14")]
    pub conn1_ref: u32,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullRep {
    #[prost(message, repeated, tag = "1")]
    pub msgs: ::prost::alloc::vec::Vec<Msg>,
    #[prost(uint32, tag = "13")]
    pub conn0_ref: u32,
    #[prost(uint32, tag = "14")]
    pub conn1_ref: u32,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqReq {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub payload: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "12")]
    pub header: ::core::option::Option<Header>,
    #[prost(uint32, tag = "13")]
    pub conn0_ref: u32,
    #[prost(uint32, tag = "14")]
    pub conn1_ref: u32,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqRep {
    #[prost(string, tag = "1")]
    pub payload: ::prost::alloc::string::String,
    #[prost(uint32, tag = "13")]
    pub conn0_ref: u32,
    #[prost(uint32, tag = "14")]
    pub conn1_ref: u32,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthReq {
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    #[prost(uint32, tag = "13")]
    pub conn0_ref: u32,
    #[prost(uint32, tag = "14")]
    pub conn1_ref: u32,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRep {
    #[prost(uint32, tag = "13")]
    pub conn0_ref: u32,
    #[prost(uint32, tag = "14")]
    pub conn1_ref: u32,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OkRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorRep {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ok2Rep {
    #[prost(uint32, tag = "13")]
    pub conn0_ref: u32,
    #[prost(uint32, tag = "14")]
    pub conn1_ref: u32,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error2Rep {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    #[prost(uint32, tag = "13")]
    pub conn0_ref: u32,
    #[prost(uint32, tag = "14")]
    pub conn1_ref: u32,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterFrontendReq {
    #[prost(uint32, tag = "1")]
    pub http_port: u32,
    #[prost(uint32, tag = "2")]
    pub https_port: u32,
    #[prost(string, tag = "3")]
    pub public_ip: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterFrontendRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterBackendReq {
    #[prost(uint32, tag = "1")]
    pub http_port: u32,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterBackendRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterServerReq {
    #[prost(uint32, tag = "1")]
    pub http_port: u32,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterServerRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRoutesReq {
    #[prost(string, repeated, tag = "1")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRoutesRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRoutesReq {
    #[prost(string, repeated, tag = "1")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRoutesRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoutesReq {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoutesRep {
    #[prost(message, repeated, tag = "1")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteAddedMsg {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteDeletedMsg {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteStatusChangedMsg {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub is_healthy: bool,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignFrontendReq {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignFrontendRep {
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocateTopicReq {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocateTopicRep {
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveIpReq {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveIpRep {
    #[prost(string, tag = "1")]
    pub ip: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    #[prost(uint64, tag = "1")]
    pub offset: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(string, tag = "1")]
    pub agent: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub is_healthy: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgType {
    /// unused, placeholder for some impls
    Unknown = 0,
    PingReq = 1,
    PingRep = 2,
    PushReq = 3,
    PushRep = 4,
    PullReq = 5,
    PullRep = 6,
    ReqReq = 7,
    ReqRep = 8,
    AuthReq = 27,
    AuthRep = 28,
    OkRep = 29,
    ErrorRep = 30,
    Ok2Rep = 31,
    Error2Rep = 32,
    RegisterFrontendReq = 65,
    RegisterFrontendRep = 66,
    RegisterBackendReq = 67,
    RegisterBackendRep = 68,
    RegisterServerReq = 69,
    RegisterServerRep = 70,
    AddRoutesReq = 91,
    AddRoutesRep = 92,
    /// DELETE_ROUTES_REQ := 93;
    /// DELETE_ROUTES_REP := 94;
    GetRoutesReq = 95,
    GetRoutesRep = 96,
    RouteAddedMsg = 100,
    RouteDeletedMsg = 101,
    RouteStatusChangedMsg = 102,
    AssignFrontendReq = 111,
    AssignFrontendRep = 112,
    LocateTopicReq = 113,
    LocateTopicRep = 114,
    ResolveIpReq = 121,
    ResolveIpRep = 122,
}
impl MsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MsgType::Unknown => "UNKNOWN",
            MsgType::PingReq => "PING_REQ",
            MsgType::PingRep => "PING_REP",
            MsgType::PushReq => "PUSH_REQ",
            MsgType::PushRep => "PUSH_REP",
            MsgType::PullReq => "PULL_REQ",
            MsgType::PullRep => "PULL_REP",
            MsgType::ReqReq => "REQ_REQ",
            MsgType::ReqRep => "REQ_REP",
            MsgType::AuthReq => "AUTH_REQ",
            MsgType::AuthRep => "AUTH_REP",
            MsgType::OkRep => "OK_REP",
            MsgType::ErrorRep => "ERROR_REP",
            MsgType::Ok2Rep => "OK2_REP",
            MsgType::Error2Rep => "ERROR2_REP",
            MsgType::RegisterFrontendReq => "REGISTER_FRONTEND_REQ",
            MsgType::RegisterFrontendRep => "REGISTER_FRONTEND_REP",
            MsgType::RegisterBackendReq => "REGISTER_BACKEND_REQ",
            MsgType::RegisterBackendRep => "REGISTER_BACKEND_REP",
            MsgType::RegisterServerReq => "REGISTER_SERVER_REQ",
            MsgType::RegisterServerRep => "REGISTER_SERVER_REP",
            MsgType::AddRoutesReq => "ADD_ROUTES_REQ",
            MsgType::AddRoutesRep => "ADD_ROUTES_REP",
            MsgType::GetRoutesReq => "GET_ROUTES_REQ",
            MsgType::GetRoutesRep => "GET_ROUTES_REP",
            MsgType::RouteAddedMsg => "ROUTE_ADDED_MSG",
            MsgType::RouteDeletedMsg => "ROUTE_DELETED_MSG",
            MsgType::RouteStatusChangedMsg => "ROUTE_STATUS_CHANGED_MSG",
            MsgType::AssignFrontendReq => "ASSIGN_FRONTEND_REQ",
            MsgType::AssignFrontendRep => "ASSIGN_FRONTEND_REP",
            MsgType::LocateTopicReq => "LOCATE_TOPIC_REQ",
            MsgType::LocateTopicRep => "LOCATE_TOPIC_REP",
            MsgType::ResolveIpReq => "RESOLVE_IP_REQ",
            MsgType::ResolveIpRep => "RESOLVE_IP_REP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "PING_REQ" => Some(Self::PingReq),
            "PING_REP" => Some(Self::PingRep),
            "PUSH_REQ" => Some(Self::PushReq),
            "PUSH_REP" => Some(Self::PushRep),
            "PULL_REQ" => Some(Self::PullReq),
            "PULL_REP" => Some(Self::PullRep),
            "REQ_REQ" => Some(Self::ReqReq),
            "REQ_REP" => Some(Self::ReqRep),
            "AUTH_REQ" => Some(Self::AuthReq),
            "AUTH_REP" => Some(Self::AuthRep),
            "OK_REP" => Some(Self::OkRep),
            "ERROR_REP" => Some(Self::ErrorRep),
            "OK2_REP" => Some(Self::Ok2Rep),
            "ERROR2_REP" => Some(Self::Error2Rep),
            "REGISTER_FRONTEND_REQ" => Some(Self::RegisterFrontendReq),
            "REGISTER_FRONTEND_REP" => Some(Self::RegisterFrontendRep),
            "REGISTER_BACKEND_REQ" => Some(Self::RegisterBackendReq),
            "REGISTER_BACKEND_REP" => Some(Self::RegisterBackendRep),
            "REGISTER_SERVER_REQ" => Some(Self::RegisterServerReq),
            "REGISTER_SERVER_REP" => Some(Self::RegisterServerRep),
            "ADD_ROUTES_REQ" => Some(Self::AddRoutesReq),
            "ADD_ROUTES_REP" => Some(Self::AddRoutesRep),
            "GET_ROUTES_REQ" => Some(Self::GetRoutesReq),
            "GET_ROUTES_REP" => Some(Self::GetRoutesRep),
            "ROUTE_ADDED_MSG" => Some(Self::RouteAddedMsg),
            "ROUTE_DELETED_MSG" => Some(Self::RouteDeletedMsg),
            "ROUTE_STATUS_CHANGED_MSG" => Some(Self::RouteStatusChangedMsg),
            "ASSIGN_FRONTEND_REQ" => Some(Self::AssignFrontendReq),
            "ASSIGN_FRONTEND_REP" => Some(Self::AssignFrontendRep),
            "LOCATE_TOPIC_REQ" => Some(Self::LocateTopicReq),
            "LOCATE_TOPIC_REP" => Some(Self::LocateTopicRep),
            "RESOLVE_IP_REQ" => Some(Self::ResolveIpReq),
            "RESOLVE_IP_REP" => Some(Self::ResolveIpRep),
            _ => None,
        }
    }
}
