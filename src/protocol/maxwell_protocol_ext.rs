use super::maxwell_protocol::*;
use bytes::{BufMut, Bytes, BytesMut};
pub use prost::DecodeError;
use prost::Message;
use std::fmt::{Debug, Formatter, Result as FmtResult};

pub enum ProtocolMsg {
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
            ProtocolMsg::PingReq(msg) => {
                write!(f, "PingReq: {{{:?}}}", msg)
            }
            ProtocolMsg::PingRep(msg) => {
                write!(f, "PingRep: {{{:?}}}", msg)
            }
            ProtocolMsg::PullReq(msg) => {
                write!(f, "PullReq: {{{:?}}}", msg)
            }
            ProtocolMsg::PullRep(msg) => {
                write!(f, "PullRep: {{{:?}}}", msg)
            }
            ProtocolMsg::PushReq(msg) => {
                write!(f, "PushReq: {{{:?}}}", msg)
            }
            ProtocolMsg::PushRep(msg) => {
                write!(f, "PushRep: {{{:?}}}", msg)
            }
            ProtocolMsg::DoReq(msg) => {
                write!(f, "DoReq: {{{:?}}}", msg)
            }
            ProtocolMsg::DoRep(msg) => {
                write!(f, "DoRep: {{{:?}}}", msg)
            }
            ProtocolMsg::Do2Req(msg) => {
                write!(f, "Do2Req: {{{:?}}}", msg)
            }
            ProtocolMsg::Do2Rep(msg) => {
                write!(f, "Do2Rep: {{{:?}}}", msg)
            }
            ProtocolMsg::AuthReq(msg) => {
                write!(f, "AuthReq: {{{:?}}}", msg)
            }
            ProtocolMsg::AuthRep(msg) => {
                write!(f, "AuthRep: {{{:?}}}", msg)
            }
            ProtocolMsg::OkRep(msg) => {
                write!(f, "OkRep: {{{:?}}}", msg)
            }
            ProtocolMsg::ErrorRep(msg) => {
                write!(f, "ErrorRep: {{{:?}}}", msg)
            }
            ProtocolMsg::Ok2Rep(msg) => {
                write!(f, "Ok2Rep: {{{:?}}}", msg)
            }
            ProtocolMsg::Error2Rep(msg) => {
                write!(f, "Error2Rep: {{{:?}}}", msg)
            }
            ProtocolMsg::RegisterFrontendReq(msg) => {
                write!(f, "RegisterFrontendReq: {{{:?}}}", msg)
            }
            ProtocolMsg::RegisterFrontendRep(msg) => {
                write!(f, "RegisterFrontendRep: {{{:?}}}", msg)
            }
            ProtocolMsg::AddRouteReq(msg) => {
                write!(f, "AddRouteReq: {{{:?}}}", msg)
            }
            ProtocolMsg::AddRouteRep(msg) => {
                write!(f, "AddRouteRep: {{{:?}}}", msg)
            }
            ProtocolMsg::DeleteRouteReq(msg) => {
                write!(f, "DeleteRouteReq: {{{:?}}}", msg)
            }
            ProtocolMsg::DeleteRouteRep(msg) => {
                write!(f, "DeleteRouteRep: {{{:?}}}", msg)
            }
            ProtocolMsg::AddRouteMsg(msg) => {
                write!(f, "AddRouteMsg: {{{:?}}}", msg)
            }
            ProtocolMsg::DeleteRouteMsg(msg) => {
                write!(f, "DeleteRouteMsg: {{{:?}}}", msg)
            }
            ProtocolMsg::PushRoutesReq(msg) => {
                write!(f, "PushRoutesReq: {{{:?}}}", msg)
            }
            ProtocolMsg::PushRoutesRep(msg) => {
                write!(f, "PushRoutesRep: {{{:?}}}", msg)
            }
            ProtocolMsg::PullRoutesReq(msg) => {
                write!(f, "PullRoutesReq: {{{:?}}}", msg)
            }
            ProtocolMsg::PullRoutesRep(msg) => {
                write!(f, "PullRoutesRep: {{{:?}}}", msg)
            }
            ProtocolMsg::RegisterBackendReq(msg) => {
                write!(f, "RegisterBackendReq: {{{:?}}}", msg)
            }
            ProtocolMsg::RegisterBackendRep(msg) => {
                write!(f, "RegisterBackendRep: {{{:?}}}", msg)
            }
            ProtocolMsg::DeleteTopicsReq(msg) => {
                write!(f, "DeleteTopicsReq: {{{:?}}}", msg)
            }
            ProtocolMsg::DeleteTopicsRep(msg) => {
                write!(f, "DeleteTopicsRep: {{{:?}}}", msg)
            }
            ProtocolMsg::ResolveFrontendReq(msg) => {
                write!(f, "ResolveFrontendReq: {{{:?}}}", msg)
            }
            ProtocolMsg::ResolveFrontendRep(msg) => {
                write!(f, "ResolveFrontendRep: {{{:?}}}", msg)
            }
            ProtocolMsg::ResolveBackendReq(msg) => {
                write!(f, "ResolveBackendReq: {{{:?}}}", msg)
            }
            ProtocolMsg::ResolveBackendRep(msg) => {
                write!(f, "ResolveBackendRep: {{{:?}}}", msg)
            }
            ProtocolMsg::WatchReq(msg) => {
                write!(f, "WatchReq: {{{:?}}}", msg)
            }
            ProtocolMsg::WatchRep(msg) => {
                write!(f, "WatchRep: {{{:?}}}", msg)
            }
            ProtocolMsg::UnwatchReq(msg) => {
                write!(f, "UnwatchReq: {{{:?}}}", msg)
            }
            ProtocolMsg::UnwatchRep(msg) => {
                write!(f, "UnwatchRep: {{{:?}}}", msg)
            }
            ProtocolMsg::ResolveIpReq(msg) => {
                write!(f, "ResolveIpReq: {{{:?}}}", msg)
            }
            ProtocolMsg::ResolveIpRep(msg) => {
                write!(f, "ResolveIpRep: {{{:?}}}", msg)
            }
        }
    }
}

