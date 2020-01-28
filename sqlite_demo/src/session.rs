use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::model::UserBase;

pub struct Session {
    ub: UserBase,
    data: Arc<Mutex<HashMap<u64, String>>>,
}

impl Session {
    pub fn new(dbfile: &str) -> Self {
        Session {
            ub: UserBase::new(dbfile),
            data: Arc::new(Mutex::new(HashMap::<u64, String>::new())),
        }
    }

    pub fn add_session(&self, uname: String) -> u64 {
        let mut mp = self.data.lock().unwrap();
        let mut r = rand::random();
        while let Some(_) = mp.get(&r) {
            r = rand::random();
        }
        mp.insert(r, uname);
        r
    }

    pub fn get_session(&self, id: u64) -> Option<String> {
        let mp = self.data.lock().unwrap();
        match mp.get(&id) {
            Some(n) => Some(n.to_owned()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_session() {
        let s = Session::new("");
        let id = s.add_session("sean".to_string());
        assert_eq!(s.get_session(id).unwrap(), "sean");
    }

}
