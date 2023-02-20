#![no_main]

use libfuzzer_sys::fuzz_target;

use std::{fmt::Debug, collections::HashMap};

use jsonmpk::FromRmp;
// use serde::{de::DeserializeOwned, Serialize};
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

fuzz_target!(|input: &[u8]| {
    // decode raw date test
    let foo1: Result<Gee, ()> = rmp_serde::from_slice(input).map_err(|_e| ());
    let foo2: Result<Gee, ()> = Value::from_rmp(input).map_or(Err(()), |v| serde_json::from_value(v).map_err(|_e| ()) );
    assert_eq!(foo1, foo2);
});