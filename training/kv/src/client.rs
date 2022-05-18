mod pb;
use std::convert::TryFrom;

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use pb::*;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let addr = "127.0.0.1:8888";
    let stream = TcpStream::connect(addr).await?;
    let mut stream = LengthDelimitedCodec::builder()
        .length_field_length(2)
        .new_framed(stream);

    let msg = Request::new_put("hello", b"world");
    stream.send(msg.into()).await?;

    let msg = Request::new_get("hello");
    stream.send(msg.into()).await?;

    let msg = Request::new_get("world");
    stream.send(msg.into()).await?;

    let msg = Request::new_del("hello");
    stream.send(msg.into()).await?;

    while let Some(Ok(buf)) = stream.next().await {
        let msg = Response::try_from(buf)?;
        println!("Got msg: {:?}", msg);
    }

    Ok(())
}
