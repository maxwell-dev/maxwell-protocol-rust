use super::maxwell_protocol::*;
use actix::Message as ActixMessage;
use bytes::{BufMut, Bytes, BytesMut};
pub use prost::DecodeError;
use prost::Message;
use std::fmt::{Debug, Formatter, Result as FmtResult};

pub enum BoxedMsg {
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

impl Debug for BoxedMsg {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            BoxedMsg::PingReq(msg) => {
                write!(f, "PingReq: {{{:?}}}", msg)
            }
            BoxedMsg::PingRep(msg) => {
                write!(f, "PingRep: {{{:?}}}", msg)
            }
            BoxedMsg::PullReq(msg) => {
                write!(f, "PullReq: {{{:?}}}", msg)
            }
            BoxedMsg::PullRep(msg) => {
                write!(f, "PullRep: {{{:?}}}", msg)
            }
            BoxedMsg::PushReq(msg) => {
                write!(f, "PushReq: {{{:?}}}", msg)
            }
            BoxedMsg::PushRep(msg) => {
                write!(f, "PushRep: {{{:?}}}", msg)
            }
            BoxedMsg::DoReq(msg) => {
                write!(f, "DoReq: {{{:?}}}", msg)
            }
            BoxedMsg::DoRep(msg) => {
                write!(f, "DoRep: {{{:?}}}", msg)
            }
            BoxedMsg::Do2Req(msg) => {
                write!(f, "Do2Req: {{{:?}}}", msg)
            }
            BoxedMsg::Do2Rep(msg) => {
                write!(f, "Do2Rep: {{{:?}}}", msg)
            }
            BoxedMsg::AuthReq(msg) => {
                write!(f, "AuthReq: {{{:?}}}", msg)
            }
            BoxedMsg::AuthRep(msg) => {
                write!(f, "AuthRep: {{{:?}}}", msg)
            }
            BoxedMsg::OkRep(msg) => {
                write!(f, "OkRep: {{{:?}}}", msg)
            }
            BoxedMsg::ErrorRep(msg) => {
                write!(f, "ErrorRep: {{{:?}}}", msg)
            }
            BoxedMsg::Ok2Rep(msg) => {
                write!(f, "Ok2Rep: {{{:?}}}", msg)
            }
            BoxedMsg::Error2Rep(msg) => {
                write!(f, "Error2Rep: {{{:?}}}", msg)
            }
            BoxedMsg::RegisterFrontendReq(msg) => {
                write!(f, "RegisterFrontendReq: {{{:?}}}", msg)
            }
            BoxedMsg::RegisterFrontendRep(msg) => {
                write!(f, "RegisterFrontendRep: {{{:?}}}", msg)
            }
            BoxedMsg::AddRouteReq(msg) => {
                write!(f, "AddRouteReq: {{{:?}}}", msg)
            }
            BoxedMsg::AddRouteRep(msg) => {
                write!(f, "AddRouteRep: {{{:?}}}", msg)
            }
            BoxedMsg::DeleteRouteReq(msg) => {
                write!(f, "DeleteRouteReq: {{{:?}}}", msg)
            }
            BoxedMsg::DeleteRouteRep(msg) => {
                write!(f, "DeleteRouteRep: {{{:?}}}", msg)
            }
            BoxedMsg::AddRouteMsg(msg) => {
                write!(f, "AddRouteMsg: {{{:?}}}", msg)
            }
            BoxedMsg::DeleteRouteMsg(msg) => {
                write!(f, "DeleteRouteMsg: {{{:?}}}", msg)
            }
            BoxedMsg::PushRoutesReq(msg) => {
                write!(f, "PushRoutesReq: {{{:?}}}", msg)
            }
            BoxedMsg::PushRoutesRep(msg) => {
                write!(f, "PushRoutesRep: {{{:?}}}", msg)
            }
            BoxedMsg::PullRoutesReq(msg) => {
                write!(f, "PullRoutesReq: {{{:?}}}", msg)
            }
            BoxedMsg::PullRoutesRep(msg) => {
                write!(f, "PullRoutesRep: {{{:?}}}", msg)
            }
            BoxedMsg::RegisterBackendReq(msg) => {
                write!(f, "RegisterBackendReq: {{{:?}}}", msg)
            }
            BoxedMsg::RegisterBackendRep(msg) => {
                write!(f, "RegisterBackendRep: {{{:?}}}", msg)
            }
            BoxedMsg::DeleteTopicsReq(msg) => {
                write!(f, "DeleteTopicsReq: {{{:?}}}", msg)
            }
            BoxedMsg::DeleteTopicsRep(msg) => {
                write!(f, "DeleteTopicsRep: {{{:?}}}", msg)
            }
            BoxedMsg::ResolveFrontendReq(msg) => {
                write!(f, "ResolveFrontendReq: {{{:?}}}", msg)
            }
            BoxedMsg::ResolveFrontendRep(msg) => {
                write!(f, "ResolveFrontendRep: {{{:?}}}", msg)
            }
            BoxedMsg::ResolveBackendReq(msg) => {
                write!(f, "ResolveBackendReq: {{{:?}}}", msg)
            }
            BoxedMsg::ResolveBackendRep(msg) => {
                write!(f, "ResolveBackendRep: {{{:?}}}", msg)
            }
            BoxedMsg::WatchReq(msg) => {
                write!(f, "WatchReq: {{{:?}}}", msg)
            }
            BoxedMsg::WatchRep(msg) => {
                write!(f, "WatchRep: {{{:?}}}", msg)
            }
            BoxedMsg::UnwatchReq(msg) => {
                write!(f, "UnwatchReq: {{{:?}}}", msg)
            }
            BoxedMsg::UnwatchRep(msg) => {
                write!(f, "UnwatchRep: {{{:?}}}", msg)
            }
            BoxedMsg::ResolveIpReq(msg) => {
                write!(f, "ResolveIpReq: {{{:?}}}", msg)
            }
            BoxedMsg::ResolveIpRep(msg) => {
                write!(f, "ResolveIpRep: {{{:?}}}", msg)
            }
        }
    }
}

