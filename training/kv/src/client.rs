mod pb;
use pb::*;
use prost::Message;

fn main() {
    let get = RequestGet {
        key: "hello".to_owned(),
    };
    let mut buf = Vec::new();
    get.encode(&mut buf).unwrap();
    println!("{:?}", buf);
}
