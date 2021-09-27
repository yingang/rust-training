mod pb;

use anyhow::Result;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt}; // for stream.next() to work
use std::{convert::TryInto, sync::Arc};
use tokio::net::TcpListener;
use tokio_util::codec::LengthDelimitedCodec;
use tracing::info;

use crate::pb::{request::Command, Request, RequestGet, RequestPut, Response};

struct ServerState {
    store: DashMap<String, Vec<u8>>,
}

impl ServerState {
    // 类型写Self也可以
    fn new() -> ServerState {
        ServerState {
            store: DashMap::new(),
        }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            store: DashMap::new(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("hello tracing!");

    let state = Arc::new(ServerState::new());
    let addr = "0.0.0.0:8188";
    let listener = TcpListener::bind(addr).await?;

    info!("listening to {:?}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("new client: {:?} accepted", addr);

        let state = state.clone();
        tokio::spawn(async move {
            let mut stream = LengthDelimitedCodec::builder()
                .length_field_length(2)
                .new_framed(stream);

            while let Some(Ok(buf)) = stream.next().await {
                let msg: Request = buf.try_into()?; // try_info()依赖pb mod里实现的TryFrom<BytesMut>
                info!("got a command: {:?}", msg);
                let response = match msg.command {
                    Some(Command::Get(RequestGet { key })) => {
                        match state.store.get(&key) {
                            Some(value) => Response::new(key, value.value().to_vec()),
                            None => Response::not_found(key),
                        }
                    },
                    Some(Command::Put(RequestPut { key, value })) => {
                        state.store.insert(key.clone(), value.clone());
                        Response::new(key, value)
                    },
                    _ => unimplemented!(),
                };
                stream.send(response.into()).await?
            }
            Ok::<(), anyhow::Error>(()) // 通过这句指定了async block的返回值，这样前面try_info()后面才能用?
        });
    }
}