pub struct SendError;

impl ActixMessage for BoxedMsg {
    type Result = Result<BoxedMsg, SendError>;
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
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(1);
    }
}

impl EncodeInto for PingRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(2);
    }
}

impl EncodeInto for PullReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(3);
    }
}

impl EncodeInto for PullRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(4);
    }
}

impl EncodeInto for PushReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(5);
    }
}

impl EncodeInto for PushRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(6);
    }
}

impl EncodeInto for DoReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(7);
    }
}

impl EncodeInto for DoRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(8);
    }
}

impl EncodeInto for Do2Req {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(9);
    }
}

impl EncodeInto for Do2Rep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(10);
    }
}

impl EncodeInto for AuthReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(27);
    }
}

impl EncodeInto for AuthRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(28);
    }
}

impl EncodeInto for OkRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(29);
    }
}

impl EncodeInto for ErrorRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(30);
    }
}

impl EncodeInto for Ok2Rep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(31);
    }
}

impl EncodeInto for Error2Rep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(32);
    }
}

impl EncodeInto for RegisterFrontendReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(65);
    }
}

impl EncodeInto for RegisterFrontendRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(66);
    }
}

impl EncodeInto for AddRouteReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(67);
    }
}

impl EncodeInto for AddRouteRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(68);
    }
}

impl EncodeInto for DeleteRouteReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(69);
    }
}

impl EncodeInto for DeleteRouteRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(70);
    }
}

impl EncodeInto for AddRouteMsg {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(71);
    }
}

impl EncodeInto for DeleteRouteMsg {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(72);
    }
}

impl EncodeInto for PushRoutesReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(73);
    }
}

impl EncodeInto for PushRoutesRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(74);
    }
}

impl EncodeInto for PullRoutesReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(75);
    }
}

impl EncodeInto for PullRoutesRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(76);
    }
}

impl EncodeInto for RegisterBackendReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(81);
    }
}

impl EncodeInto for RegisterBackendRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(82);
    }
}

impl EncodeInto for DeleteTopicsReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(83);
    }
}

impl EncodeInto for DeleteTopicsRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(84);
    }
}

impl EncodeInto for ResolveFrontendReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(97);
    }
}

impl EncodeInto for ResolveFrontendRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(98);
    }
}

