mod pb;

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use std::convert::{TryFrom};
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;
use tracing::info;

use crate::pb::{Request, Response};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    let addr = "localhost:8188";
    let stream = TcpStream::connect(addr).await?;
    let mut stream = LengthDelimitedCodec::builder()
        .length_field_length(2)
        .new_framed(stream);

    let req = Request::new_put("Hello".to_string(), b"World".to_vec());
    stream.send(req.into()).await?;
    
    let req = Request::new_get("Hello".to_string());
    stream.send(req.into()).await?;

    while let Some(Ok(buf)) = stream.next().await {
        //let msg: Response = buf.try_into()?;
        let msg = Response::try_from(buf)?;
        info!("Response: {:?}", msg);
    };

    Ok(())
}
