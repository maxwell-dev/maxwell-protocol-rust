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
  OkRep(OkRep),
  ErrorRep(ErrorRep),
  Ok2Rep(Ok2Rep),
  Error2Rep(Error2Rep),
  PushReq(PushReq),
  PushRep(PushRep),
  PullReq(PullReq),
  PullRep(PullRep),
  ReqReq(ReqReq),
  ReqRep(ReqRep),
  AuthReq(AuthReq),
  AuthRep(AuthRep),
  RegisterFrontendReq(RegisterFrontendReq),
  RegisterFrontendRep(RegisterFrontendRep),
  RegisterBackendReq(RegisterBackendReq),
  RegisterBackendRep(RegisterBackendRep),
  RegisterServiceReq(RegisterServiceReq),
  RegisterServiceRep(RegisterServiceRep),
  SetRoutesReq(SetRoutesReq),
  SetRoutesRep(SetRoutesRep),
  GetRoutesReq(GetRoutesReq),
  GetRoutesRep(GetRoutesRep),
  GetTopicDistChecksumReq(GetTopicDistChecksumReq),
  GetTopicDistChecksumRep(GetTopicDistChecksumRep),
  GetRouteDistChecksumReq(GetRouteDistChecksumReq),
  GetRouteDistChecksumRep(GetRouteDistChecksumRep),
  PickFrontendReq(PickFrontendReq),
  PickFrontendRep(PickFrontendRep),
  PickFrontendsReq(PickFrontendsReq),
  PickFrontendsRep(PickFrontendsRep),
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
      ProtocolMsg::OkRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ErrorRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::Ok2Rep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::Error2Rep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PushReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PushRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PullReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PullRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ReqReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ReqRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::AuthReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::AuthRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterFrontendReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterFrontendRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterBackendReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterBackendRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterServiceReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RegisterServiceRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::SetRoutesReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::SetRoutesRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetRoutesReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetRoutesRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetTopicDistChecksumReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetTopicDistChecksumRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetRouteDistChecksumReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetRouteDistChecksumRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PickFrontendReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PickFrontendRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PickFrontendsReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PickFrontendsRep(msg) => write!(f, "{:?}", msg),
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

impl ActixMessage for RegisterServiceReq {
  type Result = StdResult<RegisterServiceRep, HandleError<RegisterServiceReq>>;
}

impl ActixMessage for SetRoutesReq {
  type Result = StdResult<SetRoutesRep, HandleError<SetRoutesReq>>;
}

impl ActixMessage for GetRoutesReq {
  type Result = StdResult<GetRoutesRep, HandleError<GetRoutesReq>>;
}

impl ActixMessage for GetTopicDistChecksumReq {
  type Result = StdResult<GetTopicDistChecksumRep, HandleError<GetTopicDistChecksumReq>>;
}

impl ActixMessage for GetRouteDistChecksumReq {
  type Result = StdResult<GetRouteDistChecksumRep, HandleError<GetRouteDistChecksumReq>>;
}

impl ActixMessage for PickFrontendReq {
  type Result = StdResult<PickFrontendRep, HandleError<PickFrontendReq>>;
}

impl ActixMessage for PickFrontendsReq {
  type Result = StdResult<PickFrontendsRep, HandleError<PickFrontendsReq>>;
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

impl IntoEnum for RegisterServiceReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::RegisterServiceReq(self)
  }
}

impl IntoEnum for RegisterServiceRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::RegisterServiceRep(self)
  }
}

impl IntoEnum for SetRoutesReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::SetRoutesReq(self)
  }
}

impl IntoEnum for SetRoutesRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::SetRoutesRep(self)
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

impl IntoEnum for GetTopicDistChecksumReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetTopicDistChecksumReq(self)
  }
}

impl IntoEnum for GetTopicDistChecksumRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetTopicDistChecksumRep(self)
  }
}

impl IntoEnum for GetRouteDistChecksumReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetRouteDistChecksumReq(self)
  }
}

impl IntoEnum for GetRouteDistChecksumRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetRouteDistChecksumRep(self)
  }
}

