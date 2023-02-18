
use serde_derive::{Serialize, Deserialize};

use jsonmpk::*;
use serde_json::Value;


#[derive(Debug, Default, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Foo {
  age: u8,
  num: i32,
  name: String,
}


#[test]
fn test() {
  rmp2json_once(&Foo::default());
  json2rmp_once(&Foo::default());
}

fn rmp2json_once(foo: &Foo) {
  let foo = rmp_serde::to_vec_named(foo).unwrap();


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