pub trait IntoProtocol {
    fn into_protocol(self) -> ProtocolMsg;
}

impl IntoProtocol for PingReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::PingReq(self)
    }
}

impl IntoProtocol for PingRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::PingRep(self)
    }
}

impl IntoProtocol for PullReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::PullReq(self)
    }
}

impl IntoProtocol for PullRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::PullRep(self)
    }
}

impl IntoProtocol for PushReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::PushReq(self)
    }
}

impl IntoProtocol for PushRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::PushRep(self)
    }
}

impl IntoProtocol for DoReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::DoReq(self)
    }
}

impl IntoProtocol for DoRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::DoRep(self)
    }
}

impl IntoProtocol for Do2Req {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::Do2Req(self)
    }
}

impl IntoProtocol for Do2Rep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::Do2Rep(self)
    }
}

impl IntoProtocol for AuthReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::AuthReq(self)
    }
}

impl IntoProtocol for AuthRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::AuthRep(self)
    }
}

impl IntoProtocol for OkRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::OkRep(self)
    }
}

impl IntoProtocol for ErrorRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::ErrorRep(self)
    }
}

impl IntoProtocol for Ok2Rep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::Ok2Rep(self)
    }
}

impl IntoProtocol for Error2Rep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::Error2Rep(self)
    }
}

impl IntoProtocol for RegisterFrontendReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::RegisterFrontendReq(self)
    }
}

impl IntoProtocol for RegisterFrontendRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::RegisterFrontendRep(self)
    }
}

impl IntoProtocol for AddRouteReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::AddRouteReq(self)
    }
}

impl IntoProtocol for AddRouteRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::AddRouteRep(self)
    }
}

impl IntoProtocol for DeleteRouteReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::DeleteRouteReq(self)
    }
}

impl IntoProtocol for DeleteRouteRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::DeleteRouteRep(self)
    }
}

