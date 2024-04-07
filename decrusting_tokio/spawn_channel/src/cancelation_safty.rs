#[tokio::main]
async fn main() {

    let mut tcp = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    loop {
        tokio::select! {
            v = read_string_of_json(&mut tcp) => {
                println!("{}", v);
                break;
            }
            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                println!("wait");
            }

        }
    }


}
 
async fn read_string_of_json(tcp: &mut TcpStream) -> String {
    let mut s = String::new();
    loop {
        let mut buf = [0u8; 1024];
        let n = tcp.read(&mut buf).await.unwrap();
        s.push_str(std::str::from_utf8(&buf[..n]).unwrap());
        if s.len() > 100 {
            return s
        }
    }
}
