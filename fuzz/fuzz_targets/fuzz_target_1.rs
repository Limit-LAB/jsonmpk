#![no_main]

use libfuzzer_sys::fuzz_target;

use std::{fmt::Debug, collections::HashMap};

use jsonmpk::{FromRmp, ToRmp};
use serde::{de::DeserializeOwned, Serialize};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use arbitrary::Arbitrary;

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Arbitrary)]
pub struct Foo {
    age: u8,
    num: i32,
    name: String,
    access: bool,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Arbitrary)]
pub struct Bar {
    age: u8,
    aba: Vec<u16>,
    abcde: Option<bool>,
}


#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Arbitrary)]
pub struct Gee {
    ababa: Bar,
    abcde: HashMap<String, i32>,
    jet: Foo,
}

fn rmp2json_once<T: Debug + Serialize + DeserializeOwned + PartialEq>(foo: &T) {
    let foo = rmp_serde::to_vec_named(foo).unwrap();
    let foo1: T = rmp_serde::from_slice(&foo).unwrap();
    let foo2: T = serde_json::from_value(Value::from_rmp(&foo).unwrap()).unwrap();

    assert_eq!(foo1, foo2);
}

fn json2rmp_once<T: Debug + PartialEq + Serialize + DeserializeOwned>(foo: &T) {
    let foo1 = rmp_serde::to_vec_named(foo).unwrap();
    let foo2 = serde_json::to_value(foo).unwrap().to_rmp().unwrap();

    let foo1: T = rmp_serde::from_slice(&foo1).unwrap();
    let foo2: T = rmp_serde::from_slice(&foo2).unwrap();

    assert_eq!(foo1, foo2);
}

fuzz_target!(|input: Gee| {
    // fuzzed code goes here
    rmp2json_once(&input);
    json2rmp_once(&input);
});