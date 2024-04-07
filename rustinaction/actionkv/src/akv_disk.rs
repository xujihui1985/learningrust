use std::collections::HashMap;

use libactionkv::ActionKV;

#[allow(dead_code)]
#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
USAGE:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

type ByteStr = [u8];
type ByteString = Vec<u8>;

fn store_index_on_disk(a: &mut ActionKV, index_key: &ByteStr) {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index).unwrap();
    a.index = HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    const INDEX_KEY: &ByteStr = b"+index";

    let args = std::env::args().collect::<Vec<_>>();
    let fname = args.get(1).expect(USAGE);
    let action = args.get(2).expect(USAGE);
    let key = args.get(3).expect(USAGE);
    let maybe_value = args.get(4);

    let path = std::path::Path::new(fname);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match action.as_str() {
        "get" => {
            let index_as_bytes = store.get(&INDEX_KEY).unwrap().unwrap();
            let index: HashMap<ByteString, u64> = bincode::deserialize(&index_as_bytes).unwrap();
            match index.get(key.as_bytes()) {
                None => eprint!("{:?} not found", key),
                Some(&i) => {
                    let kv = store.get_at(i).unwrap();
                    println!("{}", String::from_utf8(kv.value).unwrap());
                }
            }
        },
        "insert" => {
            let value = maybe_value.expect(USAGE).as_bytes();
            store.insert(key.as_bytes(), value).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        }
        _ => panic!("invalid action"),
    }
}
