use anyhow::Result;
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt};
use serde_json::Value;

pub async fn send_unix_request(socket: &str, req: &str) -> Result<Value> {
    let mut stream = UnixStream::connect(socket).await?;
    stream.write_all(req.as_bytes()).await?;
    stream.write_all(b"\n").await?;
    let (r, _) = stream.into_split();
    let mut reader = BufReader::new(r).lines();
    if let Some(line) = reader.next_line().await? {
        let v: Value = serde_json::from_str(&line)?;
        Ok(v)
    } else {
        anyhow::bail!("no response");
    }
}
