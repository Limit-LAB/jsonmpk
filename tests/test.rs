
use rmp::Marker;
use serde_derive::{Serialize, Deserialize};

use jsonmpk::*;
use serde_json::Value;


#[derive(Debug, Default, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Foo {
  age: u8,
  num: i32,
  name: String,
  access: bool,
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
}

fn rmp2json_once(foo: &Foo) {
  let foo = rmp_serde::to_vec_named(foo).unwrap();
  for i in foo.iter() {
    print!("{:#08b}\t", i);
    print!("{:?}", Marker::from_u8(*i));
    println!("");
  }
  println!("------------------");

  let foo1 = rmp_serde::from_slice::<Foo>(&foo).unwrap();
  let foo2 = serde_json::from_value::<Foo>(Value::from_rmp(&foo).unwrap()).unwrap();
  assert_eq!(foo1, foo2);
}



fn json2rmp_once(foo: &Foo) {
  let foo1 = rmp_serde::to_vec_named(foo).unwrap();
  let foo2 = serde_json::to_value(foo).unwrap().to_rmp().unwrap();

  let foo1 = rmp_serde::from_slice::<Foo>(&foo1).unwrap();
  let foo2 = rmp_serde::from_slice::<Foo>(&foo2).unwrap();
  assert_eq!(foo1, foo2);
}