impl IntoProtocol for AddRouteMsg {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::AddRouteMsg(self)
    }
}

impl IntoProtocol for DeleteRouteMsg {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::DeleteRouteMsg(self)
    }
}

impl IntoProtocol for PushRoutesReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::PushRoutesReq(self)
    }
}

impl IntoProtocol for PushRoutesRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::PushRoutesRep(self)
    }
}

impl IntoProtocol for PullRoutesReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::PullRoutesReq(self)
    }
}

impl IntoProtocol for PullRoutesRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::PullRoutesRep(self)
    }
}

impl IntoProtocol for RegisterBackendReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::RegisterBackendReq(self)
    }
}

impl IntoProtocol for RegisterBackendRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::RegisterBackendRep(self)
    }
}

impl IntoProtocol for DeleteTopicsReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::DeleteTopicsReq(self)
    }
}

impl IntoProtocol for DeleteTopicsRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::DeleteTopicsRep(self)
    }
}

impl IntoProtocol for ResolveFrontendReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::ResolveFrontendReq(self)
    }
}

impl IntoProtocol for ResolveFrontendRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::ResolveFrontendRep(self)
    }
}

impl IntoProtocol for ResolveBackendReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::ResolveBackendReq(self)
    }
}

impl IntoProtocol for ResolveBackendRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::ResolveBackendRep(self)
    }
}

impl IntoProtocol for WatchReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::WatchReq(self)
    }
}

impl IntoProtocol for WatchRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::WatchRep(self)
    }
}

impl IntoProtocol for UnwatchReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::UnwatchReq(self)
    }
}

impl IntoProtocol for UnwatchRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::UnwatchRep(self)
    }
}

impl IntoProtocol for ResolveIpReq {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::ResolveIpReq(self)
    }
}

impl IntoProtocol for ResolveIpRep {
    #[inline]
    fn into_protocol(self) -> ProtocolMsg {
        ProtocolMsg::ResolveIpRep(self)
    }
}

pub trait EncodeInto: Message + Sized {
    fn encode_type(bytes: &mut BytesMut);

    fn encode_into(&self) -> Bytes {
        let size = self.encoded_len() as usize;
        let mut bytes = BytesMut::with_capacity(size + 1);
        Self::encode_type(&mut bytes);
        if let Err(err) = self.encode(&mut bytes) {
            panic!("Failed to encode msg: {:?}", err);
        }
        return bytes.freeze();
    }
}

impl EncodeInto for PingReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(1);
    }
}

impl EncodeInto for PingRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(2);
    }
}

impl EncodeInto for PullReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(3);
    }
}

impl EncodeInto for PullRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(4);
    }
}

impl EncodeInto for PushReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(5);
    }
}

impl EncodeInto for PushRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(6);
    }
}

impl EncodeInto for DoReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(7);
    }
}

impl EncodeInto for DoRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(8);
    }
}

impl EncodeInto for Do2Req {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(9);
    }
}

impl EncodeInto for Do2Rep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(10);
    }
}

impl EncodeInto for AuthReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(27);
    }
}

impl EncodeInto for AuthRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(28);
    }
}

impl EncodeInto for OkRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(29);
    }
}

impl EncodeInto for ErrorRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(30);
    }
}

impl EncodeInto for Ok2Rep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(31);
    }
}

impl EncodeInto for Error2Rep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(32);
    }
}

impl EncodeInto for RegisterFrontendReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(65);
    }
}

impl EncodeInto for RegisterFrontendRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(66);
    }
}

impl EncodeInto for AddRouteReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(67);
    }
}

impl EncodeInto for AddRouteRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(68);
    }
}

impl EncodeInto for DeleteRouteReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(69);
    }
}

impl EncodeInto for DeleteRouteRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(70);
    }
}

impl EncodeInto for AddRouteMsg {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(71);
    }
}

impl EncodeInto for DeleteRouteMsg {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(72);
    }
}

