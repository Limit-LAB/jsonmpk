use rmp::encode::{buffer::ByteBuf, *};
use serde_json::Value;
use serde_json::Value::*;



pub trait ToRmp {
  #[allow(clippy::result_unit_err)]
  fn to_rmp(&self) -> Result<Vec<u8>, ()>;
}

impl ToRmp for Value {
  fn to_rmp(&self) -> Result<Vec<u8>, ()> { to_rmp(self) }
}


#[allow(clippy::result_unit_err)]
pub fn to_rmp(this: &Value) -> Result<Vec<u8>, ()> {
  let mut buf = ByteBuf::new();
  write_value(&mut buf, this).map_err(|_e| ())?;
  Ok(buf.into_vec())
}

#[allow(clippy::result_unit_err)]
pub fn write_value<W: RmpWrite>(wr: &mut W, val: &Value) -> Result<(), ()> {
  match val {
    Null => write_nil(wr).map_err(|_e| ()),
    Bool(v) => write_bool(wr, *v).map_err(|_e| ()),
    Number(v) => {
      if v.is_i64() {
        write_sint(wr, v.as_i64().unwrap()).map_err(|_e| ())?;
      } else if v.is_u64() {
        write_uint(wr, v.as_u64().unwrap()).map_err(|_e| ())?;
      } else if v.is_f64() {
        write_f64(wr, v.as_f64().unwrap()).map_err(|_e| ())?;
      } else {
        unreachable!();
      }
      Ok(())
    }
    String(v) => write_str(wr, v).map_err(|_e| ()),
    Array(v) => {
      write_array_len(wr, v.len().try_into().unwrap()).map_err(|_e| ())?;
      for i in v {
        write_value(wr, i).map_err(|_e| ())?;
      }
      Ok(())
    }
    Object(v) => {
      write_map_len(wr, v.len().try_into().unwrap()).map_err(|_e| ())?;
      for (k, v) in v.iter() {
        write_str(wr, k).map_err(|_e| ())?;
        write_value(wr, v).map_err(|_e| ())?;
      }
      Ok(())
    }
  }
}
