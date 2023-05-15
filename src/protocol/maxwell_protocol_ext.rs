use super::maxwell_protocol::*;
use actix::Message as ActixMessage;
use bytes::{BufMut, Bytes, BytesMut};
pub use prost::DecodeError;
use thiserror::Error as ThisError;
use prost::Message as ProstMessage;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

pub enum ProtocolMsg {
  None,
  PingReq(PingReq),
  PingRep(PingRep),
  PushReq(PushReq),
  PushRep(PushRep),
  PullReq(PullReq),
  PullRep(PullRep),
  ReqReq(ReqReq),
  ReqRep(ReqRep),
  AuthReq(AuthReq),
  AuthRep(AuthRep),
  OkRep(OkRep),
  ErrorRep(ErrorRep),
  Ok2Rep(Ok2Rep),
  Error2Rep(Error2Rep),
  RegisterFrontendReq(RegisterFrontendReq),
  RegisterFrontendRep(RegisterFrontendRep),
  RegisterBackendReq(RegisterBackendReq),
  RegisterBackendRep(RegisterBackendRep),
  RegisterServerReq(RegisterServerReq),
  RegisterServerRep(RegisterServerRep),
  AddRoutesReq(AddRoutesReq),
  AddRoutesRep(AddRoutesRep),
  GetRoutesReq(GetRoutesReq),
  GetRoutesRep(GetRoutesRep),
  RouteAddedMsg(RouteAddedMsg),
  RouteDeletedMsg(RouteDeletedMsg),
  RouteHealthChangedMsg(RouteHealthChangedMsg),
  AssignFrontendReq(AssignFrontendReq),
  AssignFrontendRep(AssignFrontendRep),
  LocateTopicReq(LocateTopicReq),
  LocateTopicRep(LocateTopicRep),
  ResolveIpReq(ResolveIpReq),
  ResolveIpRep(ResolveIpRep),
}

impl Debug for ProtocolMsg {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match self {
      ProtocolMsg::None => write!(f, "None"),
      ProtocolMsg::PingReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PingRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PushReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PushRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PullReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PullRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ReqReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ReqRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::AuthReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::AuthRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::OkRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ErrorRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::Ok2Rep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::Error2Rep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterFrontendReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterFrontendRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterBackendReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterBackendRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterServerReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterServerRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::AddRoutesReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::AddRoutesRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetRoutesReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetRoutesRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RouteAddedMsg(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RouteDeletedMsg(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RouteHealthChangedMsg(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::AssignFrontendReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::AssignFrontendRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::LocateTopicReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::LocateTopicRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ResolveIpReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ResolveIpRep(msg) => write!(f, "{:?}", msg),
    }
  }
}

impl ProtocolMsg {
  #[inline]
  pub fn is_none(&self) -> bool {
    match self {
      Self::None => true,
      _ => false,
    }
  }

  #[inline]
  pub fn is_some(&self) -> bool {
    match self {
      Self::None => false,
      _ => true,
    }
  }
}

#[derive(Debug, ThisError)]
pub enum HandleError<M> {
  #[error("The mailbox was full")]
  MailboxFull,
  #[error("The mailbox has closed")]
  MailboxClosed,
  #[error("Message delivery timed out")]
  Timeout,
  #[error("Error occured: code: {code}, desc: {desc}")]
  Any { code: i32, desc: String, msg: M },
}

impl ActixMessage for ProtocolMsg {
  type Result = StdResult<ProtocolMsg, HandleError<ProtocolMsg>>;
}

impl ActixMessage for PingReq {
  type Result = StdResult<PingRep, HandleError<PingReq>>;
}

impl ActixMessage for PushReq {
  type Result = StdResult<PushRep, HandleError<PushReq>>;
}

impl ActixMessage for PullReq {
  type Result = StdResult<PullRep, HandleError<PullReq>>;
}

impl ActixMessage for ReqReq {
  type Result = StdResult<ReqRep, HandleError<ReqReq>>;
}

impl ActixMessage for AuthReq {
  type Result = StdResult<AuthRep, HandleError<AuthReq>>;
}

impl ActixMessage for RegisterFrontendReq {
  type Result = StdResult<RegisterFrontendRep, HandleError<RegisterFrontendReq>>;
}

impl ActixMessage for RegisterBackendReq {
  type Result = StdResult<RegisterBackendRep, HandleError<RegisterBackendReq>>;
}

impl ActixMessage for RegisterServerReq {
  type Result = StdResult<RegisterServerRep, HandleError<RegisterServerReq>>;
}

impl ActixMessage for AddRoutesReq {
  type Result = StdResult<AddRoutesRep, HandleError<AddRoutesReq>>;
}

impl ActixMessage for GetRoutesReq {
  type Result = StdResult<GetRoutesRep, HandleError<GetRoutesReq>>;
}

impl ActixMessage for AssignFrontendReq {
  type Result = StdResult<AssignFrontendRep, HandleError<AssignFrontendReq>>;
}

impl ActixMessage for LocateTopicReq {
  type Result = StdResult<LocateTopicRep, HandleError<LocateTopicReq>>;
}

impl ActixMessage for ResolveIpReq {
  type Result = StdResult<ResolveIpRep, HandleError<ResolveIpReq>>;
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

impl IntoEnum for ReqReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::ReqReq(self)
  }
}

impl IntoEnum for ReqRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::ReqRep(self)
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

impl IntoEnum for RegisterServerReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::RegisterServerReq(self)
  }
}

