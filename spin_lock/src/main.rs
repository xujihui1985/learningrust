use std::thread;

mod channel;
mod oneshot;
mod oneshot_v2;
mod spinlock;
mod arc;
mod arc_weak;

fn main() {
    let mut c = oneshot_v2::Channel::new();
    thread::scope(|s| {
        let (sender, receiver) = c.split();
        s.spawn(move || {
            sender.send("hello world");
        });
        //s.spawn(move || {
            // println!("receive {}", receiver.receive());
        //});
        println!("receive {}", receiver.receive());
    });
}
