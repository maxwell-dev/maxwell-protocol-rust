use super::maxwell_protocol::*;
use actix::Message as ActixMessage;
use bytes::{BufMut, Bytes, BytesMut};
pub use prost::DecodeError;
use prost::Message as ProstMessage;
use std::{
  fmt::{Debug, Formatter, Result as FmtResult},
  result::Result as StdResult,
};

pub enum ProtocolMsg {
  None,
  PingReq(PingReq),
  PingRep(PingRep),
  PullReq(PullReq),
  PullRep(PullRep),
  PushReq(PushReq),
  PushRep(PushRep),
  DoReq(DoReq),
  DoRep(DoRep),
  Do2Req(Do2Req),
  Do2Rep(Do2Rep),
  AuthReq(AuthReq),
  AuthRep(AuthRep),
  OkRep(OkRep),
  ErrorRep(ErrorRep),
  Ok2Rep(Ok2Rep),
  Error2Rep(Error2Rep),
  RegisterFrontendReq(RegisterFrontendReq),
  RegisterFrontendRep(RegisterFrontendRep),
  AddRouteReq(AddRouteReq),
  AddRouteRep(AddRouteRep),
  DeleteRouteReq(DeleteRouteReq),
  DeleteRouteRep(DeleteRouteRep),
  AddRouteMsg(AddRouteMsg),
  DeleteRouteMsg(DeleteRouteMsg),
  PushRoutesReq(PushRoutesReq),
  PushRoutesRep(PushRoutesRep),
  PullRoutesReq(PullRoutesReq),
  PullRoutesRep(PullRoutesRep),
  RegisterBackendReq(RegisterBackendReq),
  RegisterBackendRep(RegisterBackendRep),
  DeleteTopicsReq(DeleteTopicsReq),
  DeleteTopicsRep(DeleteTopicsRep),
  ResolveFrontendReq(ResolveFrontendReq),
  ResolveFrontendRep(ResolveFrontendRep),
  ResolveBackendReq(ResolveBackendReq),
  ResolveBackendRep(ResolveBackendRep),
  WatchReq(WatchReq),
  WatchRep(WatchRep),
  UnwatchReq(UnwatchReq),
  UnwatchRep(UnwatchRep),
  ResolveIpReq(ResolveIpReq),
  ResolveIpRep(ResolveIpRep),
}

impl Debug for ProtocolMsg {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match self {
      ProtocolMsg::None => write!(f, "None"),
      ProtocolMsg::PingReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PingRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PullReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PullRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PushReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PushRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::DoReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::DoRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::Do2Req(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::Do2Rep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::AuthReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::AuthRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::OkRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ErrorRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::Ok2Rep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::Error2Rep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterFrontendReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterFrontendRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::AddRouteReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::AddRouteRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::DeleteRouteReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::DeleteRouteRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::AddRouteMsg(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::DeleteRouteMsg(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PushRoutesReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PushRoutesRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PullRoutesReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PullRoutesRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterBackendReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterBackendRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::DeleteTopicsReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::DeleteTopicsRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ResolveFrontendReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ResolveFrontendRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ResolveBackendReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ResolveBackendRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::WatchReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::WatchRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::UnwatchReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::UnwatchRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ResolveIpReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ResolveIpRep(msg) => write!(f, "{:?}", msg),
    }
  }
}

pub trait IntoEnum {
  fn into_enum(self) -> ProtocolMsg;
}

impl IntoEnum for PingReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PingReq(self)
  }
}

impl IntoEnum for PingRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PingRep(self)
  }
}

impl IntoEnum for PullReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PullReq(self)
  }
}

impl IntoEnum for PullRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PullRep(self)
  }
}

impl IntoEnum for PushReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PushReq(self)
  }
}

impl IntoEnum for PushRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PushRep(self)
  }
}

impl IntoEnum for DoReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::DoReq(self)
  }
}

impl IntoEnum for DoRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::DoRep(self)
  }
}

impl IntoEnum for Do2Req {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::Do2Req(self)
  }
}

impl IntoEnum for Do2Rep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::Do2Rep(self)
  }
}

impl IntoEnum for AuthReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::AuthReq(self)
  }
}

impl IntoEnum for AuthRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::AuthRep(self)
  }
}

impl IntoEnum for OkRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::OkRep(self)
  }
}

impl IntoEnum for ErrorRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::ErrorRep(self)
  }
}

impl IntoEnum for Ok2Rep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::Ok2Rep(self)
  }
}

