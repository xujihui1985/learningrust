use hyper::body::HttpBody as _;
use hyper::Client;
use tokio::io::{ self, AsyncWriteExt as _};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let client = Client::new();
    let uri = "http://httpbin.org/ip".parse()?;
    let mut resp = client.get(uri).await?;
    println!("{}", resp.status());

    while let Some(chunk) = resp.body_mut().data().await {
        io::stdout().write_all(&chunk?).await?;
    }
    Ok(())
}
