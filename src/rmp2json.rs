use rmp::decode::{*, bytes::Bytes};
use serde_json::{Value, Number, Map};


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

#[allow(clippy::result_unit_err)]
pub fn read_value(rd: &mut Bytes) -> Result<Value, ()> {
  if read_nil(rd).is_ok() {
    return Ok(Value::Null);
  }
  if let Ok(v) = read_bool(rd) {
    return Ok(Value::Bool(v));
  }
  if let Ok(v) = read_int::<i64, _>(rd) {
    return Ok(Value::Number(Number::from(v)))
  }
  if let Ok(v) = read_int::<u64, _>(rd) {
    return Ok(Value::Number(Number::from(v)))
  }
  if let Ok(v) = read_int::<f64, _>(rd) {
    return Ok(Value::Number(Number::from_f64(v).unwrap()))
  }
  if let Ok(v) = utils_read_str(rd) {
    return Ok(Value::String(v));
  }
  if let Ok(len) = read_array_len(rd) {
    let arr = (0..len).map(|_| read_value(rd)).collect::<Result<Vec<_>, _>>();
    let arr = arr.map_err(|_e| ())?;
    return Ok(Value::Array(arr));
  }
  if let Ok(len) = read_map_len(rd) {
    let map = (0..len)
      .map(|_| -> Result<(String, Value), ()> { Ok((utils_read_str(rd)?, read_value(rd)?)) })
      .collect::<Result<Map<String, Value>, _>>()?;
    return Ok(Value::Object(map));
  }
  Err(())
}

/*
pub fn read_value(rd: &mut Bytes) -> Result<Value, ()> {
  let m = read_marker(rd).map_err(|_e| ())?;
  let r = match m {
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
      for _ in 0..size {
        read_str

      }
    },
    Str8  => todo!(),
    Str16 => todo!(),
    Str32 => todo!(),
    FixArray(size) => todo!(),
    Array16 => todo!(),
    Array32 => todo!(),
    FixMap(size) => todo!(),
    Map16 => todo!(),
    Map32 => todo!(),
    // FixPos(_) => todo!(),
    // FixNeg(_) => todo!(),
    // Bin8 => todo!(),
    // Bin16 => todo!(),
    // Bin32 => todo!(),
    // FixExt1 => todo!(),
    // FixExt2 => todo!(),
    // FixExt4 => todo!(),
    // FixExt8 => todo!(),
    // FixExt16 => todo!(),
    // Ext8 => todo!(),
    // Ext16 => todo!(),
    // Ext32 => todo!(),
    // Reserved => todo!(),
    _ => unimplemented!()
  };
  Ok(r)
}
 */