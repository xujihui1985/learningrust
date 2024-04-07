mod intermut;

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn derive(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

impl CubeSat {
    fn new(id: u64) -> Self {
        Self {
            id,
        }
    }

    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.derive(self)
    }
}


struct GroundStation;

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat::new(sat_id)
    }

    fn send(
        &self,
        mailbox: &mut Mailbox,
        msg: Message,
    ) {
        mailbox.post(msg)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1,2,3]
}

fn main() {
    intermut::borrow_mut();
    //let base = GroundStation{};
    //let mut mailbox = Mailbox{messages: vec![]};
    //let sat_ids = fetch_sat_ids();

    //for id in sat_ids {
        //let sat = base.connect(id);
        //base.send(&mut mailbox, Message{to: id, content: "hi there".into()});
        //let m = sat.recv(&mut mailbox);
        //println!("msg {:?}", m.unwrap());
    //}
}