impl IntoEnum for PickFrontendReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PickFrontendReq(self)
  }
}

impl IntoEnum for PickFrontendRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PickFrontendRep(self)
  }
}

impl IntoEnum for PickFrontendsReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PickFrontendsReq(self)
  }
}

impl IntoEnum for PickFrontendsRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PickFrontendsRep(self)
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

impl From<ProtocolMsg> for PingReq {
  #[inline]
  fn from(item: ProtocolMsg) -> PingReq {
    match item {
      ProtocolMsg::PingReq(msg) => msg,
      _ => panic!("Unable to convert to PingReq"),
    }
  }
}

impl From<ProtocolMsg> for PingRep {
  #[inline]
  fn from(item: ProtocolMsg) -> PingRep {
    match item {
      ProtocolMsg::PingRep(msg) => msg,
      _ => panic!("Unable to convert to PingRep"),
    }
  }
}

impl From<ProtocolMsg> for OkRep {
  #[inline]
  fn from(item: ProtocolMsg) -> OkRep {
    match item {
      ProtocolMsg::OkRep(msg) => msg,
      _ => panic!("Unable to convert to OkRep"),
    }
  }
}

impl From<ProtocolMsg> for ErrorRep {
  #[inline]
  fn from(item: ProtocolMsg) -> ErrorRep {
    match item {
      ProtocolMsg::ErrorRep(msg) => msg,
      _ => panic!("Unable to convert to ErrorRep"),
    }
  }
}

impl From<ProtocolMsg> for Ok2Rep {
  #[inline]
  fn from(item: ProtocolMsg) -> Ok2Rep {
    match item {
      ProtocolMsg::Ok2Rep(msg) => msg,
      _ => panic!("Unable to convert to Ok2Rep"),
    }
  }
}

impl From<ProtocolMsg> for Error2Rep {
  #[inline]
  fn from(item: ProtocolMsg) -> Error2Rep {
    match item {
      ProtocolMsg::Error2Rep(msg) => msg,
      _ => panic!("Unable to convert to Error2Rep"),
    }
  }
}

impl From<ProtocolMsg> for PushReq {
  #[inline]
  fn from(item: ProtocolMsg) -> PushReq {
    match item {
      ProtocolMsg::PushReq(msg) => msg,
      _ => panic!("Unable to convert to PushReq"),
    }
  }
}

impl From<ProtocolMsg> for PushRep {
  #[inline]
  fn from(item: ProtocolMsg) -> PushRep {
    match item {
      ProtocolMsg::PushRep(msg) => msg,
      _ => panic!("Unable to convert to PushRep"),
    }
  }
}

impl From<ProtocolMsg> for PullReq {
  #[inline]
  fn from(item: ProtocolMsg) -> PullReq {
    match item {
      ProtocolMsg::PullReq(msg) => msg,
      _ => panic!("Unable to convert to PullReq"),
    }
  }
}

impl From<ProtocolMsg> for PullRep {
  #[inline]
  fn from(item: ProtocolMsg) -> PullRep {
    match item {
      ProtocolMsg::PullRep(msg) => msg,
      _ => panic!("Unable to convert to PullRep"),
    }
  }
}

impl From<ProtocolMsg> for ReqReq {
  #[inline]
  fn from(item: ProtocolMsg) -> ReqReq {
    match item {
      ProtocolMsg::ReqReq(msg) => msg,
      _ => panic!("Unable to convert to ReqReq"),
    }
  }
}

impl From<ProtocolMsg> for ReqRep {
  #[inline]
  fn from(item: ProtocolMsg) -> ReqRep {
    match item {
      ProtocolMsg::ReqRep(msg) => msg,
      _ => panic!("Unable to convert to ReqRep"),
    }
  }
}

impl From<ProtocolMsg> for AuthReq {
  #[inline]
  fn from(item: ProtocolMsg) -> AuthReq {
    match item {
      ProtocolMsg::AuthReq(msg) => msg,
      _ => panic!("Unable to convert to AuthReq"),
    }
  }
}

impl From<ProtocolMsg> for AuthRep {
  #[inline]
  fn from(item: ProtocolMsg) -> AuthRep {
    match item {
      ProtocolMsg::AuthRep(msg) => msg,
      _ => panic!("Unable to convert to AuthRep"),
    }
  }
}

