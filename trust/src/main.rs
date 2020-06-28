use tun_tap::{Iface, Mode};
use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};
use std::io;
use std::collections::HashMap;
use std::net::Ipv4Addr;
mod tcp;


#[derive(Clone,Copy,Debug,Hash,Eq,PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, tcp::Connection> = Default::default();
    let mut nic = Iface::without_packet_info("tun0", Mode::Tun)?;
    let mut buf = [0u8; 1504];

    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        // let flags = u16::from_be_bytes([buf[0], buf[1]]);
        // let proto = u16::from_be_bytes([buf[2], buf[3]]);
        // ipv4
        // if proto != 0x0800 {
          //   continue;
        // }

        match Ipv4HeaderSlice::from_slice(&buf[..nbytes]) {
            Ok(iph) => {
                let src = iph.source_addr();
                let des = iph.destination_addr();
                let proto = iph.protocol();
                if proto != 0x06 {
                    continue;
                }

                let ip_head_size = iph.slice().len();
                match TcpHeaderSlice::from_slice(&buf[iph.slice().len()..nbytes]) {
                    Ok(tcph) => {
                        use std::collections::hash_map::Entry;
                        let tcp_head_size = tcph.slice().len();
                        let data_offset = ip_head_size + tcp_head_size;
                        match connections
                            .entry(Quad {
                                src: (src, tcph.source_port()),
                                dst: (des, tcph.destination_port()),
                            }) {
                                Entry::Occupied(mut c) => {
                                    c.get_mut().on_packet(&mut nic, iph, tcph, &buf[data_offset..nbytes])?;
                                },
                                Entry::Vacant(mut e) => {
                                    if let Some(c) = tcp::Connection::accept(&mut nic, iph, tcph, &buf[data_offset..nbytes])? {
                                        e.insert(c);
                                    }
                                }
                            }
                    },

                    Err(e) => {
                        eprintln!("ignore unknown error {:?}", e);
                    }
                }
            },
            Err(e) => {
                eprintln!("ignore error {:?}", e);
            }
        }
    }
    Ok(())
}