impl EncodeInto for ResolveBackendReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(99);
    }
}

impl EncodeInto for ResolveBackendRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(100);
    }
}

impl EncodeInto for WatchReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(105);
    }
}

impl EncodeInto for WatchRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(106);
    }
}

impl EncodeInto for UnwatchReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(107);
    }
}

impl EncodeInto for UnwatchRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(108);
    }
}

impl EncodeInto for ResolveIpReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(121);
    }
}

impl EncodeInto for ResolveIpRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(122);
    }
}

pub fn encode_into<T: EncodeInto>(msg: &T) -> Bytes {
    return msg.encode_into()
}

pub fn encode(boxed_msg: &BoxedMsg) -> Bytes {
    match boxed_msg {
        BoxedMsg::PingReq(msg) => encode_into(msg),
        BoxedMsg::PingRep(msg) => encode_into(msg),
        BoxedMsg::PullReq(msg) => encode_into(msg),
        BoxedMsg::PullRep(msg) => encode_into(msg),
        BoxedMsg::PushReq(msg) => encode_into(msg),
        BoxedMsg::PushRep(msg) => encode_into(msg),
        BoxedMsg::DoReq(msg) => encode_into(msg),
        BoxedMsg::DoRep(msg) => encode_into(msg),
        BoxedMsg::Do2Req(msg) => encode_into(msg),
        BoxedMsg::Do2Rep(msg) => encode_into(msg),
        BoxedMsg::AuthReq(msg) => encode_into(msg),
        BoxedMsg::AuthRep(msg) => encode_into(msg),
        BoxedMsg::OkRep(msg) => encode_into(msg),
        BoxedMsg::ErrorRep(msg) => encode_into(msg),
        BoxedMsg::Ok2Rep(msg) => encode_into(msg),
        BoxedMsg::Error2Rep(msg) => encode_into(msg),
        BoxedMsg::RegisterFrontendReq(msg) => encode_into(msg),
        BoxedMsg::RegisterFrontendRep(msg) => encode_into(msg),
        BoxedMsg::AddRouteReq(msg) => encode_into(msg),
        BoxedMsg::AddRouteRep(msg) => encode_into(msg),
        BoxedMsg::DeleteRouteReq(msg) => encode_into(msg),
        BoxedMsg::DeleteRouteRep(msg) => encode_into(msg),
        BoxedMsg::AddRouteMsg(msg) => encode_into(msg),
        BoxedMsg::DeleteRouteMsg(msg) => encode_into(msg),
        BoxedMsg::PushRoutesReq(msg) => encode_into(msg),
        BoxedMsg::PushRoutesRep(msg) => encode_into(msg),
        BoxedMsg::PullRoutesReq(msg) => encode_into(msg),
        BoxedMsg::PullRoutesRep(msg) => encode_into(msg),
        BoxedMsg::RegisterBackendReq(msg) => encode_into(msg),
        BoxedMsg::RegisterBackendRep(msg) => encode_into(msg),
        BoxedMsg::DeleteTopicsReq(msg) => encode_into(msg),
        BoxedMsg::DeleteTopicsRep(msg) => encode_into(msg),
        BoxedMsg::ResolveFrontendReq(msg) => encode_into(msg),
        BoxedMsg::ResolveFrontendRep(msg) => encode_into(msg),
        BoxedMsg::ResolveBackendReq(msg) => encode_into(msg),
        BoxedMsg::ResolveBackendRep(msg) => encode_into(msg),
        BoxedMsg::WatchReq(msg) => encode_into(msg),
        BoxedMsg::WatchRep(msg) => encode_into(msg),
        BoxedMsg::UnwatchReq(msg) => encode_into(msg),
        BoxedMsg::UnwatchRep(msg) => encode_into(msg),
        BoxedMsg::ResolveIpReq(msg) => encode_into(msg),
        BoxedMsg::ResolveIpRep(msg) => encode_into(msg),
    }
}

