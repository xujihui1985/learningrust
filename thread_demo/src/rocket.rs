use std::{collections::VecDeque, sync::{Arc, Barrier, Condvar, Mutex}, thread, time::Duration};

pub struct XorShiftRng {
    state: u32,
}

impl XorShiftRng {
    fn with_seed(seed: u32) -> Self {
        XorShiftRng { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state
    }
}

struct Resources {
    lock: Mutex<(bool, bool, bool)>,
    fuel: Condvar,
    oxidizer: Condvar,
    astronauts: Condvar,
}

impl Default for Resources {
    fn default() -> Self {
        Resources {
            lock: Mutex::new((false, false, false)),
            fuel: Condvar::new(),
            oxidizer: Condvar::new(),
            astronauts: Condvar::new(),
        }
    }
}

fn producer(resources: Arc<Resources>) {
    let mut rng = XorShiftRng::with_seed(2005);
    loop {
        let mut guard = resources.lock.lock().unwrap();
        (*guard).0 = false;
        (*guard).1 = false;
        (*guard).2 = false;

        match rng.next_u32() % 3 {
            0 => {
                (*guard).0 = true;
                resources.fuel.notify_all()
            }

            1 => {
                (*guard).1 = true;
                resources.oxidizer.notify_all()
            }
            2 => {
                (*guard).2 = true;
                resources.astronauts.notify_all()
            }
            _ => unreachable!(),
        }
    }
}

fn rocket(name: String, resources: Arc<Resources>, all_go: Arc<Barrier>, lift_off: Arc<Barrier>) {
    {
        let mut guard = resources.lock.lock().unwrap();
        while !(*guard).0 {
            guard = resources.fuel.wait(guard).unwrap();
        }
        (*guard).0 = false;
        println!("{:<6} acquire fule", name);
    }
    {
        let mut guard = resources.lock.lock().unwrap();
        while !(*guard).1 {
            guard = resources.oxidizer.wait(guard).unwrap();
        }
        (*guard).1 = false;
        println!("{:<6} acquire oxidizer", name);
    }
    {
        let mut guard = resources.lock.lock().unwrap();
        while !(*guard).2 {
            guard = resources.astronauts.wait(guard).unwrap();
        }
        (*guard).2 = false;
        println!("{:<6} acquire astronauts", name);
    }

    all_go.wait();
    lift_off.wait();
    println!("{:<6} lift off", name);
}

fn main() {
    VecDeque
    let all_go = Arc::new(Barrier::new(4));
    let lift_off = Arc::new(Barrier::new(4));
    let resources = Arc::new(Resources::default());

    let mut rockets = Vec::new();
    ["KSC", "VAB", "WSMR"].iter().for_each(|e| {
        let all_go = Arc::clone(&all_go);
        let lift_off = Arc::clone(&lift_off);
        let resources = Arc::clone(&resources);
        rockets.push(thread::spawn(move || {
            rocket(e.to_string(), resources, all_go, lift_off);
        }));
    });

    thread::spawn(move || {
        producer(resources);
    });

    all_go.wait();

    let one_second = Duration::from_millis(1_000);
    println!("T-11");

    for i in 0..10 {
        println!("{:>4}", 10 - i);
        thread::sleep(one_second);
    }
    lift_off.wait();

    for jh in rockets {
        jh.join().unwrap();
    }
}
