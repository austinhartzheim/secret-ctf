
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};
use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_service::Service;
use futures::{future, Future, BoxFuture};
use tokio_proto::TcpServer;

/// A simple codec that accepts any message. (We don't need to actually extract a message
/// from a UDP packet to consider it a knock, so any message will suffice.)
struct KnockCodec;

impl Decoder for KnockCodec {
    type Item = String; // TODO: check if using a string will only accept valid UTF8 strings
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        let buffer_size = buf.len();
        if buffer_size == 0 {
            return Ok(None)
        }

        let full = buf.split_to(buffer_size);
        Ok(Some("got message".to_string()))
    }
}

impl Encoder for KnockCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        Ok(())
    }
}

/// A simple protocol that only listens for one message. (We don't need anything else because
/// a port knock will be received with any singular message and there will be no reply.)
struct KnockProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for KnockProto {
    type Request = String;
    type Response = String; // TODO: do we need to respond?
    type Transport = Framed<T, KnockCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(KnockCodec))
    }
}

struct KnockService;

impl Service for KnockService {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}

fn main() {
    let addr = "0.0.0.0:12345".parse().unwrap();
    let server = TcpServer::new(KnockProto, addr);
    server.serve(|| Ok(KnockService));
}