impl IntoEnum for RegisterServerRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::RegisterServerRep(self)
  }
}

impl IntoEnum for AddRoutesReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::AddRoutesReq(self)
  }
}

impl IntoEnum for AddRoutesRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::AddRoutesRep(self)
  }
}

impl IntoEnum for GetRoutesReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetRoutesReq(self)
  }
}

impl IntoEnum for GetRoutesRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetRoutesRep(self)
  }
}

impl IntoEnum for RouteAddedMsg {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::RouteAddedMsg(self)
  }
}

impl IntoEnum for RouteDeletedMsg {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::RouteDeletedMsg(self)
  }
}

impl IntoEnum for RouteHealthChangedMsg {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::RouteHealthChangedMsg(self)
  }
}

impl IntoEnum for AssignFrontendReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::AssignFrontendReq(self)
  }
}

impl IntoEnum for AssignFrontendRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::AssignFrontendRep(self)
  }
}

impl IntoEnum for LocateTopicReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::LocateTopicReq(self)
  }
}

impl IntoEnum for LocateTopicRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::LocateTopicRep(self)
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

impl Encode for PushReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(3);
  }
}

impl Encode for PushRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(4);
  }
}

impl Encode for PullReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(5);
  }
}

impl Encode for PullRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(6);
  }
}

impl Encode for ReqReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(7);
  }
}

impl Encode for ReqRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(8);
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

impl Encode for RegisterBackendReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(67);
  }
}

impl Encode for RegisterBackendRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(68);
  }
}

impl Encode for RegisterServerReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(69);
  }
}

impl Encode for RegisterServerRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(70);
  }
}

impl Encode for AddRoutesReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(91);
  }
}

impl Encode for AddRoutesRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(92);
  }
}

impl Encode for GetRoutesReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(95);
  }
}

impl Encode for GetRoutesRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(96);
  }
}

impl Encode for RouteAddedMsg {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(100);
  }
}

impl Encode for RouteDeletedMsg {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(101);
  }
}

impl Encode for RouteHealthChangedMsg {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(102);
  }
}

impl Encode for AssignFrontendReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(111);
  }
}

impl Encode for AssignFrontendRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(112);
  }
}

impl Encode for LocateTopicReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(113);
  }
}

