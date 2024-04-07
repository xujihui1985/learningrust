use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, PollOpt, Ready};
use mio::{Poll, Token};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::str::from_utf8;
use std::sync::Arc;
use std::time::Duration;
use std::{thread, vec};

const SERVER: mio::Token = mio::Token(0);
const WAKE_TOKEN: mio::Token = mio::Token(10);
const DATA: &[u8] = b"hello world\n";

fn main() -> io::Result<()> {
    env_logger::init();

    let mut poll = Poll::new()?;
    let mut events = mio::Events::with_capacity(128);

    // let waker = Arc::new(Waker::new(poll.registry(), WAKE_TOKEN)?);

    // let handle = thread::spawn({
    //     let waker1 = Arc::clone(&waker);
    //     move || {
    //         log::info!("wait 5s to wake up");
    //         thread::sleep(Duration::from_millis(5000));
    //         waker1.wake().expect("unable to wake");
    //         log::info!("wake from another thread");
    //     }
    // });

    let addr = "127.0.0.1:9000".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    poll.register(
        &mut server,
        SERVER,
        Ready::readable() | Ready::writable(),
        PollOpt::level(),
    )?;

    let mut connections = HashMap::new();
    let mut uniq_token = Token(SERVER.0 + 1);

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                WAKE_TOKEN => {
                    log::info!("wake event {:?}", event);
                }
                SERVER => loop {
                    let (mut conn, addr) = match server.accept() {
                        Ok((conn, addr)) => (conn, addr),
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            break;
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    };
                    log::info!("accept connection from {} token {:?}", addr, SERVER);
                    let token = next(&mut uniq_token);

                    poll.register(
                        &mut conn,
                        token,
                        Ready::readable() | Ready::writable(),
                        PollOpt::level(),
                    )?;
                    // poll.registry().register(
                    //     &mut conn,
                    //     token,
                    //     Interest::READABLE.add(Interest::WRITABLE),
                    // )?;
                    connections.insert(token, conn);
                },
                t => {
                    let done = if let Some(conn) = connections.get_mut(&t) {
                        handle_connection_event(poll.registry(), conn, event)?
                    } else {
                        false
                    };
                    if done {
                        log::info!("remove conn from connections {:?}", &t);
                        if let Some(mut conn) = connections.remove(&t) {
                            poll.registry().deregister(&mut conn)?;
                        }
                    }
                }
            }
        }
    }
}

fn handle_connection_event(
    registry: &mio::Registry,
    conn: &mut TcpStream,
    event: &Event,
) -> io::Result<bool> {
    if event.is_writable() {
        log::info!("writable event {:?}", event);
        match conn.write(DATA) {
            Ok(n) if n < DATA.len() => return Err(io::ErrorKind::WriteZero.into()),
            Ok(_) => {
                registry.reregister(conn, event.token(), Interest::READABLE)?;
            }
            Err(ref err) if would_block(err) => {}
            Err(ref err) if interrupted(err) => {
                return handle_connection_event(registry, conn, event);
            }
            Err(err) => return Err(err),
        }
    }

    if event.is_readable() {
        log::info!("readable event {:?}", event);
        let mut connection_closed = false;
        let mut received_data = vec![0; 4096];
        let mut bytes_read = 0;

        loop {
            match conn.read(&mut received_data[bytes_read..]) {
                Ok(0) => {
                    connection_closed = true;
                    break;
                }
                Ok(n) => {
                    bytes_read += n;
                    if bytes_read == received_data.len() {
                        received_data.resize(received_data.len() + 1024, 0);
                    }
                }
                Err(ref err) if would_block(err) => break,
                Err(ref err) if interrupted(err) => continue,
                Err(err) => return Err(err),
            }
        }

        if bytes_read != 0 {
            let received_data = &received_data[..bytes_read];
            if let Ok(str_buf) = from_utf8(received_data) {
                log::info!("received data {}", str_buf.trim_end());
            } else {
                log::info!("received data: {:?}", received_data);
            }
        }

        if connection_closed {
            log::info!("connection closed");
            return Ok(true);
        }
    }

    Ok(false)
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}
