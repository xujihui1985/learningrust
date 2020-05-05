use std::io;
use std::io::prelude::*;

pub enum State {
    Closed,
    Listen,
    SynRcvd,
    Estab,
}

// transmit control block
pub struct Connection {
    state: State,
    send: SendSequenceSpace,
    recv: RecvSequenceSpace,
    ip: etherparse::Ipv4Header,
}

//Send Sequence Variables

//     SND.UNA - send unacknowledged
//    SND.NXT - send next
//     SND.WND - send window
//     SND.UP  - send urgent pointer
//     SND.WL1 - segment sequence number used for last window update
//     SND.WL2 - segment acknowledgment number used for last window
//               update
//     ISS     - initial send sequence number
struct SendSequenceSpace {
    una: u32,
    nxt: u32,
    wnd: u16,
    up: bool,
    wl1: usize,
    wl2: usize,
    iss: u32,
}

struct RecvSequenceSpace {
    nxt: u32,
    wnd: u16,
    up: bool,
    irs: u32,
}

impl Connection {

    pub fn accept<'a>(
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>, 
        tcph: etherparse::TcpHeaderSlice<'a>, 
        data: &'a [u8],
    ) -> io::Result<Option<Self>> {
        let mut buf = [0u8; 1500];
        if !tcph.syn() {
            // only expect syn packet
            return Ok(None);
        }

        let iss = 0;
        let mut c = Connection{
            state: State::SynRcvd,
            send: SendSequenceSpace {
                iss,
                una: iss,
                nxt: iss + 1,
                wnd: 10,
                up: false,
                wl1: 0,
                wl2: 0,
            },
            recv: RecvSequenceSpace {
                nxt: tcph.sequence_number() + 1,
                wnd: tcph.window_size(),
                irs: tcph.sequence_number(),
                up: false,
            },
            ip: etherparse::Ipv4Header::new(
                0
                64,
                etherparse::IpTrafficClass::Tcp,
                [
                    iph.destination()[0],
                    iph.destination()[1],
                    iph.destination()[2],
                    iph.destination()[3],
                ],
                [
                    iph.source()[0],
                    iph.source()[1],
                    iph.source()[2],
                    iph.source()[3],
                ],
            ),
        };
        // start estab connection
        let mut syn_ack = etherparse::TcpHeader::new(
            tcph.destination_port(),
            tcph.source_port(),
            c.send.iss, // sequence_number, rand
            c.send.wnd, // window size
        );
        syn_ack.acknowledgment_number = c.recv.nxt;
        syn_ack.syn = true;
        syn_ack.ack = true;
        c.ip.set_payload_len(syn_ack.header_len() as usize + 0);

        let unwritten = {
            let mut unwritten = &mut buf[..];
            c.ip.write(&mut unwritten);
            syn_ack.write(&mut unwritten);
            unwritten.len()
        };
        nic.send(&buf[..buf.len() - unwritten])?;
        Ok(Some(c))
    }
    // lifetime 'a here is the lifetime of real packet
    pub fn on_packet<'a>(
        &mut self, 
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>, 
        tcph: etherparse::TcpHeaderSlice<'a>, 
        data: &'a [u8],
    ) -> io::Result<()> {
        let ackn = tcph.acknowledgment_number();
        if self.send.una < ackn {
            if self.send.nxt >= self.send.una && self.send.nxt < ackn {
                return Ok(());
            }
        }

        if !(self.send.una < tcph.acknowledgment_number() && tcph.acknowledgment_number() <= self.send.nxt) {
            return Ok(());
        }
        match self.state {
            State::SynRcvd => {
            },
            State::Estab => {

            }
        }
        Ok(())
    }
} 
