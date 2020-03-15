use std::net::TcpStream;

fn main() {
    let conn = TcpStream::connect("127.0.0.1:22").unwrap();

}