impl EncodeInto for PushRoutesReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(73);
    }
}

impl EncodeInto for PushRoutesRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(74);
    }
}

impl EncodeInto for PullRoutesReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(75);
    }
}

impl EncodeInto for PullRoutesRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(76);
    }
}

impl EncodeInto for RegisterBackendReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(81);
    }
}

impl EncodeInto for RegisterBackendRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(82);
    }
}

impl EncodeInto for DeleteTopicsReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(83);
    }
}

impl EncodeInto for DeleteTopicsRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(84);
    }
}

impl EncodeInto for ResolveFrontendReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(97);
    }
}

impl EncodeInto for ResolveFrontendRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(98);
    }
}

impl EncodeInto for ResolveBackendReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(99);
    }
}

impl EncodeInto for ResolveBackendRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(100);
    }
}

impl EncodeInto for WatchReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(105);
    }
}

impl EncodeInto for WatchRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(106);
    }
}

impl EncodeInto for UnwatchReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(107);
    }
}

impl EncodeInto for UnwatchRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(108);
    }
}

impl EncodeInto for ResolveIpReq {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(121);
    }
}

impl EncodeInto for ResolveIpRep {
    #[inline]
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(122);
    }
}

pub fn encode(protocol_msg: &ProtocolMsg) -> Bytes {
    match protocol_msg {
        ProtocolMsg::PingReq(msg) => msg.encode_into(),
        ProtocolMsg::PingRep(msg) => msg.encode_into(),
        ProtocolMsg::PullReq(msg) => msg.encode_into(),
        ProtocolMsg::PullRep(msg) => msg.encode_into(),
        ProtocolMsg::PushReq(msg) => msg.encode_into(),
        ProtocolMsg::PushRep(msg) => msg.encode_into(),
        ProtocolMsg::DoReq(msg) => msg.encode_into(),
        ProtocolMsg::DoRep(msg) => msg.encode_into(),
        ProtocolMsg::Do2Req(msg) => msg.encode_into(),
        ProtocolMsg::Do2Rep(msg) => msg.encode_into(),
        ProtocolMsg::AuthReq(msg) => msg.encode_into(),
        ProtocolMsg::AuthRep(msg) => msg.encode_into(),
        ProtocolMsg::OkRep(msg) => msg.encode_into(),
        ProtocolMsg::ErrorRep(msg) => msg.encode_into(),
        ProtocolMsg::Ok2Rep(msg) => msg.encode_into(),
        ProtocolMsg::Error2Rep(msg) => msg.encode_into(),
        ProtocolMsg::RegisterFrontendReq(msg) => msg.encode_into(),
        ProtocolMsg::RegisterFrontendRep(msg) => msg.encode_into(),
        ProtocolMsg::AddRouteReq(msg) => msg.encode_into(),
        ProtocolMsg::AddRouteRep(msg) => msg.encode_into(),
        ProtocolMsg::DeleteRouteReq(msg) => msg.encode_into(),
        ProtocolMsg::DeleteRouteRep(msg) => msg.encode_into(),
        ProtocolMsg::AddRouteMsg(msg) => msg.encode_into(),
        ProtocolMsg::DeleteRouteMsg(msg) => msg.encode_into(),
        ProtocolMsg::PushRoutesReq(msg) => msg.encode_into(),
        ProtocolMsg::PushRoutesRep(msg) => msg.encode_into(),
        ProtocolMsg::PullRoutesReq(msg) => msg.encode_into(),
        ProtocolMsg::PullRoutesRep(msg) => msg.encode_into(),
        ProtocolMsg::RegisterBackendReq(msg) => msg.encode_into(),
        ProtocolMsg::RegisterBackendRep(msg) => msg.encode_into(),
        ProtocolMsg::DeleteTopicsReq(msg) => msg.encode_into(),
        ProtocolMsg::DeleteTopicsRep(msg) => msg.encode_into(),
        ProtocolMsg::ResolveFrontendReq(msg) => msg.encode_into(),
        ProtocolMsg::ResolveFrontendRep(msg) => msg.encode_into(),
        ProtocolMsg::ResolveBackendReq(msg) => msg.encode_into(),
        ProtocolMsg::ResolveBackendRep(msg) => msg.encode_into(),
        ProtocolMsg::WatchReq(msg) => msg.encode_into(),
        ProtocolMsg::WatchRep(msg) => msg.encode_into(),
        ProtocolMsg::UnwatchReq(msg) => msg.encode_into(),
        ProtocolMsg::UnwatchRep(msg) => msg.encode_into(),
        ProtocolMsg::ResolveIpReq(msg) => msg.encode_into(),
        ProtocolMsg::ResolveIpRep(msg) => msg.encode_into(),
    }
}

