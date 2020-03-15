use std::io::prelude::*;
use std::net::TcpStream;
use ssh2::Session;

fn main() {
        //println!("list identities");
    //for id in agent.identities().unwrap() {
        //println!("{}", id.comment());
    //}

    let conn = TcpStream::connect("111.231.79.169:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(conn);
    sess.handshake().unwrap();
    sess.userauth_agent("root").unwrap();
    let mut agent = sess.agent().unwrap();
    agent.connect().unwrap();
    agent.list_identities().unwrap();
    for id in agent.identities().unwrap() {
        println!("{}", id.comment());
    }

    let authed = sess.authenticated();
    println!("{}", authed);

    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("stdout is {}", s);
    channel.wait_close();
    println!("result is {}", channel.exit_status().unwrap());
}
