use std::fmt::Debug;

use serde::{Serialize, de::DeserializeOwned};
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;
use rmp::Marker;

use jsonmpk::*;



#[derive(Debug, Default, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Foo {
  age: u8,
  num: i32,
  name: String,
  access: bool,
}


#[derive(Debug, Default, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Bar {
  age: u8,
  aba: Vec<u16>,
  abcde: Option<bool>,
}


#[test]
fn test() {
  let test_cases = [
    Foo::default(),
    Foo {
        age: 11,
        num: 114514,
        name: "1145141919810".to_string(),
        access: true,
    }
  ];
  for i in test_cases {
    rmp2json_once(&i);
    json2rmp_once(&i);
  }

  rmp2json_once(&Bar::default());
  json2rmp_once(&Bar::default());
}

fn rmp2json_once<T: Debug + Serialize + DeserializeOwned + PartialEq>(foo: &T) {
  let foo = rmp_serde::to_vec_named(foo).unwrap();
  for i in foo.iter() {
    print!("{:#08b}\t", i);
    print!("{:?}\t", *i as char);
    print!("{:?}\t", Marker::from_u8(*i));
    println!("");
  }
  println!("------------------");

  let foo1: T = rmp_serde::from_slice(&foo).unwrap();
  let foo2: T = serde_json::from_value(dbg!(Value::from_rmp(&foo).unwrap())).unwrap();

  assert_eq!(foo1, foo2);
}



fn json2rmp_once<T: Debug + PartialEq + Serialize + DeserializeOwned>(foo: &T) {
  let foo1 = rmp_serde::to_vec_named(foo).unwrap();
  let foo2 = serde_json::to_value(foo).unwrap().to_rmp().unwrap();

  let foo1: T = rmp_serde::from_slice(&foo1).unwrap();
  let foo2: T = rmp_serde::from_slice(&foo2).unwrap();

  assert_eq!(foo1, foo2);
}