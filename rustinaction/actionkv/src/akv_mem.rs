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

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let fname = args.get(1).expect(USAGE);
    let action = args.get(2).expect(USAGE);
    let key = args.get(3).expect(USAGE);
    let maybe_value = args.get(4);

    let path = std::path::Path::new(fname);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match action.as_str() {
        "get" => match store.get(key.as_bytes()).unwrap() {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", String::from_utf8(value).unwrap()),
        },
        "insert" => {
            let value = maybe_value.expect(USAGE).as_bytes();
            store.insert(key.as_bytes(), value).unwrap();
        }
        _ => panic!("invalid action"),
    }
}
