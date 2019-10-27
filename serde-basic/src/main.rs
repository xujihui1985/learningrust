extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    workers: u64,
    ignore: bool,
    auth_server: Option<String>,
}

fn main() {
    let config = ServerConfig{
        workers: 100,
        ignore: false,
        auth_server: Some("serveraddress".to_string()),
    };
    let json_str = serde_json::to_string(&config).unwrap();
    println!("{}", json_str);
    let deserialized: ServerConfig = serde_json::from_str(&json_str).unwrap();
    println!("{:?}", deserialized);
}
