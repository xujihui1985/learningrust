use std::net::TcpStream;
use std::io::{BufReader, Write, BufRead};

fn main() {
    let remote_address = "127.0.0.1:8888";
    let mut stream = TcpStream::connect(remote_address).expect("failed to connect server");
    loop {
        let mut input = String::new();
        let mut buf:Vec<u8> = Vec::new();
        std::io::stdin().read_line(&mut input).expect("failed to read from stdin");
        stream.write(input.as_bytes()).expect("failed to write to server");

        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buf).expect("could not read into buffer");
        print!("{}", std::str::from_utf8(&buf).expect("could not write buffer as string"));
    }
}
