use rmp::decode::{*, bytes::Bytes};
use rmp::Marker::*;
use serde_json::{Value, Number, Map};


pub trait FromRmp {
  type Ret;
  #[allow(clippy::result_unit_err)]
  fn from_rmp(rd: &[u8]) -> Result<Self::Ret, ()>;
}

impl FromRmp for Value {
  type Ret = Self;
  fn from_rmp(rd: &[u8]) -> Result<Self::Ret, ()> {
    from_rmp(rd)
  }
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


pub fn read_value(rd: &mut Bytes) -> Result<Value, ()> {
  let m = read_marker(rd).map_err(|_e| ())?;
  let r = match m {
    FixPos(val) => Value::Number(Number::from(val)),
    FixNeg(val) => Value::Number(Number::from(val)),
    Null  => Value::Null,
    True  => Value::Bool(true),
    False => Value::Bool(false),
    U8  => Value::Number(Number::from(read_u8 (rd).map_err(|_e| ())?)),
    U16 => Value::Number(Number::from(read_u16(rd).map_err(|_e| ())?)),
    U32 => Value::Number(Number::from(read_u32(rd).map_err(|_e| ())?)),
    U64 => Value::Number(Number::from(read_u64(rd).map_err(|_e| ())?)),
    I8  => Value::Number(Number::from(read_i8 (rd).map_err(|_e| ())?)),
    I16 => Value::Number(Number::from(read_i16(rd).map_err(|_e| ())?)),
    I32 => Value::Number(Number::from(read_i32(rd).map_err(|_e| ())?)),
    I64 => Value::Number(Number::from(read_i64(rd).map_err(|_e| ())?)),
    F32 => Value::Number(Number::from_f64(read_f32(rd).map_err(|_e| ())?.into()).unwrap()),
    F64 => Value::Number(Number::from_f64(read_f64(rd).map_err(|_e| ())?).unwrap()),
    FixStr(size) => {
      let size = size as usize;
      let str = (0..size).map(|_| rd.read_u8().unwrap()).collect::<Vec<u8>>();
      Value::String(String::from_utf8(str).unwrap())
    },
    Str8 => {
      let size = rd.read_data_u8().unwrap() as usize;
      let str = (0..size).map(|_| rd.read_u8().unwrap()).collect::<Vec<u8>>();
      Value::String(String::from_utf8(str).unwrap())
    },
    Str16 => {
      let size = rd.read_data_u16().unwrap() as usize;
      let str = (0..size).map(|_| rd.read_u8().unwrap()).collect::<Vec<u8>>();
      Value::String(String::from_utf8(str).unwrap())
    },
    Str32 => {
      let size = rd.read_data_u32().unwrap() as usize;
      let str = (0..size).map(|_| rd.read_u8().unwrap()).collect::<Vec<u8>>();
      Value::String(String::from_utf8(str).unwrap())
    },
    FixArray(size) => {
      let size = size as usize;
      let arr = (0..size).map(|_| read_value(rd)).collect::<Result<Vec<_>, _>>();
      let arr = arr.map_err(|_e| ())?;
      Value::Array(arr)
    },
    Array16 => {
      let size = rd.read_data_u16().unwrap() as usize;
      let arr = (0..size).map(|_| read_value(rd)).collect::<Result<Vec<_>, _>>();
      let arr = arr.map_err(|_e| ())?;
      Value::Array(arr)
    },
    Array32 => {
      let size = rd.read_data_u32().unwrap() as usize;
      let arr = (0..size).map(|_| read_value(rd)).collect::<Result<Vec<_>, _>>();
      let arr = arr.map_err(|_e| ())?;
      Value::Array(arr)
    },
    FixMap(size) => {
      let size = size as usize;
      let map = (0..size)
        .map(|_| -> Result<(String, Value), ()> { Ok((utils_read_str(rd)?, read_value(rd)?)) })
        .collect::<Result<Map<String, Value>, _>>()?;
      Value::Object(map)
    },
    Map16 => {
      let size = rd.read_data_u16().unwrap() as usize;
      let map = (0..size)
        .map(|_| -> Result<(String, Value), ()> { Ok((utils_read_str(rd)?, read_value(rd)?)) })
        .collect::<Result<Map<String, Value>, _>>()?;
      Value::Object(map)
    },
    Map32 => {
      let size = rd.read_data_u32().unwrap() as usize;
      let map = (0..size)
        .map(|_| -> Result<(String, Value), ()> { Ok((utils_read_str(rd)?, read_value(rd)?)) })
        .collect::<Result<Map<String, Value>, _>>()?;
      Value::Object(map)
    },
    _ => unimplemented!()
  };
  Ok(r)
}
