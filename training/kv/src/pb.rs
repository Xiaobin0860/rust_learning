use bytes::{Bytes, BytesMut};
use prost::Message;
use std::convert::TryFrom;

mod abi;

pub use abi::*;

impl Response {
    pub fn new(key: String, value: Vec<u8>) -> Self {
        Self {
            code: 0,
            key,
            value,
        }
    }

    pub fn not_found(key: String) -> Self {
        Self {
            code: 404,
            key,
            ..Default::default()
        }
    }

    pub fn not_impl() -> Self {
        Self {
            code: 500,
            ..Default::default()
        }
    }
}

impl TryFrom<BytesMut> for Request {
    type Error = prost::DecodeError;

    fn try_from(buf: BytesMut) -> Result<Self, Self::Error> {
        Message::decode(buf)
    }
}

impl TryFrom<BytesMut> for Response {
    type Error = prost::DecodeError;

    fn try_from(buf: BytesMut) -> Result<Self, Self::Error> {
        Message::decode(buf)
    }
}

impl From<Response> for Bytes {
    fn from(msg: Response) -> Self {
        let mut buf = BytesMut::new();
        msg.encode(&mut buf).unwrap();
        buf.freeze()
    }
}

impl From<Request> for Bytes {
    fn from(msg: Request) -> Self {
        let mut buf = BytesMut::new();
        msg.encode(&mut buf).unwrap();
        buf.freeze()
    }
}
