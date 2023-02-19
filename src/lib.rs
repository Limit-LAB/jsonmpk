pub mod json2rmp;
pub mod rmp2json;

pub use json2rmp::*;
pub use rmp2json::*;

/*
use serde::{Serializer, Deserializer, Serialize, Deserialize};
use serde_json::{Value, to_value, from_value};
use rmp_serde::{to_vec_named, from_read};
use thiserror::Error;
use std::{io::Cursor, collections::HashMap};

#[derive(Debug, Error)]
pub enum SerdeJsonRMPError {
    SerdeJsonError(serde_json::Error),
    SerdeRMPEncodeError(rmp_serde::encode::Error),
    SerdeRMPDecodeError(rmp_serde::decode::Error),
}

impl std::fmt::Display for SerdeJsonRMPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct SerdeJsonRMP;

impl SerdeJsonRMP {
    fn new() -> Self {
        SerdeJsonRMP
    }

    fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, SerdeJsonRMPError> {
        let json_value: Value = to_value(value).map_err(|e| SerdeJsonRMPError::SerdeJsonError(e))?;
        let messagepack_data = to_vec_named(&json_value).map_err(|e| SerdeJsonRMPError::SerdeRMPEncodeError(e))?;
        Ok(messagepack_data)
    }

    fn deserialize<'a, T>(&self, data: &'a [u8]) -> Result<T, SerdeJsonRMPError> where
        for<'de> T: Deserialize<'de>
    {
        let messagepack_data = from_read(Cursor::new(data)).map_err(|e| SerdeJsonRMPError::SerdeRMPDecodeError(e))?;
        let json_value: Value = from_value(messagepack_data).map_err(|e| SerdeJsonRMPError::SerdeJsonError(e))?;
        let deserialized_value: T = from_value(json_value).map_err(|e| SerdeJsonRMPError::SerdeJsonError(e))?;
        Ok(deserialized_value)
    }
}
 */
