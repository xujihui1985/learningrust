#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::channel(8);
    let tcp = tokio::net::TcpStream::connect("127.0.0.1:8080").await.unwrap();

    tokio::spawn(async move {

        while let Some((byte, ack)) = rx.next().await {
            ack.send(tcp.write_all(bytes).await);
        }
    });

    {
        let tx = tx.clone();
        tokio::spawn(async move {
            loop {
                let (syn, ack) = tokio::sync::onshot::channel();
                tx.send((vec![1,2,3], syn)).await;
                let num_write = ack.await.unwrap();
            }
        });
    }

}