pub fn decode(bytes: &Bytes) -> Result<ProtocolMsg, DecodeError> {
    let msg_type = bytes[0] as i8;
    let msg_bytes = bytes.slice(1..);
    if msg_type == 1 {
        let res: Result<PingReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::PingReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 2 {
        let res: Result<PingRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::PingRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 3 {
        let res: Result<PullReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::PullReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 4 {
        let res: Result<PullRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::PullRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 5 {
        let res: Result<PushReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::PushReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 6 {
        let res: Result<PushRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::PushRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 7 {
        let res: Result<DoReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::DoReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 8 {
        let res: Result<DoRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::DoRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 9 {
        let res: Result<Do2Req, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::Do2Req(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 10 {
        let res: Result<Do2Rep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::Do2Rep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 27 {
        let res: Result<AuthReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::AuthReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 28 {
        let res: Result<AuthRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::AuthRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 29 {
        let res: Result<OkRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::OkRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 30 {
        let res: Result<ErrorRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::ErrorRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 31 {
        let res: Result<Ok2Rep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::Ok2Rep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 32 {
        let res: Result<Error2Rep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::Error2Rep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 65 {
        let res: Result<RegisterFrontendReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::RegisterFrontendReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 66 {
        let res: Result<RegisterFrontendRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::RegisterFrontendRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 67 {
        let res: Result<AddRouteReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::AddRouteReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 68 {
        let res: Result<AddRouteRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::AddRouteRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 69 {
        let res: Result<DeleteRouteReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::DeleteRouteReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 70 {
        let res: Result<DeleteRouteRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::DeleteRouteRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 71 {
        let res: Result<AddRouteMsg, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::AddRouteMsg(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 72 {
        let res: Result<DeleteRouteMsg, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::DeleteRouteMsg(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 73 {
        let res: Result<PushRoutesReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::PushRoutesReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 74 {
        let res: Result<PushRoutesRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::PushRoutesRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 75 {
        let res: Result<PullRoutesReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::PullRoutesReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 76 {
        let res: Result<PullRoutesRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::PullRoutesRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 81 {
        let res: Result<RegisterBackendReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::RegisterBackendReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 82 {
        let res: Result<RegisterBackendRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::RegisterBackendRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 83 {
        let res: Result<DeleteTopicsReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::DeleteTopicsReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 84 {
        let res: Result<DeleteTopicsRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::DeleteTopicsRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 97 {
        let res: Result<ResolveFrontendReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::ResolveFrontendReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 98 {
        let res: Result<ResolveFrontendRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::ResolveFrontendRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 99 {
        let res: Result<ResolveBackendReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::ResolveBackendReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 100 {
        let res: Result<ResolveBackendRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::ResolveBackendRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 105 {
        let res: Result<WatchReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::WatchReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 106 {
        let res: Result<WatchRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::WatchRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 107 {
        let res: Result<UnwatchReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::UnwatchReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 108 {
        let res: Result<UnwatchRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::UnwatchRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 121 {
        let res: Result<ResolveIpReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(ProtocolMsg::ResolveIpReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 122 {
        let res: Result<ResolveIpRep, DecodeError> = Message::decode(msg_bytes);
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