impl IntoEnum for Error2Rep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::Error2Rep(self)
  }
}

impl IntoEnum for RegisterFrontendReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::RegisterFrontendReq(self)
  }
}

impl IntoEnum for RegisterFrontendRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::RegisterFrontendRep(self)
  }
}

impl IntoEnum for AddRouteReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::AddRouteReq(self)
  }
}

impl IntoEnum for AddRouteRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::AddRouteRep(self)
  }
}

impl IntoEnum for DeleteRouteReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::DeleteRouteReq(self)
  }
}

impl IntoEnum for DeleteRouteRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::DeleteRouteRep(self)
  }
}

impl IntoEnum for AddRouteMsg {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::AddRouteMsg(self)
  }
}

impl IntoEnum for DeleteRouteMsg {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::DeleteRouteMsg(self)
  }
}

impl IntoEnum for PushRoutesReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PushRoutesReq(self)
  }
}

impl IntoEnum for PushRoutesRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PushRoutesRep(self)
  }
}

impl IntoEnum for PullRoutesReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PullRoutesReq(self)
  }
}

impl IntoEnum for PullRoutesRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PullRoutesRep(self)
  }
}

impl IntoEnum for RegisterBackendReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::RegisterBackendReq(self)
  }
}

impl IntoEnum for RegisterBackendRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::RegisterBackendRep(self)
  }
}

impl IntoEnum for DeleteTopicsReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::DeleteTopicsReq(self)
  }
}

impl IntoEnum for DeleteTopicsRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::DeleteTopicsRep(self)
  }
}

impl IntoEnum for ResolveFrontendReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::ResolveFrontendReq(self)
  }
}

impl IntoEnum for ResolveFrontendRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::ResolveFrontendRep(self)
  }
}

impl IntoEnum for ResolveBackendReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::ResolveBackendReq(self)
  }
}

impl IntoEnum for ResolveBackendRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::ResolveBackendRep(self)
  }
}

impl IntoEnum for WatchReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::WatchReq(self)
  }
}

impl IntoEnum for WatchRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::WatchRep(self)
  }
}

impl IntoEnum for UnwatchReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::UnwatchReq(self)
  }
}

impl IntoEnum for UnwatchRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::UnwatchRep(self)
  }
}

impl IntoEnum for ResolveIpReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::ResolveIpReq(self)
  }
}

impl IntoEnum for ResolveIpRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::ResolveIpRep(self)
  }
}

#[derive(Debug)]
pub enum SendError {
  Timeout,
  Closed,
  Any(Box<dyn std::error::Error + Send + Sync>),
}

impl ActixMessage for ProtocolMsg {
  type Result = StdResult<ProtocolMsg, SendError>;
}

pub trait Encode: ProstMessage + Sized {
  fn encode_type(bytes: &mut BytesMut);

  #[inline]
  fn encode_body(&self, bytes: &mut BytesMut) {
    if let Err(err) = self.encode(bytes) {
      panic!("Failed to encode msg: {:?}", err);
    }
  }

  fn encode_msg(&self) -> Bytes {
    let size = self.encoded_len() as usize;
    let mut bytes = BytesMut::with_capacity(size + 1);
    Self::encode_type(&mut bytes);
    self.encode_body(&mut bytes);
    return bytes.freeze();
  }
}

impl Encode for PingReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(1);
  }
}

impl Encode for PingRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(2);
  }
}

impl Encode for PullReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(3);
  }
}

impl Encode for PullRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(4);
  }
}

impl Encode for PushReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(5);
  }
}

impl Encode for PushRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(6);
  }
}

impl Encode for DoReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(7);
  }
}

impl Encode for DoRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(8);
  }
}

impl Encode for Do2Req {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(9);
  }
}

impl Encode for Do2Rep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(10);
  }
}

impl Encode for AuthReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(27);
  }
}

impl Encode for AuthRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(28);
  }
}

impl Encode for OkRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(29);
  }
}

impl Encode for ErrorRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(30);
  }
}

impl Encode for Ok2Rep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(31);
  }
}

impl Encode for Error2Rep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(32);
  }
}

impl Encode for RegisterFrontendReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(65);
  }
}

impl Encode for RegisterFrontendRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(66);
  }
}

impl Encode for AddRouteReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(67);
  }
}

impl Encode for AddRouteRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(68);
  }
}

impl Encode for DeleteRouteReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(69);
  }
}

impl Encode for DeleteRouteRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(70);
  }
}

