use std::collections::HashMap;

fn main() {
    test_loan();
    let i = Inputdata{
        payload: Some(vec![])
    };

    //let x = i.payload.unwrap_or(vec![]);
    
}

fn test_loan() {
    let mut x = 123;
    let y = &x;
    x += 1;
}

//fn get_or_insert<'a>(map: &'a mut HashMap<u32, String>) -> &'a String {
//    HashMap::get(map, &22);
    //match map.get(&22) {
        //Some(v) => v,
        //None => {
            //map.insert(22, String::from("hello"));
            //&map[&22]
        //}
    //}
//}

struct Inputdata {
    payload: Option<Vec<u8>>
}

fn encrypted(_: &[u8]) -> Vec<u8> {
    vec![]
}

impl Inputdata {
    fn encrypted(&self) -> Vec<u8> {
        //let payload = &self.payload.as_ref().unwrap_or(&vec![]);
        encrypted(&self.payload.as_ref().unwrap_or(&vec![]))
    }
}
