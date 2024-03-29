use bytes::{Bytes, BytesMut};
use prost::Message;
use std::convert::TryFrom;

mod abi;

pub use abi::*;

use self::request::Command;

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

impl Request {
    pub fn new_get(key: &str) -> Self {
        Self {
            command: Some(Command::Get(RequestGet {
                key: key.to_owned(),
            })),
        }
    }

    pub fn new_del(key: &str) -> Self {
        Self {
            command: Some(Command::Del(RequestDel {
                key: key.to_owned(),
            })),
        }
    }

    pub fn new_put(key: &str, value: &[u8]) -> Self {
        Self {
            command: Some(Command::Put(RequestPut {
                key: key.to_owned(),
                value: value.to_vec(),
            })),
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
