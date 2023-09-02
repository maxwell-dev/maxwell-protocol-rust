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
pub struct RegisterFrontendReq {
    #[prost(uint32, tag = "1")]
    pub http_port: u32,
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
pub struct RegisterServiceReq {
    #[prost(uint32, tag = "1")]
    pub http_port: u32,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterServiceRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoutesReq {
    #[prost(string, repeated, tag = "1")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoutesRep {
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
    pub route_groups: ::prost::alloc::vec::Vec<RouteGroup>,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicDistChecksumReq {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicDistChecksumRep {
    #[prost(uint32, tag = "1")]
    pub checksum: u32,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRouteDistChecksumReq {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRouteDistChecksumRep {
    #[prost(uint32, tag = "1")]
    pub checksum: u32,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickFrontendReq {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickFrontendRep {
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickFrontendsReq {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickFrontendsRep {
    #[prost(string, repeated, tag = "1")]
    pub endpoints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
    #[prost(string, tag = "3")]
    pub token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteGroup {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub healthy_endpoints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub unhealthy_endpoints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// unused, placeholder for some impls
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgType {
    Unknown = 0,
    PingReq = 1,
    PingRep = 2,
    OkRep = 29,
    ErrorRep = 30,
    Ok2Rep = 31,
    Error2Rep = 32,
    PushReq = 33,
    PushRep = 34,
    PullReq = 35,
    PullRep = 36,
    ReqReq = 39,
    ReqRep = 40,
    AuthReq = 41,
    AuthRep = 42,
    RegisterFrontendReq = 65,
    RegisterFrontendRep = 66,
    RegisterBackendReq = 67,
    RegisterBackendRep = 68,
    RegisterServiceReq = 69,
    RegisterServiceRep = 70,
    SetRoutesReq = 71,
    SetRoutesRep = 72,
    GetRoutesReq = 75,
    GetRoutesRep = 76,
    GetTopicDistChecksumReq = 77,
    GetTopicDistChecksumRep = 78,
    GetRouteDistChecksumReq = 79,
    GetRouteDistChecksumRep = 80,
    PickFrontendReq = 81,
    PickFrontendRep = 82,
    PickFrontendsReq = 83,
    PickFrontendsRep = 84,
    LocateTopicReq = 85,
    LocateTopicRep = 86,
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
            MsgType::OkRep => "OK_REP",
            MsgType::ErrorRep => "ERROR_REP",
            MsgType::Ok2Rep => "OK2_REP",
            MsgType::Error2Rep => "ERROR2_REP",
            MsgType::PushReq => "PUSH_REQ",
            MsgType::PushRep => "PUSH_REP",
            MsgType::PullReq => "PULL_REQ",
            MsgType::PullRep => "PULL_REP",
            MsgType::ReqReq => "REQ_REQ",
            MsgType::ReqRep => "REQ_REP",
            MsgType::AuthReq => "AUTH_REQ",
            MsgType::AuthRep => "AUTH_REP",
            MsgType::RegisterFrontendReq => "REGISTER_FRONTEND_REQ",
            MsgType::RegisterFrontendRep => "REGISTER_FRONTEND_REP",
            MsgType::RegisterBackendReq => "REGISTER_BACKEND_REQ",
            MsgType::RegisterBackendRep => "REGISTER_BACKEND_REP",
            MsgType::RegisterServiceReq => "REGISTER_SERVICE_REQ",
            MsgType::RegisterServiceRep => "REGISTER_SERVICE_REP",
            MsgType::SetRoutesReq => "SET_ROUTES_REQ",
            MsgType::SetRoutesRep => "SET_ROUTES_REP",
            MsgType::GetRoutesReq => "GET_ROUTES_REQ",
            MsgType::GetRoutesRep => "GET_ROUTES_REP",
            MsgType::GetTopicDistChecksumReq => "GET_TOPIC_DIST_CHECKSUM_REQ",
            MsgType::GetTopicDistChecksumRep => "GET_TOPIC_DIST_CHECKSUM_REP",
            MsgType::GetRouteDistChecksumReq => "GET_ROUTE_DIST_CHECKSUM_REQ",
            MsgType::GetRouteDistChecksumRep => "GET_ROUTE_DIST_CHECKSUM_REP",
            MsgType::PickFrontendReq => "PICK_FRONTEND_REQ",
            MsgType::PickFrontendRep => "PICK_FRONTEND_REP",
            MsgType::PickFrontendsReq => "PICK_FRONTENDS_REQ",
            MsgType::PickFrontendsRep => "PICK_FRONTENDS_REP",
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
            "OK_REP" => Some(Self::OkRep),
            "ERROR_REP" => Some(Self::ErrorRep),
            "OK2_REP" => Some(Self::Ok2Rep),
            "ERROR2_REP" => Some(Self::Error2Rep),
            "PUSH_REQ" => Some(Self::PushReq),
            "PUSH_REP" => Some(Self::PushRep),
            "PULL_REQ" => Some(Self::PullReq),
            "PULL_REP" => Some(Self::PullRep),
            "REQ_REQ" => Some(Self::ReqReq),
            "REQ_REP" => Some(Self::ReqRep),
            "AUTH_REQ" => Some(Self::AuthReq),
            "AUTH_REP" => Some(Self::AuthRep),
            "REGISTER_FRONTEND_REQ" => Some(Self::RegisterFrontendReq),
            "REGISTER_FRONTEND_REP" => Some(Self::RegisterFrontendRep),
            "REGISTER_BACKEND_REQ" => Some(Self::RegisterBackendReq),
            "REGISTER_BACKEND_REP" => Some(Self::RegisterBackendRep),
            "REGISTER_SERVICE_REQ" => Some(Self::RegisterServiceReq),
            "REGISTER_SERVICE_REP" => Some(Self::RegisterServiceRep),
            "SET_ROUTES_REQ" => Some(Self::SetRoutesReq),
            "SET_ROUTES_REP" => Some(Self::SetRoutesRep),
            "GET_ROUTES_REQ" => Some(Self::GetRoutesReq),
            "GET_ROUTES_REP" => Some(Self::GetRoutesRep),
            "GET_TOPIC_DIST_CHECKSUM_REQ" => Some(Self::GetTopicDistChecksumReq),
            "GET_TOPIC_DIST_CHECKSUM_REP" => Some(Self::GetTopicDistChecksumRep),
            "GET_ROUTE_DIST_CHECKSUM_REQ" => Some(Self::GetRouteDistChecksumReq),
            "GET_ROUTE_DIST_CHECKSUM_REP" => Some(Self::GetRouteDistChecksumRep),
            "PICK_FRONTEND_REQ" => Some(Self::PickFrontendReq),
            "PICK_FRONTEND_REP" => Some(Self::PickFrontendRep),
            "PICK_FRONTENDS_REQ" => Some(Self::PickFrontendsReq),
            "PICK_FRONTENDS_REP" => Some(Self::PickFrontendsRep),
            "LOCATE_TOPIC_REQ" => Some(Self::LocateTopicReq),
            "LOCATE_TOPIC_REP" => Some(Self::LocateTopicRep),
            "RESOLVE_IP_REQ" => Some(Self::ResolveIpReq),
            "RESOLVE_IP_REP" => Some(Self::ResolveIpRep),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorCode {
    /// common error codes: 0~99
    Ok = 0,
    UnknownMsg = 1,
    /// master error codes: 100~199
    NotAllowedToRegisterFrontend = 100,
    NotAllowedToRegisterBackend = 101,
    NotAllowedToRegisterService = 102,
    FailedToPickFrontend = 103,
    FailedToLocateTopic = 104,
    MasterError = 199,
    /// frontend error codes: 200~299
    FailedToRequestService = 200,
    FailedToRequestBackend = 201,
    FrontendError = 299,
    /// backend error codes: 300~399
    FailedToPush = 300,
    FailedToPull = 301,
    UnknownTopic = 302,
    BackendError = 399,
    /// service error codes: 400~499
    UnknownPath = 400,
    ServiceError = 499,
    /// client error codes: 500~599
    ClientError = 599,
}
impl ErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ErrorCode::Ok => "OK",
            ErrorCode::UnknownMsg => "UNKNOWN_MSG",
            ErrorCode::NotAllowedToRegisterFrontend => "NOT_ALLOWED_TO_REGISTER_FRONTEND",
            ErrorCode::NotAllowedToRegisterBackend => "NOT_ALLOWED_TO_REGISTER_BACKEND",
            ErrorCode::NotAllowedToRegisterService => "NOT_ALLOWED_TO_REGISTER_SERVICE",
            ErrorCode::FailedToPickFrontend => "FAILED_TO_PICK_FRONTEND",
            ErrorCode::FailedToLocateTopic => "FAILED_TO_LOCATE_TOPIC",
            ErrorCode::MasterError => "MASTER_ERROR",
            ErrorCode::FailedToRequestService => "FAILED_TO_REQUEST_SERVICE",
            ErrorCode::FailedToRequestBackend => "FAILED_TO_REQUEST_BACKEND",
            ErrorCode::FrontendError => "FRONTEND_ERROR",
            ErrorCode::FailedToPush => "FAILED_TO_PUSH",
            ErrorCode::FailedToPull => "FAILED_TO_PULL",
            ErrorCode::UnknownTopic => "UNKNOWN_TOPIC",
            ErrorCode::BackendError => "BACKEND_ERROR",
            ErrorCode::UnknownPath => "UNKNOWN_PATH",
            ErrorCode::ServiceError => "SERVICE_ERROR",
            ErrorCode::ClientError => "CLIENT_ERROR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OK" => Some(Self::Ok),
            "UNKNOWN_MSG" => Some(Self::UnknownMsg),
            "NOT_ALLOWED_TO_REGISTER_FRONTEND" => {
                Some(Self::NotAllowedToRegisterFrontend)
            }
            "NOT_ALLOWED_TO_REGISTER_BACKEND" => Some(Self::NotAllowedToRegisterBackend),
            "NOT_ALLOWED_TO_REGISTER_SERVICE" => Some(Self::NotAllowedToRegisterService),
            "FAILED_TO_PICK_FRONTEND" => Some(Self::FailedToPickFrontend),
            "FAILED_TO_LOCATE_TOPIC" => Some(Self::FailedToLocateTopic),
            "MASTER_ERROR" => Some(Self::MasterError),
            "FAILED_TO_REQUEST_SERVICE" => Some(Self::FailedToRequestService),
            "FAILED_TO_REQUEST_BACKEND" => Some(Self::FailedToRequestBackend),
            "FRONTEND_ERROR" => Some(Self::FrontendError),
            "FAILED_TO_PUSH" => Some(Self::FailedToPush),
            "FAILED_TO_PULL" => Some(Self::FailedToPull),
            "UNKNOWN_TOPIC" => Some(Self::UnknownTopic),
            "BACKEND_ERROR" => Some(Self::BackendError),
            "UNKNOWN_PATH" => Some(Self::UnknownPath),
            "SERVICE_ERROR" => Some(Self::ServiceError),
            "CLIENT_ERROR" => Some(Self::ClientError),
            _ => None,
        }
    }
}
