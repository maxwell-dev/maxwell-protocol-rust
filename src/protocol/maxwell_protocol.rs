// msg type defs

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
// client-frontend protocols

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
// frontend-master protocols

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
// backend-master protocols

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
// client-master protocols

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
// misc protocols

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
// data type defs

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
// msg type enum

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
  // client-backend protocols: 113~120
  /// misc protocols: 121~127
  ResolveIpReq = 121,
  ResolveIpRep = 122,
}
