use std::fs;

use bitcask::{db, options::Options};
use bytes::Bytes;

fn main() {
    let opts = Options::default();
    let engine = db::Engine::open(opts).expect("failed to open bitcask engine");

    let res1 = engine.put(Bytes::from("name"), Bytes::from("bitcask"));
    assert!(res1.is_ok());

    let res2 = engine.get(Bytes::from("name"));
    assert!(res2.is_ok());
    let val2 = res2.unwrap();
    assert_eq!(Bytes::from("bitcask"), val2);

    let res3 = engine.delete(Bytes::from("name"));
    assert!(res3.is_ok());

    let res3 = engine.delete(Bytes::from("value"));
    assert!(res3.is_ok());

    fs::remove_dir_all(std::env::temp_dir().join("bitcask")).expect("failed to remove path");
}