impl From<ProtocolMsg> for RegisterFrontendReq {
  #[inline]
  fn from(item: ProtocolMsg) -> RegisterFrontendReq {
    match item {
      ProtocolMsg::RegisterFrontendReq(msg) => msg,
      _ => panic!("Unable to convert to RegisterFrontendReq"),
    }
  }
}

impl From<ProtocolMsg> for RegisterFrontendRep {
  #[inline]
  fn from(item: ProtocolMsg) -> RegisterFrontendRep {
    match item {
      ProtocolMsg::RegisterFrontendRep(msg) => msg,
      _ => panic!("Unable to convert to RegisterFrontendRep"),
    }
  }
}

impl From<ProtocolMsg> for RegisterBackendReq {
  #[inline]
  fn from(item: ProtocolMsg) -> RegisterBackendReq {
    match item {
      ProtocolMsg::RegisterBackendReq(msg) => msg,
      _ => panic!("Unable to convert to RegisterBackendReq"),
    }
  }
}

impl From<ProtocolMsg> for RegisterBackendRep {
  #[inline]
  fn from(item: ProtocolMsg) -> RegisterBackendRep {
    match item {
      ProtocolMsg::RegisterBackendRep(msg) => msg,
      _ => panic!("Unable to convert to RegisterBackendRep"),
    }
  }
}

impl From<ProtocolMsg> for RegisterServiceReq {
  #[inline]
  fn from(item: ProtocolMsg) -> RegisterServiceReq {
    match item {
      ProtocolMsg::RegisterServiceReq(msg) => msg,
      _ => panic!("Unable to convert to RegisterServiceReq"),
    }
  }
}

impl From<ProtocolMsg> for RegisterServiceRep {
  #[inline]
  fn from(item: ProtocolMsg) -> RegisterServiceRep {
    match item {
      ProtocolMsg::RegisterServiceRep(msg) => msg,
      _ => panic!("Unable to convert to RegisterServiceRep"),
    }
  }
}

impl From<ProtocolMsg> for SetRoutesReq {
  #[inline]
  fn from(item: ProtocolMsg) -> SetRoutesReq {
    match item {
      ProtocolMsg::SetRoutesReq(msg) => msg,
      _ => panic!("Unable to convert to SetRoutesReq"),
    }
  }
}

impl From<ProtocolMsg> for SetRoutesRep {
  #[inline]
  fn from(item: ProtocolMsg) -> SetRoutesRep {
    match item {
      ProtocolMsg::SetRoutesRep(msg) => msg,
      _ => panic!("Unable to convert to SetRoutesRep"),
    }
  }
}

impl From<ProtocolMsg> for GetRoutesReq {
  #[inline]
  fn from(item: ProtocolMsg) -> GetRoutesReq {
    match item {
      ProtocolMsg::GetRoutesReq(msg) => msg,
      _ => panic!("Unable to convert to GetRoutesReq"),
    }
  }
}

impl From<ProtocolMsg> for GetRoutesRep {
  #[inline]
  fn from(item: ProtocolMsg) -> GetRoutesRep {
    match item {
      ProtocolMsg::GetRoutesRep(msg) => msg,
      _ => panic!("Unable to convert to GetRoutesRep"),
    }
  }
}

impl From<ProtocolMsg> for GetTopicDistChecksumReq {
  #[inline]
  fn from(item: ProtocolMsg) -> GetTopicDistChecksumReq {
    match item {
      ProtocolMsg::GetTopicDistChecksumReq(msg) => msg,
      _ => panic!("Unable to convert to GetTopicDistChecksumReq"),
    }
  }
}

impl From<ProtocolMsg> for GetTopicDistChecksumRep {
  #[inline]
  fn from(item: ProtocolMsg) -> GetTopicDistChecksumRep {
    match item {
      ProtocolMsg::GetTopicDistChecksumRep(msg) => msg,
      _ => panic!("Unable to convert to GetTopicDistChecksumRep"),
    }
  }
}

