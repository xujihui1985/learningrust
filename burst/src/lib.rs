use std::collections::HashMap;
use std::io;

pub struct SSHConnection {}

pub struct Machine {
    ssh: SSHConnection,
    instance_type: String,
    id: String,
    dns: String,
}

pub struct MachineSetup {
    instance_type: String,
    ami: String,
    setup: Box<dyn Fn(&mut SSHConnection) -> io::Result<()>>,
}

impl MachineSetup {
    pub fn new<F>(instance_type: &str, ami: &str, setup: F) -> Self
        where F: Fn(&mut SSHConnection) -> io::Result<()> + 'static 
        // static here means F should not borrow variable from short lifetime
        // because we take F as a parameter and return a variable that borrow the 
        // Fn, so if Fn borrow some varible from the function, that will be problem
        // so we indicat that F should be 'static, which means that F should not borrow
        // some shortlife varible
    {
        MachineSetup {
            instance_type: instance_type.to_owned(),
            ami: ami.to_owned(),
            setup: Box::new(setup),
        }
    }
}

struct BurstBuilder {
    descriptors: HashMap<String, (MachineSetup, u32)>,
}

impl Default for BurstBuilder {
    fn default() -> Self {
        BurstBuilder {
            descriptors: Default::default(),
        }
    }
}

impl BurstBuilder {
    pub fn add_set(&mut self, name: &str, number: u32, setup: MachineSetup) {
        self.descriptors.insert(name.to_owned(), (setup, number));
    }

    pub fn run<F>(&self, f: F)
    where
        F: FnOnce(HashMap<String, &mut [Machine]>) -> io::Result<()>,
    {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut b = BurstBuilder::default();
        b.add_set(
            "server",
            1,
            MachineSetup::new("t2.micro", "ami-e1111", |ssh| {
                Ok(())
            }),
        );
        b.add_set(
            "client",
            2,
            MachineSetup::new("t2.micro", "ami-e1111", |ssh| {
                Ok(())
            }),
        );
        b.run(|vms: HashMap<String, &mut [Machine]>| {
            Ok(())
        });
    }
}
