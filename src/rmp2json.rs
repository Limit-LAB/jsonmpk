use std::fmt::Debug;

use rmp::decode::{bytes::Bytes, *};
use rmp::Marker::*;
use serde_json::{Map, Number, Value};


pub trait FromRmp {
  type Ret;
  #[allow(clippy::result_unit_err)]
  fn from_rmp(rd: &[u8]) -> Result<Self::Ret, ()>;
}

impl FromRmp for Value {
  type Ret = Self;

  fn from_rmp(rd: &[u8]) -> Result<Self::Ret, ()> { from_rmp(rd) }
}


#[allow(clippy::result_unit_err)]
pub fn from_rmp(rd: &[u8]) -> Result<Value, ()> {
  let mut rd = Bytes::new(rd);
  read_value(&mut rd)
}


#[allow(clippy::result_unit_err)]
fn utils_read_str(rd: &mut Bytes) -> Result<String, ()> {
  if let Ok(len) = read_str_len(rd) {
    let str = (0..len).map(|_| rd.read_u8().unwrap()).collect::<Vec<u8>>();
    return Ok(String::from_utf8(str).unwrap());
  }
  Err(())
}

fn err_handle<T: Debug>(e: T) {
  if cfg!(test) || cfg!(debug_assertions) {
    println!("error: {e:?}");
  }
}

#[allow(clippy::result_unit_err)]
pub fn read_value(rd: &mut Bytes) -> Result<Value, ()> {
  let m = read_marker(rd).map_err(err_handle)?;
  let r = match m {
    FixPos(val) => Value::Number(Number::from(val)),
    FixNeg(val) => Value::Number(Number::from(val)),
    Null => Value::Null,
    True => Value::Bool(true),
    False => Value::Bool(false),
    U8 => Value::Number(Number::from(rd.read_data_u8().map_err(err_handle)?)),
    U16 => Value::Number(Number::from(rd.read_data_u16().map_err(err_handle)?)),
    U32 => Value::Number(Number::from(rd.read_data_u32().map_err(err_handle)?)),
    U64 => Value::Number(Number::from(rd.read_data_u64().map_err(err_handle)?)),
    I8 => Value::Number(Number::from(rd.read_data_i8().map_err(err_handle)?)),
    I16 => Value::Number(Number::from(rd.read_data_i16().map_err(err_handle)?)),
    I32 => Value::Number(Number::from(rd.read_data_i32().map_err(err_handle)?)),
    I64 => Value::Number(Number::from(rd.read_data_i64().map_err(err_handle)?)),
    F32 => Value::Number(Number::from_f64(rd.read_data_f32().map_err(err_handle)?.into()).unwrap()),
    F64 => Value::Number(Number::from_f64(rd.read_data_f64().map_err(err_handle)?).unwrap()),
    FixStr(size) => {
      let size = size as usize;
      let str = (0..size)
        .map(|_| rd.read_u8().unwrap())
        .collect::<Vec<u8>>();
      Value::String(String::from_utf8(str).unwrap())
    }
    Str8 => {
      let size = rd.read_data_u8().unwrap() as usize;
      let str = (0..size)
        .map(|_| rd.read_u8().unwrap())
        .collect::<Vec<u8>>();
      Value::String(String::from_utf8(str).unwrap())
    }
    Str16 => {
      let size = rd.read_data_u16().unwrap() as usize;
      let str = (0..size)
        .map(|_| rd.read_u8().unwrap())
        .collect::<Vec<u8>>();
      Value::String(String::from_utf8(str).unwrap())
    }
    Str32 => {
      let size = rd.read_data_u32().unwrap() as usize;
      let str = (0..size)
        .map(|_| rd.read_u8().unwrap())
        .collect::<Vec<u8>>();
      Value::String(String::from_utf8(str).unwrap())
    }
    FixArray(size) => {
      let size = size as usize;
      let arr = (0..size)
        .map(|_| read_value(rd))
        .collect::<Result<Vec<_>, _>>();
      let arr = arr.map_err(err_handle)?;
      Value::Array(arr)
    }
    Array16 => {
      let size = rd.read_data_u16().unwrap() as usize;
      let arr = (0..size)
        .map(|_| read_value(rd))
        .collect::<Result<Vec<_>, _>>();
      let arr = arr.map_err(err_handle)?;
      Value::Array(arr)
    }
    Array32 => {
      let size = rd.read_data_u32().unwrap() as usize;
      let arr = (0..size)
        .map(|_| read_value(rd))
        .collect::<Result<Vec<_>, _>>();
      let arr = arr.map_err(err_handle)?;
      Value::Array(arr)
    }
    FixMap(size) => {
      let size = size as usize;
      let map = (0..size)
        .map(|_| -> Result<(String, Value), ()> { Ok((utils_read_str(rd)?, read_value(rd)?)) })
        .collect::<Result<Map<String, Value>, _>>()?;
      Value::Object(map)
    }
    Map16 => {
      let size = rd.read_data_u16().unwrap() as usize;
      let map = (0..size)
        .map(|_| -> Result<(String, Value), ()> { Ok((utils_read_str(rd)?, read_value(rd)?)) })
        .collect::<Result<Map<String, Value>, _>>()?;
      Value::Object(map)
    }
    Map32 => {
      let size = rd.read_data_u32().unwrap() as usize;
      let map = (0..size)
        .map(|_| -> Result<(String, Value), ()> { Ok((utils_read_str(rd)?, read_value(rd)?)) })
        .collect::<Result<Map<String, Value>, _>>()?;
      Value::Object(map)
    }
    _ => unimplemented!(),
  };
  Ok(r)
}