impl From<ProtocolMsg> for GetRouteDistChecksumReq {
  #[inline]
  fn from(item: ProtocolMsg) -> GetRouteDistChecksumReq {
    match item {
      ProtocolMsg::GetRouteDistChecksumReq(msg) => msg,
      _ => panic!("Unable to convert to GetRouteDistChecksumReq"),
    }
  }
}

impl From<ProtocolMsg> for GetRouteDistChecksumRep {
  #[inline]
  fn from(item: ProtocolMsg) -> GetRouteDistChecksumRep {
    match item {
      ProtocolMsg::GetRouteDistChecksumRep(msg) => msg,
      _ => panic!("Unable to convert to GetRouteDistChecksumRep"),
    }
  }
}

impl From<ProtocolMsg> for PickFrontendReq {
  #[inline]
  fn from(item: ProtocolMsg) -> PickFrontendReq {
    match item {
      ProtocolMsg::PickFrontendReq(msg) => msg,
      _ => panic!("Unable to convert to PickFrontendReq"),
    }
  }
}

impl From<ProtocolMsg> for PickFrontendRep {
  #[inline]
  fn from(item: ProtocolMsg) -> PickFrontendRep {
    match item {
      ProtocolMsg::PickFrontendRep(msg) => msg,
      _ => panic!("Unable to convert to PickFrontendRep"),
    }
  }
}

impl From<ProtocolMsg> for PickFrontendsReq {
  #[inline]
  fn from(item: ProtocolMsg) -> PickFrontendsReq {
    match item {
      ProtocolMsg::PickFrontendsReq(msg) => msg,
      _ => panic!("Unable to convert to PickFrontendsReq"),
    }
  }
}

impl From<ProtocolMsg> for PickFrontendsRep {
  #[inline]
  fn from(item: ProtocolMsg) -> PickFrontendsRep {
    match item {
      ProtocolMsg::PickFrontendsRep(msg) => msg,
      _ => panic!("Unable to convert to PickFrontendsRep"),
    }
  }
}

impl From<ProtocolMsg> for LocateTopicReq {
  #[inline]
  fn from(item: ProtocolMsg) -> LocateTopicReq {
    match item {
      ProtocolMsg::LocateTopicReq(msg) => msg,
      _ => panic!("Unable to convert to LocateTopicReq"),
    }
  }
}

impl From<ProtocolMsg> for LocateTopicRep {
  #[inline]
  fn from(item: ProtocolMsg) -> LocateTopicRep {
    match item {
      ProtocolMsg::LocateTopicRep(msg) => msg,
      _ => panic!("Unable to convert to LocateTopicRep"),
    }
  }
}

impl From<ProtocolMsg> for ResolveIpReq {
  #[inline]
  fn from(item: ProtocolMsg) -> ResolveIpReq {
    match item {
      ProtocolMsg::ResolveIpReq(msg) => msg,
      _ => panic!("Unable to convert to ResolveIpReq"),
    }
  }
}

impl From<ProtocolMsg> for ResolveIpRep {
  #[inline]
  fn from(item: ProtocolMsg) -> ResolveIpRep {
    match item {
      ProtocolMsg::ResolveIpRep(msg) => msg,
      _ => panic!("Unable to convert to ResolveIpRep"),
    }
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

impl Encode for PushReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(33);
  }
}

impl Encode for PushRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(34);
  }
}

impl Encode for PullReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(35);
  }
}

impl Encode for PullRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(36);
  }
}

impl Encode for ReqReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(39);
  }
}

impl Encode for ReqRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(40);
  }
}

impl Encode for AuthReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(41);
  }
}

impl Encode for AuthRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(42);
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

impl Encode for RegisterServiceReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(69);
  }
}

impl Encode for RegisterServiceRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(70);
  }
}

impl Encode for SetRoutesReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(71);
  }
}

impl Encode for SetRoutesRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(72);
  }
}

impl Encode for GetRoutesReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(75);
  }
}

impl Encode for GetRoutesRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(76);
  }
}

impl Encode for GetTopicDistChecksumReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(77);
  }
}

impl Encode for GetTopicDistChecksumRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(78);
  }
}

impl Encode for GetRouteDistChecksumReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(79);
  }
}

