use std::ops::Deref;

use prost::Message;

include!(concat!(env!("OUT_DIR"), "/hello.rs"));

include!(concat!(env!("OUT_DIR"), "/hello.serde.rs"));

fn main() {
    let bytes = std::fs::read("test.bin").unwrap();
    let idk = Greeting::decode(bytes.deref()).unwrap();

    let json = std::fs::read_to_string("test.json").unwrap();
    serde_json::from_str::<Greeting>(&json).unwrap();
    dbg!(idk);
}
