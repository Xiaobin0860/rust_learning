mod pb;
use std::sync::Arc;

use dashmap::DashMap;
use pb::{request::*, *};

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use std::convert::TryInto;
use tokio::net::TcpListener;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;
use tracing::info;

#[derive(Debug)]
struct ServerState {
    store: DashMap<String, Vec<u8>>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerState {
    pub(crate) fn new() -> Self {
        Self {
            store: DashMap::new(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let state = Arc::new(ServerState::new());
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(addr).await?;

    info!("Listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("New client {:?} accepted", addr);

        let shared = state.clone();

        tokio::spawn(async move {
            let mut stream = LengthDelimitedCodec::builder()
                .length_field_length(2)
                .new_framed(stream);

            while let Some(Ok(buf)) = stream.next().await {
                let msg: Request = buf.try_into()?;
                info!("Got a command: {:?}", msg);
                let response = match msg.command {
                    Some(Command::Get(RequestGet { key })) => match shared.store.get(&key) {
                        Some(v) => Response::new(key, v.value().to_vec()),
                        None => Response::not_found(key),
                    },
                    Some(Command::Put(RequestPut { key, value })) => {
                        shared.store.insert(key.clone(), value.clone());
                        Response::new(key, value)
                    }
                    Some(Command::Del(RequestDel { key })) => match shared.store.remove(&key) {
                        Some((k, v)) => Response::new(k, v),
                        None => Response::not_found(key),
                    },
                    None => Response::not_impl(),
                };
                stream.send(response.into()).await?;
            }
            Ok::<(), anyhow::Error>(())
        });
    }
}