impl Encode for GetRouteDistChecksumRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(80);
  }
}

impl Encode for PickFrontendReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(81);
  }
}

impl Encode for PickFrontendRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(82);
  }
}

impl Encode for PickFrontendsReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(83);
  }
}

impl Encode for PickFrontendsRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(84);
  }
}

impl Encode for LocateTopicReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(85);
  }
}

impl Encode for LocateTopicRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(86);
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
    ProtocolMsg::OkRep(msg) => msg.encode_msg(),
    ProtocolMsg::ErrorRep(msg) => msg.encode_msg(),
    ProtocolMsg::Ok2Rep(msg) => msg.encode_msg(),
    ProtocolMsg::Error2Rep(msg) => msg.encode_msg(),
    ProtocolMsg::PushReq(msg) => msg.encode_msg(),
    ProtocolMsg::PushRep(msg) => msg.encode_msg(),
    ProtocolMsg::PullReq(msg) => msg.encode_msg(),
    ProtocolMsg::PullRep(msg) => msg.encode_msg(),
    ProtocolMsg::ReqReq(msg) => msg.encode_msg(),
    ProtocolMsg::ReqRep(msg) => msg.encode_msg(),
    ProtocolMsg::AuthReq(msg) => msg.encode_msg(),
    ProtocolMsg::AuthRep(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterFrontendReq(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterFrontendRep(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterBackendReq(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterBackendRep(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterServiceReq(msg) => msg.encode_msg(),
    ProtocolMsg::RegisterServiceRep(msg) => msg.encode_msg(),
    ProtocolMsg::SetRoutesReq(msg) => msg.encode_msg(),
    ProtocolMsg::SetRoutesRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetRoutesReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetRoutesRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetTopicDistChecksumReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetTopicDistChecksumRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetRouteDistChecksumReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetRouteDistChecksumRep(msg) => msg.encode_msg(),
    ProtocolMsg::PickFrontendReq(msg) => msg.encode_msg(),
    ProtocolMsg::PickFrontendRep(msg) => msg.encode_msg(),
    ProtocolMsg::PickFrontendsReq(msg) => msg.encode_msg(),
    ProtocolMsg::PickFrontendsRep(msg) => msg.encode_msg(),
    ProtocolMsg::LocateTopicReq(msg) => msg.encode_msg(),
    ProtocolMsg::LocateTopicRep(msg) => msg.encode_msg(),
    ProtocolMsg::ResolveIpReq(msg) => msg.encode_msg(),
    ProtocolMsg::ResolveIpRep(msg) => msg.encode_msg(),
  }
}

pub fn decode(bytes: &Bytes) -> Result<ProtocolMsg, DecodeError> {
  decode_bytes(bytes.as_ref())
}

pub fn decode_bytes(bytes: &[u8]) -> Result<ProtocolMsg, DecodeError> {
  let msg_type = bytes[0] as i8;
  let msg_body = &bytes[1..];
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
  } else if msg_type == 33 {
    let res: Result<PushReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PushReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 34 {
    let res: Result<PushRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PushRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 35 {
    let res: Result<PullReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PullReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 36 {
    let res: Result<PullRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PullRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 39 {
    let res: Result<ReqReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::ReqReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 40 {
    let res: Result<ReqRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::ReqRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 41 {
    let res: Result<AuthReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::AuthReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 42 {
    let res: Result<AuthRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::AuthRep(msg)),
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
    let res: Result<RegisterServiceReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RegisterServiceReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 70 {
    let res: Result<RegisterServiceRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RegisterServiceRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 71 {
    let res: Result<SetRoutesReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::SetRoutesReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 72 {
    let res: Result<SetRoutesRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::SetRoutesRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 75 {
    let res: Result<GetRoutesReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetRoutesReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 76 {
    let res: Result<GetRoutesRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetRoutesRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 77 {
    let res: Result<GetTopicDistChecksumReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetTopicDistChecksumReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 78 {
    let res: Result<GetTopicDistChecksumRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetTopicDistChecksumRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 79 {
    let res: Result<GetRouteDistChecksumReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetRouteDistChecksumReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 80 {
    let res: Result<GetRouteDistChecksumRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetRouteDistChecksumRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 81 {
    let res: Result<PickFrontendReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PickFrontendReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 82 {
    let res: Result<PickFrontendRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PickFrontendRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 83 {
    let res: Result<PickFrontendsReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PickFrontendsReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 84 {
    let res: Result<PickFrontendsRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PickFrontendsRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 85 {
    let res: Result<LocateTopicReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::LocateTopicReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 86 {
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
    ProtocolMsg::OkRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::ErrorRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::Ok2Rep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::Error2Rep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::PushReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::PushRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::PullReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::PullRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::ReqReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::ReqRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::AuthReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::AuthRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RegisterFrontendReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RegisterFrontendRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RegisterBackendReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RegisterBackendRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RegisterServiceReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::RegisterServiceRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::SetRoutesReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::SetRoutesRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::GetRoutesReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::GetRoutesRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::GetTopicDistChecksumReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::GetTopicDistChecksumRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::GetRouteDistChecksumReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::GetRouteDistChecksumRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::PickFrontendReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::PickFrontendRep(msg) => msg.r#ref = r#ref,
    ProtocolMsg::PickFrontendsReq(msg) => msg.r#ref = r#ref,
    ProtocolMsg::PickFrontendsRep(msg) => msg.r#ref = r#ref,
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
    ProtocolMsg::OkRep(msg) => msg.r#ref,
    ProtocolMsg::ErrorRep(msg) => msg.r#ref,
    ProtocolMsg::Ok2Rep(msg) => msg.r#ref,
    ProtocolMsg::Error2Rep(msg) => msg.r#ref,
    ProtocolMsg::PushReq(msg) => msg.r#ref,
    ProtocolMsg::PushRep(msg) => msg.r#ref,
    ProtocolMsg::PullReq(msg) => msg.r#ref,
    ProtocolMsg::PullRep(msg) => msg.r#ref,
    ProtocolMsg::ReqReq(msg) => msg.r#ref,
    ProtocolMsg::ReqRep(msg) => msg.r#ref,
    ProtocolMsg::AuthReq(msg) => msg.r#ref,
    ProtocolMsg::AuthRep(msg) => msg.r#ref,
    ProtocolMsg::RegisterFrontendReq(msg) => msg.r#ref,
    ProtocolMsg::RegisterFrontendRep(msg) => msg.r#ref,
    ProtocolMsg::RegisterBackendReq(msg) => msg.r#ref,
    ProtocolMsg::RegisterBackendRep(msg) => msg.r#ref,
    ProtocolMsg::RegisterServiceReq(msg) => msg.r#ref,
    ProtocolMsg::RegisterServiceRep(msg) => msg.r#ref,
    ProtocolMsg::SetRoutesReq(msg) => msg.r#ref,
    ProtocolMsg::SetRoutesRep(msg) => msg.r#ref,
    ProtocolMsg::GetRoutesReq(msg) => msg.r#ref,
    ProtocolMsg::GetRoutesRep(msg) => msg.r#ref,
    ProtocolMsg::GetTopicDistChecksumReq(msg) => msg.r#ref,
    ProtocolMsg::GetTopicDistChecksumRep(msg) => msg.r#ref,
    ProtocolMsg::GetRouteDistChecksumReq(msg) => msg.r#ref,
    ProtocolMsg::GetRouteDistChecksumRep(msg) => msg.r#ref,
    ProtocolMsg::PickFrontendReq(msg) => msg.r#ref,
    ProtocolMsg::PickFrontendRep(msg) => msg.r#ref,
    ProtocolMsg::PickFrontendsReq(msg) => msg.r#ref,
    ProtocolMsg::PickFrontendsRep(msg) => msg.r#ref,
    ProtocolMsg::LocateTopicReq(msg) => msg.r#ref,
    ProtocolMsg::LocateTopicRep(msg) => msg.r#ref,
    ProtocolMsg::ResolveIpReq(msg) => msg.r#ref,
    ProtocolMsg::ResolveIpRep(msg) => msg.r#ref,
  }
}