pub fn decode(bytes: &Bytes) -> Result<BoxedMsg, DecodeError> {
    let msg_type = bytes[0] as i8;
    let msg_bytes = bytes.slice(1..);
    if msg_type == 1 {
        let res: Result<PingReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::PingReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 2 {
        let res: Result<PingRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::PingRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 3 {
        let res: Result<PullReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::PullReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 4 {
        let res: Result<PullRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::PullRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 5 {
        let res: Result<PushReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::PushReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 6 {
        let res: Result<PushRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::PushRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 7 {
        let res: Result<DoReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::DoReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 8 {
        let res: Result<DoRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::DoRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 9 {
        let res: Result<Do2Req, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::Do2Req(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 10 {
        let res: Result<Do2Rep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::Do2Rep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 27 {
        let res: Result<AuthReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::AuthReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 28 {
        let res: Result<AuthRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::AuthRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 29 {
        let res: Result<OkRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::OkRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 30 {
        let res: Result<ErrorRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::ErrorRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 31 {
        let res: Result<Ok2Rep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::Ok2Rep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 32 {
        let res: Result<Error2Rep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::Error2Rep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 65 {
        let res: Result<RegisterFrontendReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::RegisterFrontendReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 66 {
        let res: Result<RegisterFrontendRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::RegisterFrontendRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 67 {
        let res: Result<AddRouteReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::AddRouteReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 68 {
        let res: Result<AddRouteRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::AddRouteRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 69 {
        let res: Result<DeleteRouteReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::DeleteRouteReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 70 {
        let res: Result<DeleteRouteRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::DeleteRouteRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 71 {
        let res: Result<AddRouteMsg, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::AddRouteMsg(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 72 {
        let res: Result<DeleteRouteMsg, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::DeleteRouteMsg(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 73 {
        let res: Result<PushRoutesReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::PushRoutesReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 74 {
        let res: Result<PushRoutesRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::PushRoutesRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 75 {
        let res: Result<PullRoutesReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::PullRoutesReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 76 {
        let res: Result<PullRoutesRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::PullRoutesRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 81 {
        let res: Result<RegisterBackendReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::RegisterBackendReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 82 {
        let res: Result<RegisterBackendRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::RegisterBackendRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 83 {
        let res: Result<DeleteTopicsReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::DeleteTopicsReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 84 {
        let res: Result<DeleteTopicsRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::DeleteTopicsRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 97 {
        let res: Result<ResolveFrontendReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::ResolveFrontendReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 98 {
        let res: Result<ResolveFrontendRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::ResolveFrontendRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 99 {
        let res: Result<ResolveBackendReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::ResolveBackendReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 100 {
        let res: Result<ResolveBackendRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::ResolveBackendRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 105 {
        let res: Result<WatchReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::WatchReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 106 {
        let res: Result<WatchRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::WatchRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 107 {
        let res: Result<UnwatchReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::UnwatchReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 108 {
        let res: Result<UnwatchRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::UnwatchRep(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 121 {
        let res: Result<ResolveIpReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::ResolveIpReq(msg)),
            Err(err) => Err(err),
        }
    } else if msg_type == 122 {
        let res: Result<ResolveIpRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::ResolveIpRep(msg)),
            Err(err) => Err(err),
        }
    } else {
        Err(DecodeError::new(format!("Invalid msg type: {}", msg_type)))
    }
}

pub fn set_round_ref(boxed_msg: &mut BoxedMsg, round_ref: u32) -> &BoxedMsg {
    match boxed_msg {
        BoxedMsg::PingReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::PingRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::PullReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::PullRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::PushReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::PushRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::DoReq(msg) => msg.traces[0].r#ref = round_ref,
        BoxedMsg::DoRep(msg) => msg.traces[0].r#ref = round_ref,
        BoxedMsg::Do2Req(msg) => msg.traces[0].r#ref = round_ref,
        BoxedMsg::Do2Rep(msg) => msg.traces[0].r#ref = round_ref,
        BoxedMsg::AuthReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::AuthRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::OkRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::ErrorRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::Ok2Rep(msg) => msg.traces[0].r#ref = round_ref,
        BoxedMsg::Error2Rep(msg) => msg.traces[0].r#ref = round_ref,
        BoxedMsg::RegisterFrontendReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::RegisterFrontendRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::AddRouteReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::AddRouteRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::DeleteRouteReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::DeleteRouteRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::AddRouteMsg(msg) => msg.r#ref = round_ref,
        BoxedMsg::DeleteRouteMsg(msg) => msg.r#ref = round_ref,
        BoxedMsg::PushRoutesReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::PushRoutesRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::PullRoutesReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::PullRoutesRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::RegisterBackendReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::RegisterBackendRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::DeleteTopicsReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::DeleteTopicsRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::ResolveFrontendReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::ResolveFrontendRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::ResolveBackendReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::ResolveBackendRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::WatchReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::WatchRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::UnwatchReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::UnwatchRep(msg) => msg.r#ref = round_ref,
        BoxedMsg::ResolveIpReq(msg) => msg.r#ref = round_ref,
        BoxedMsg::ResolveIpRep(msg) => msg.r#ref = round_ref,
    }
    boxed_msg
}

pub fn get_round_ref(boxed_msg: &BoxedMsg) -> u32 {
    match boxed_msg {
        BoxedMsg::PingReq(msg) => msg.r#ref,
        BoxedMsg::PingRep(msg) => msg.r#ref,
        BoxedMsg::PullReq(msg) => msg.r#ref,
        BoxedMsg::PullRep(msg) => msg.r#ref,
        BoxedMsg::PushReq(msg) => msg.r#ref,
        BoxedMsg::PushRep(msg) => msg.r#ref,
        BoxedMsg::DoReq(msg) => msg.traces[0].r#ref,
        BoxedMsg::DoRep(msg) => msg.traces[0].r#ref,
        BoxedMsg::Do2Req(msg) => msg.traces[0].r#ref,
        BoxedMsg::Do2Rep(msg) => msg.traces[0].r#ref,
        BoxedMsg::AuthReq(msg) => msg.r#ref,
        BoxedMsg::AuthRep(msg) => msg.r#ref,
        BoxedMsg::OkRep(msg) => msg.r#ref,
        BoxedMsg::ErrorRep(msg) => msg.r#ref,
        BoxedMsg::Ok2Rep(msg) => msg.traces[0].r#ref,
        BoxedMsg::Error2Rep(msg) => msg.traces[0].r#ref,
        BoxedMsg::RegisterFrontendReq(msg) => msg.r#ref,
        BoxedMsg::RegisterFrontendRep(msg) => msg.r#ref,
        BoxedMsg::AddRouteReq(msg) => msg.r#ref,
        BoxedMsg::AddRouteRep(msg) => msg.r#ref,
        BoxedMsg::DeleteRouteReq(msg) => msg.r#ref,
        BoxedMsg::DeleteRouteRep(msg) => msg.r#ref,
        BoxedMsg::AddRouteMsg(msg) => msg.r#ref,
        BoxedMsg::DeleteRouteMsg(msg) => msg.r#ref,
        BoxedMsg::PushRoutesReq(msg) => msg.r#ref,
        BoxedMsg::PushRoutesRep(msg) => msg.r#ref,
        BoxedMsg::PullRoutesReq(msg) => msg.r#ref,
        BoxedMsg::PullRoutesRep(msg) => msg.r#ref,
        BoxedMsg::RegisterBackendReq(msg) => msg.r#ref,
        BoxedMsg::RegisterBackendRep(msg) => msg.r#ref,
        BoxedMsg::DeleteTopicsReq(msg) => msg.r#ref,
        BoxedMsg::DeleteTopicsRep(msg) => msg.r#ref,
        BoxedMsg::ResolveFrontendReq(msg) => msg.r#ref,
        BoxedMsg::ResolveFrontendRep(msg) => msg.r#ref,
        BoxedMsg::ResolveBackendReq(msg) => msg.r#ref,
        BoxedMsg::ResolveBackendRep(msg) => msg.r#ref,
        BoxedMsg::WatchReq(msg) => msg.r#ref,
        BoxedMsg::WatchRep(msg) => msg.r#ref,
        BoxedMsg::UnwatchReq(msg) => msg.r#ref,
        BoxedMsg::UnwatchRep(msg) => msg.r#ref,
        BoxedMsg::ResolveIpReq(msg) => msg.r#ref,
        BoxedMsg::ResolveIpRep(msg) => msg.r#ref,
    }
}