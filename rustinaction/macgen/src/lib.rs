#![allow(dead_code)]
use std::fmt::Display;
use rand::RngCore;


#[derive(Debug)]
struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let octet = &self.0;
        write!(
            f, 
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octet[0],
            octet[1],
            octet[2],
            octet[3],
            octet[4],
            octet[5],
        )
    }
}

impl MacAddress {
    pub fn new() -> Self {
        let mut octets = [0_u8; 6];
        rand::thread_rng().fill_bytes(&mut octets);
        octets[0] |= 0b_0000_0011;
        MacAddress(octets)
    }

    pub fn is_local(&self) -> bool {
        (self.0[0] & 0b_0000_0010) == 0b_0000_0010
    }

    pub fn is_unicast(&self) -> bool {
        (self.0[0] & 0b_0000_0001) == 0b_0000_0001
    }
}

#[cfg(test)]
mod tests {
    use crate::MacAddress;

    #[test]
    fn it_works() {
        let mac = MacAddress::new();
        assert_eq!(true, mac.is_unicast());
        assert_eq!(true, mac.is_local());
    }
}
