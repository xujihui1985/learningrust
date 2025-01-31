use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> MyResult<()> {
    for i in 1..20 {
        tokio::spawn(async move {
            println!("hello {}", i);
        });
    }
    std::thread::sleep(std::time::Duration::from_millis(1000));
    Ok(())
    //let bind = ":::1026";
    //let mut listener = TcpListener::bind(&bind).await?;

    //loop {
        //let (socket, _) = listener.accept().await?;
        //tokio::spawn(async move {
            //let _ = handle_connection(socket).await;
        //});
    //}
}

//async fn handle_connection(mut socket: TcpStream) -> MyResult<()> {
    //let remote_ip = socket.peer_addr()?.ip();
    //println!("remote_ip is {}", remote_ip);


    //loop {
        //let mut buf: [u8; 1024] = [0; 1024];
        //let n = socket.read(&mut buf).await?;
        //if n == 0 {
            //break;
        //}
        //let received = String::from_utf8_lossy(&buf[0..n]);
        //println!("received {}", received);
    //}

    //Ok(())
//}

fn generate() -> Vec<i32> {
    let bound = 10;
    futures::executor::block_on(
        tokio::spawn(async move {
            let mut res = vec![];
            for i in 0..bound {
                res.push(i);
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            res
        })
    ).unwrap()
}


//async fn handle_connection_by_lines(mut socket: TcpStream) -> MyResult<()> {
    //let remote_ip = socket.peer_addr()?.ip();
    //println!("remote_ip is {}", remote_ip);

    //let mut client = FramedRead::new(socket, LinesCodec::new_with_max_length(1024));

    //let query = match client.next().await {
        //Some(Ok(q)) => q,
        //_ => return Err("No query received".into()),
    //};
    //println!("receive query {}", query);
    //Ok(())
//}
