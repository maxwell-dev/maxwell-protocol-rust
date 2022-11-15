/// core protocols
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingReq {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullReq {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    /// int64, not uint64
    #[prost(int64, tag = "2")]
    pub offset: i64,
    #[prost(uint32, tag = "3")]
    pub limit: u32,
    #[prost(uint32, tag = "4")]
    pub puller: u32,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullRep {
    #[prost(message, repeated, tag = "1")]
    pub msgs: ::prost::alloc::vec::Vec<Msg>,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushReq {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoReq {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    #[prost(bool, tag = "13")]
    pub source_enabled: bool,
    #[prost(message, optional, tag = "14")]
    pub source: ::core::option::Option<Source>,
    #[prost(message, repeated, tag = "15")]
    pub traces: ::prost::alloc::vec::Vec<Trace>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoRep {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "15")]
    pub traces: ::prost::alloc::vec::Vec<Trace>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Do2Req {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bool, tag = "13")]
    pub source_enabled: bool,
    #[prost(message, optional, tag = "14")]
    pub source: ::core::option::Option<Source>,
    #[prost(message, repeated, tag = "15")]
    pub traces: ::prost::alloc::vec::Vec<Trace>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Do2Rep {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "15")]
    pub traces: ::prost::alloc::vec::Vec<Trace>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthReq {
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OkRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorRep {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ok2Rep {
    #[prost(message, repeated, tag = "15")]
    pub traces: ::prost::alloc::vec::Vec<Trace>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error2Rep {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "15")]
    pub traces: ::prost::alloc::vec::Vec<Trace>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchReq {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnwatchReq {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnwatchRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterFrontendReq {
    /// public endpoint
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterFrontendRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRouteReq {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRouteRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRouteReq {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRouteRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRouteMsg {
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRouteMsg {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushRoutesReq {
    #[prost(string, repeated, tag = "1")]
    pub types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushRoutesRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullRoutesReq {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullRoutesRep {
    #[prost(message, repeated, tag = "1")]
    pub route_groups: ::prost::alloc::vec::Vec<RouteGroup>,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTopicsReq {
    #[prost(string, repeated, tag = "1")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTopicsRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterBackendReq {
    /// public endpoint
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterBackendRep {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveFrontendReq {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveFrontendRep {
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveBackendReq {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveBackendRep {
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveIpReq {
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveIpRep {
    #[prost(string, tag = "1")]
    pub ip: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub r#ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    /// uint64, not int64
    #[prost(uint64, tag = "1")]
    pub offset: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub timestamp: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Source {
    #[prost(string, tag = "1")]
    pub agent: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trace {
    #[prost(uint32, tag = "1")]
    pub r#ref: u32,
    #[prost(uint32, tag = "2")]
    pub handler_id: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub node_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteGroup {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub endpoints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgType {
    /// unused, placeholder for gpb: 0
    Unknown = 0,
    /// core protocols: 1~32
    PingReq = 1,
    PingRep = 2,
    PullReq = 3,
    PullRep = 4,
    PushReq = 5,
    PushRep = 6,
    DoReq = 7,
    DoRep = 8,
    Do2Req = 9,
    Do2Rep = 10,
    AuthReq = 27,
    AuthRep = 28,
    OkRep = 29,
    ErrorRep = 30,
    Ok2Rep = 31,
    Error2Rep = 32,
    /// frontend-master protocols: 65~80
    RegisterFrontendReq = 65,
    RegisterFrontendRep = 66,
    AddRouteReq = 67,
    AddRouteRep = 68,
    DeleteRouteReq = 69,
    DeleteRouteRep = 70,
    AddRouteMsg = 71,
    DeleteRouteMsg = 72,
    PushRoutesReq = 73,
    PushRoutesRep = 74,
    PullRoutesReq = 75,
    PullRoutesRep = 76,
    /// backend-master protocols: 81~96
    RegisterBackendReq = 81,
    RegisterBackendRep = 82,
    DeleteTopicsReq = 83,
    DeleteTopicsRep = 84,
    /// client-master protocols: 97-104
    ResolveFrontendReq = 97,
    ResolveFrontendRep = 98,
    ResolveBackendReq = 99,
    ResolveBackendRep = 100,
    /// client-frontend protocols: 105~112
    WatchReq = 105,
    WatchRep = 106,
    UnwatchReq = 107,
    UnwatchRep = 108,
    /// misc protocols: 121~127
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
            MsgType::PullReq => "PULL_REQ",
            MsgType::PullRep => "PULL_REP",
            MsgType::PushReq => "PUSH_REQ",
            MsgType::PushRep => "PUSH_REP",
            MsgType::DoReq => "DO_REQ",
            MsgType::DoRep => "DO_REP",
            MsgType::Do2Req => "DO2_REQ",
            MsgType::Do2Rep => "DO2_REP",
            MsgType::AuthReq => "AUTH_REQ",
            MsgType::AuthRep => "AUTH_REP",
            MsgType::OkRep => "OK_REP",
            MsgType::ErrorRep => "ERROR_REP",
            MsgType::Ok2Rep => "OK2_REP",
            MsgType::Error2Rep => "ERROR2_REP",
            MsgType::RegisterFrontendReq => "REGISTER_FRONTEND_REQ",
            MsgType::RegisterFrontendRep => "REGISTER_FRONTEND_REP",
            MsgType::AddRouteReq => "ADD_ROUTE_REQ",
            MsgType::AddRouteRep => "ADD_ROUTE_REP",
            MsgType::DeleteRouteReq => "DELETE_ROUTE_REQ",
            MsgType::DeleteRouteRep => "DELETE_ROUTE_REP",
            MsgType::AddRouteMsg => "ADD_ROUTE_MSG",
            MsgType::DeleteRouteMsg => "DELETE_ROUTE_MSG",
            MsgType::PushRoutesReq => "PUSH_ROUTES_REQ",
            MsgType::PushRoutesRep => "PUSH_ROUTES_REP",
            MsgType::PullRoutesReq => "PULL_ROUTES_REQ",
            MsgType::PullRoutesRep => "PULL_ROUTES_REP",
            MsgType::RegisterBackendReq => "REGISTER_BACKEND_REQ",
            MsgType::RegisterBackendRep => "REGISTER_BACKEND_REP",
            MsgType::DeleteTopicsReq => "DELETE_TOPICS_REQ",
            MsgType::DeleteTopicsRep => "DELETE_TOPICS_REP",
            MsgType::ResolveFrontendReq => "RESOLVE_FRONTEND_REQ",
            MsgType::ResolveFrontendRep => "RESOLVE_FRONTEND_REP",
            MsgType::ResolveBackendReq => "RESOLVE_BACKEND_REQ",
            MsgType::ResolveBackendRep => "RESOLVE_BACKEND_REP",
            MsgType::WatchReq => "WATCH_REQ",
            MsgType::WatchRep => "WATCH_REP",
            MsgType::UnwatchReq => "UNWATCH_REQ",
            MsgType::UnwatchRep => "UNWATCH_REP",
            MsgType::ResolveIpReq => "RESOLVE_IP_REQ",
            MsgType::ResolveIpRep => "RESOLVE_IP_REP",
        }
    }
}