impl Encode for LocateTopicRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(114);
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
    ProtocolMsg::PushReq(msg) => msg.encode_msg(),
    ProtocolMsg::PushRep(msg) => msg.encode_msg(),
    ProtocolMsg::PullReq(msg) => msg.encode_msg(),
    ProtocolMsg::PullRep(msg) => msg.encode_msg(),
    ProtocolMsg::ReqReq(msg) => msg.encode_msg(),
    ProtocolMsg::ReqRep(msg) => msg.encode_msg(),
    ProtocolMsg::AuthReq(msg) => msg.encode_msg(),
    ProtocolMsg::AuthRep(msg) => msg.encode_msg(),
    ProtocolMsg::OkRep(msg) => msg.encode_msg(),
    ProtocolMsg::ErrorRep(msg) => msg.encode_msg(),
    ProtocolMsg::Ok2Rep(msg) => msg.encode_msg(),
    ProtocolMsg::Error2Rep(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterFrontendReq(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterFrontendRep(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterBackendReq(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterBackendRep(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterServerReq(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterServerRep(msg) => msg.encode_msg(),
    ProtocolMsg::AddRoutesReq(msg) => msg.encode_msg(),
    ProtocolMsg::AddRoutesRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetRoutesReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetRoutesRep(msg) => msg.encode_msg(),
    ProtocolMsg::RouteAddedMsg(msg) => msg.encode_msg(),
    ProtocolMsg::RouteDeletedMsg(msg) => msg.encode_msg(),
    ProtocolMsg::RouteHealthChangedMsg(msg) => msg.encode_msg(),
    ProtocolMsg::AssignFrontendReq(msg) => msg.encode_msg(),
    ProtocolMsg::AssignFrontendRep(msg) => msg.encode_msg(),
    ProtocolMsg::LocateTopicReq(msg) => msg.encode_msg(),
    ProtocolMsg::LocateTopicRep(msg) => msg.encode_msg(),
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
    let res: Result<PushReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PushReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 4 {
    let res: Result<PushRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PushRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 5 {
    let res: Result<PullReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PullReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 6 {
    let res: Result<PullRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PullRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 7 {
    let res: Result<ReqReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::ReqReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 8 {
    let res: Result<ReqRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::ReqRep(msg)),
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
    let res: Result<RegisterBackendReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RegisterBackendReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 68 {
    let res: Result<RegisterBackendRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RegisterBackendRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 69 {
    let res: Result<RegisterServerReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RegisterServerReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 70 {
    let res: Result<RegisterServerRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RegisterServerRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 91 {
    let res: Result<AddRoutesReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::AddRoutesReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 92 {
    let res: Result<AddRoutesRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::AddRoutesRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 95 {
    let res: Result<GetRoutesReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetRoutesReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 96 {
    let res: Result<GetRoutesRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetRoutesRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 100 {
    let res: Result<RouteAddedMsg, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RouteAddedMsg(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 101 {
    let res: Result<RouteDeletedMsg, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RouteDeletedMsg(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 102 {
    let res: Result<RouteHealthChangedMsg, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RouteHealthChangedMsg(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 111 {
    let res: Result<AssignFrontendReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::AssignFrontendReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 112 {
    let res: Result<AssignFrontendRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::AssignFrontendRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 113 {
    let res: Result<LocateTopicReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::LocateTopicReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 114 {
    let res: Result<LocateTopicRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::LocateTopicRep(msg)),
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

pub fn set_ref(protocol_msg: &mut ProtocolMsg, r#ref: u32) -> &ProtocolMsg {
  match protocol_msg {
    ProtocolMsg::None => panic!("Failed to set ref for ProtocolMsg::None"),
    ProtocolMsg::PingReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::PingRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::PushReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::PushRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::PullReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::PullRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::ReqReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::ReqRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::AuthReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::AuthRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::OkRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::ErrorRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::Ok2Rep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::Error2Rep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RegisterFrontendReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RegisterFrontendRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RegisterBackendReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RegisterBackendRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RegisterServerReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RegisterServerRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::AddRoutesReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::AddRoutesRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::GetRoutesReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::GetRoutesRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RouteAddedMsg(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RouteDeletedMsg(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RouteHealthChangedMsg(msg) => msg.r#ref = r#ref,
    ProtocolMsg::AssignFrontendReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::AssignFrontendRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::LocateTopicReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::LocateTopicRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::ResolveIpReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::ResolveIpRep(msg) => msg.r#ref = r#ref,
  }
  protocol_msg
}

pub fn get_ref(protocol_msg: &ProtocolMsg) -> u32 {
  match protocol_msg {
    ProtocolMsg::None => panic!("Failed to get ref from ProtocolMsg::None"),
    ProtocolMsg::PingReq(msg) => msg.r#ref,
    ProtocolMsg::PingRep(msg) => msg.r#ref,
    ProtocolMsg::PushReq(msg) => msg.r#ref,
    ProtocolMsg::PushRep(msg) => msg.r#ref,
    ProtocolMsg::PullReq(msg) => msg.r#ref,
    ProtocolMsg::PullRep(msg) => msg.r#ref,
    ProtocolMsg::ReqReq(msg) => msg.r#ref,
    ProtocolMsg::ReqRep(msg) => msg.r#ref,
    ProtocolMsg::AuthReq(msg) => msg.r#ref,
    ProtocolMsg::AuthRep(msg) => msg.r#ref,
    ProtocolMsg::OkRep(msg) => msg.r#ref,
    ProtocolMsg::ErrorRep(msg) => msg.r#ref,
    ProtocolMsg::Ok2Rep(msg) => msg.r#ref,
    ProtocolMsg::Error2Rep(msg) => msg.r#ref,
    ProtocolMsg::RegisterFrontendReq(msg) => msg.r#ref,
    ProtocolMsg::RegisterFrontendRep(msg) => msg.r#ref,
    ProtocolMsg::RegisterBackendReq(msg) => msg.r#ref,
    ProtocolMsg::RegisterBackendRep(msg) => msg.r#ref,
    ProtocolMsg::RegisterServerReq(msg) => msg.r#ref,
    ProtocolMsg::RegisterServerRep(msg) => msg.r#ref,
    ProtocolMsg::AddRoutesReq(msg) => msg.r#ref,
    ProtocolMsg::AddRoutesRep(msg) => msg.r#ref,
    ProtocolMsg::GetRoutesReq(msg) => msg.r#ref,
    ProtocolMsg::GetRoutesRep(msg) => msg.r#ref,
    ProtocolMsg::RouteAddedMsg(msg) => msg.r#ref,
    ProtocolMsg::RouteDeletedMsg(msg) => msg.r#ref,
    ProtocolMsg::RouteHealthChangedMsg(msg) => msg.r#ref,
    ProtocolMsg::AssignFrontendReq(msg) => msg.r#ref,
    ProtocolMsg::AssignFrontendRep(msg) => msg.r#ref,
    ProtocolMsg::LocateTopicReq(msg) => msg.r#ref,
    ProtocolMsg::LocateTopicRep(msg) => msg.r#ref,
    ProtocolMsg::ResolveIpReq(msg) => msg.r#ref,
    ProtocolMsg::ResolveIpRep(msg) => msg.r#ref,
  }
}