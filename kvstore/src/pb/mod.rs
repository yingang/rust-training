mod abi;

use std::convert::TryFrom;

pub use abi::*;
use bytes::{Bytes, BytesMut};
//use prost::bytes::BufMut;
use prost::Message;

use self::request::Command;

impl TryFrom<BytesMut> for Request {
    type Error = prost::DecodeError;

    fn try_from(buf: BytesMut) -> Result<Self, Self::Error> {
        Message::decode(buf) // 为什么不需要使用abi.rs里定义的类型来decode？
    }
}

impl Request {
    pub fn new_put(key: String, value: Vec<u8>) -> Self {
        Self {
            command: Some(Command::Put(
                RequestPut { key, value }
            )),
        }
    }

    pub fn new_get(key: String) -> Self {
        Self {
            command: Some(Command::Get(
                RequestGet { key }
            )),
        }
    }
}

impl TryFrom<BytesMut> for Response {
    type Error = prost::DecodeError;

    fn try_from(buf: BytesMut) -> Result<Self, Self::Error> {
        Message::decode(buf)
    }
}

impl Response {
    pub fn new(key: String, value: Vec<u8>) -> Self {
        Self {
            code: 0,
            key: key,
            value: value,
        }
    }

    pub fn not_found(key: String) -> Self {
        Self {
            code: 404,
            key: key,
            ..Default::default() // 这是啥写法？
        }
    }
}

// 目前还不支持类似impl From<T: prost::Message> for Bytes的写法！
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