impl Encode for AddRouteMsg {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(71);
  }
}

impl Encode for DeleteRouteMsg {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(72);
  }
}

impl Encode for PushRoutesReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(73);
  }
}

impl Encode for PushRoutesRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(74);
  }
}

impl Encode for PullRoutesReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(75);
  }
}

impl Encode for PullRoutesRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(76);
  }
}

impl Encode for RegisterBackendReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(81);
  }
}

impl Encode for RegisterBackendRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(82);
  }
}

impl Encode for DeleteTopicsReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(83);
  }
}

impl Encode for DeleteTopicsRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(84);
  }
}

impl Encode for ResolveFrontendReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(97);
  }
}

impl Encode for ResolveFrontendRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(98);
  }
}

impl Encode for ResolveBackendReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(99);
  }
}

impl Encode for ResolveBackendRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(100);
  }
}

impl Encode for WatchReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(105);
  }
}

impl Encode for WatchRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(106);
  }
}

impl Encode for UnwatchReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(107);
  }
}

impl Encode for UnwatchRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(108);
  }
}

impl Encode for ResolveIpReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(121);
  }
}

impl Encode for ResolveIpRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(122);
  }
}

pub fn encode(protocol_msg: &ProtocolMsg) -> Bytes {
  match protocol_msg {
    ProtocolMsg::None => panic!("Failed to encode ProtocolMsg::None"),
    ProtocolMsg::PingReq(msg) => msg.encode_msg(),
    ProtocolMsg::PingRep(msg) => msg.encode_msg(),
    ProtocolMsg::PullReq(msg) => msg.encode_msg(),
    ProtocolMsg::PullRep(msg) => msg.encode_msg(),
    ProtocolMsg::PushReq(msg) => msg.encode_msg(),
    ProtocolMsg::PushRep(msg) => msg.encode_msg(),
    ProtocolMsg::DoReq(msg) => msg.encode_msg(),
    ProtocolMsg::DoRep(msg) => msg.encode_msg(),
    ProtocolMsg::Do2Req(msg) => msg.encode_msg(),
    ProtocolMsg::Do2Rep(msg) => msg.encode_msg(),
    ProtocolMsg::AuthReq(msg) => msg.encode_msg(),
    ProtocolMsg::AuthRep(msg) => msg.encode_msg(),
    ProtocolMsg::OkRep(msg) => msg.encode_msg(),
    ProtocolMsg::ErrorRep(msg) => msg.encode_msg(),
    ProtocolMsg::Ok2Rep(msg) => msg.encode_msg(),
    ProtocolMsg::Error2Rep(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterFrontendReq(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterFrontendRep(msg) => msg.encode_msg(),
    ProtocolMsg::AddRouteReq(msg) => msg.encode_msg(),
    ProtocolMsg::AddRouteRep(msg) => msg.encode_msg(),
    ProtocolMsg::DeleteRouteReq(msg) => msg.encode_msg(),
    ProtocolMsg::DeleteRouteRep(msg) => msg.encode_msg(),
    ProtocolMsg::AddRouteMsg(msg) => msg.encode_msg(),
    ProtocolMsg::DeleteRouteMsg(msg) => msg.encode_msg(),
    ProtocolMsg::PushRoutesReq(msg) => msg.encode_msg(),
    ProtocolMsg::PushRoutesRep(msg) => msg.encode_msg(),
    ProtocolMsg::PullRoutesReq(msg) => msg.encode_msg(),
    ProtocolMsg::PullRoutesRep(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterBackendReq(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterBackendRep(msg) => msg.encode_msg(),
    ProtocolMsg::DeleteTopicsReq(msg) => msg.encode_msg(),
    ProtocolMsg::DeleteTopicsRep(msg) => msg.encode_msg(),
    ProtocolMsg::ResolveFrontendReq(msg) => msg.encode_msg(),
    ProtocolMsg::ResolveFrontendRep(msg) => msg.encode_msg(),
    ProtocolMsg::ResolveBackendReq(msg) => msg.encode_msg(),
    ProtocolMsg::ResolveBackendRep(msg) => msg.encode_msg(),
    ProtocolMsg::WatchReq(msg) => msg.encode_msg(),
    ProtocolMsg::WatchRep(msg) => msg.encode_msg(),
    ProtocolMsg::UnwatchReq(msg) => msg.encode_msg(),
    ProtocolMsg::UnwatchRep(msg) => msg.encode_msg(),
    ProtocolMsg::ResolveIpReq(msg) => msg.encode_msg(),
    ProtocolMsg::ResolveIpRep(msg) => msg.encode_msg(),
  }
}

pub fn decode(bytes: &Bytes) -> Result<ProtocolMsg, DecodeError> {
  let msg_type = bytes[0] as i8;
  let msg_body = bytes.slice(1..);
  if msg_type == 1 {
    let res: Result<PingReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PingReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 2 {
    let res: Result<PingRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PingRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 3 {
    let res: Result<PullReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PullReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 4 {
    let res: Result<PullRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PullRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 5 {
    let res: Result<PushReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PushReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 6 {
    let res: Result<PushRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PushRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 7 {
    let res: Result<DoReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::DoReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 8 {
    let res: Result<DoRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::DoRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 9 {
    let res: Result<Do2Req, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::Do2Req(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 10 {
    let res: Result<Do2Rep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::Do2Rep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 27 {
    let res: Result<AuthReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::AuthReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 28 {
    let res: Result<AuthRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::AuthRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 29 {
    let res: Result<OkRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::OkRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 30 {
    let res: Result<ErrorRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::ErrorRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 31 {
    let res: Result<Ok2Rep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::Ok2Rep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 32 {
    let res: Result<Error2Rep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::Error2Rep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 65 {
    let res: Result<RegisterFrontendReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RegisterFrontendReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 66 {
    let res: Result<RegisterFrontendRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RegisterFrontendRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 67 {
    let res: Result<AddRouteReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::AddRouteReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 68 {
    let res: Result<AddRouteRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::AddRouteRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 69 {
    let res: Result<DeleteRouteReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::DeleteRouteReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 70 {
    let res: Result<DeleteRouteRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::DeleteRouteRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 71 {
    let res: Result<AddRouteMsg, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::AddRouteMsg(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 72 {
    let res: Result<DeleteRouteMsg, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::DeleteRouteMsg(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 73 {
    let res: Result<PushRoutesReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PushRoutesReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 74 {
    let res: Result<PushRoutesRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PushRoutesRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 75 {
    let res: Result<PullRoutesReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PullRoutesReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 76 {
    let res: Result<PullRoutesRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PullRoutesRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 81 {
    let res: Result<RegisterBackendReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RegisterBackendReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 82 {
    let res: Result<RegisterBackendRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RegisterBackendRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 83 {
    let res: Result<DeleteTopicsReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::DeleteTopicsReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 84 {
    let res: Result<DeleteTopicsRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::DeleteTopicsRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 97 {
    let res: Result<ResolveFrontendReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::ResolveFrontendReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 98 {
    let res: Result<ResolveFrontendRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::ResolveFrontendRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 99 {
    let res: Result<ResolveBackendReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::ResolveBackendReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 100 {
    let res: Result<ResolveBackendRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::ResolveBackendRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 105 {
    let res: Result<WatchReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::WatchReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 106 {
    let res: Result<WatchRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::WatchRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 107 {
    let res: Result<UnwatchReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::UnwatchReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 108 {
    let res: Result<UnwatchRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::UnwatchRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 121 {
    let res: Result<ResolveIpReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::ResolveIpReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 122 {
    let res: Result<ResolveIpRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::ResolveIpRep(msg)),
      Err(err) => Err(err),
    }
  } else {
    Err(DecodeError::new(format!("Invalid msg type: {}", msg_type)))
  }
}

pub fn set_round_ref(protocol_msg: &mut ProtocolMsg, round_ref: u32) -> &ProtocolMsg {
  match protocol_msg {
    ProtocolMsg::None => panic!("Failed to set round ref for ProtocolMsg::None"),
    ProtocolMsg::PingReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::PingRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::PullReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::PullRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::PushReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::PushRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::DoReq(msg) => msg.traces[0].r#ref = round_ref,
    ProtocolMsg::DoRep(msg) => msg.traces[0].r#ref = round_ref,
    ProtocolMsg::Do2Req(msg) => msg.traces[0].r#ref = round_ref,
    ProtocolMsg::Do2Rep(msg) => msg.traces[0].r#ref = round_ref,
    ProtocolMsg::AuthReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::AuthRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::OkRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::ErrorRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::Ok2Rep(msg) => msg.traces[0].r#ref = round_ref,
    ProtocolMsg::Error2Rep(msg) => msg.traces[0].r#ref = round_ref,
    ProtocolMsg::RegisterFrontendReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::RegisterFrontendRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::AddRouteReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::AddRouteRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::DeleteRouteReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::DeleteRouteRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::AddRouteMsg(msg) => msg.r#ref = round_ref,
    ProtocolMsg::DeleteRouteMsg(msg) => msg.r#ref = round_ref,
    ProtocolMsg::PushRoutesReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::PushRoutesRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::PullRoutesReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::PullRoutesRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::RegisterBackendReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::RegisterBackendRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::DeleteTopicsReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::DeleteTopicsRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::ResolveFrontendReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::ResolveFrontendRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::ResolveBackendReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::ResolveBackendRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::WatchReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::WatchRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::UnwatchReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::UnwatchRep(msg) => msg.r#ref = round_ref,
    ProtocolMsg::ResolveIpReq(msg) => msg.r#ref = round_ref,
    ProtocolMsg::ResolveIpRep(msg) => msg.r#ref = round_ref,
  }
  protocol_msg
}

pub fn get_round_ref(protocol_msg: &ProtocolMsg) -> u32 {
  match protocol_msg {
    ProtocolMsg::None => panic!("Failed to get round ref from ProtocolMsg::None"),
    ProtocolMsg::PingReq(msg) => msg.r#ref,
    ProtocolMsg::PingRep(msg) => msg.r#ref,
    ProtocolMsg::PullReq(msg) => msg.r#ref,
    ProtocolMsg::PullRep(msg) => msg.r#ref,
    ProtocolMsg::PushReq(msg) => msg.r#ref,
    ProtocolMsg::PushRep(msg) => msg.r#ref,
    ProtocolMsg::DoReq(msg) => msg.traces[0].r#ref,
    ProtocolMsg::DoRep(msg) => msg.traces[0].r#ref,
    ProtocolMsg::Do2Req(msg) => msg.traces[0].r#ref,
    ProtocolMsg::Do2Rep(msg) => msg.traces[0].r#ref,
    ProtocolMsg::AuthReq(msg) => msg.r#ref,
    ProtocolMsg::AuthRep(msg) => msg.r#ref,
    ProtocolMsg::OkRep(msg) => msg.r#ref,
    ProtocolMsg::ErrorRep(msg) => msg.r#ref,
    ProtocolMsg::Ok2Rep(msg) => msg.traces[0].r#ref,
    ProtocolMsg::Error2Rep(msg) => msg.traces[0].r#ref,
    ProtocolMsg::RegisterFrontendReq(msg) => msg.r#ref,
    ProtocolMsg::RegisterFrontendRep(msg) => msg.r#ref,
    ProtocolMsg::AddRouteReq(msg) => msg.r#ref,
    ProtocolMsg::AddRouteRep(msg) => msg.r#ref,
    ProtocolMsg::DeleteRouteReq(msg) => msg.r#ref,
    ProtocolMsg::DeleteRouteRep(msg) => msg.r#ref,
    ProtocolMsg::AddRouteMsg(msg) => msg.r#ref,
    ProtocolMsg::DeleteRouteMsg(msg) => msg.r#ref,
    ProtocolMsg::PushRoutesReq(msg) => msg.r#ref,
    ProtocolMsg::PushRoutesRep(msg) => msg.r#ref,
    ProtocolMsg::PullRoutesReq(msg) => msg.r#ref,
    ProtocolMsg::PullRoutesRep(msg) => msg.r#ref,
    ProtocolMsg::RegisterBackendReq(msg) => msg.r#ref,
    ProtocolMsg::RegisterBackendRep(msg) => msg.r#ref,
    ProtocolMsg::DeleteTopicsReq(msg) => msg.r#ref,
    ProtocolMsg::DeleteTopicsRep(msg) => msg.r#ref,
    ProtocolMsg::ResolveFrontendReq(msg) => msg.r#ref,
    ProtocolMsg::ResolveFrontendRep(msg) => msg.r#ref,
    ProtocolMsg::ResolveBackendReq(msg) => msg.r#ref,
    ProtocolMsg::ResolveBackendRep(msg) => msg.r#ref,
    ProtocolMsg::WatchReq(msg) => msg.r#ref,
    ProtocolMsg::WatchRep(msg) => msg.r#ref,
    ProtocolMsg::UnwatchReq(msg) => msg.r#ref,
    ProtocolMsg::UnwatchRep(msg) => msg.r#ref,
    ProtocolMsg::ResolveIpReq(msg) => msg.r#ref,
    ProtocolMsg::ResolveIpRep(msg) => msg.r#ref,
  }
}
