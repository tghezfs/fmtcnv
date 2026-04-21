use std::error::Error;
use std::io::{ Error as IOError, ErrorKind };
use std::str::FromStr;

use serde_yaml_ng::Value as YamlValue;
use serde_json::Value as JsonValue;

use toml::Value as TomlValue;
use toml::value::Datetime;
use toml::map::Map;

use serde_json::Number as JsonNumber;
use serde_json::Map as JsonMap;

use crate::utils::is_iso_8601;

pub fn yaml_to_json(value: YamlValue ) -> Result<Option<JsonValue>, Box<dyn Error>> {
    match value {
        YamlValue::Null => Ok(Some(JsonValue::Null)),
        YamlValue::Bool(val) => Ok(Some(JsonValue::Bool(val))),
        YamlValue::Number(val) => {
            if let Some(val_i64) = val.as_i64() {
                Ok(Some(JsonValue::Number(JsonNumber::from(val_i64))))
            } else if let Some(val_u64) = val.as_u64() {
                Ok(Some(JsonValue::Number(JsonNumber::from(val_u64))))
            } else if let Some(val_f64) = val.as_f64() {
                let number = JsonNumber::from_f64(val_f64)
                    .ok_or(Box::new(IOError::new(ErrorKind::InvalidData, "Value not supported for a number in JSON format.!")))?;

                Ok(Some(JsonValue::Number(number)))
            } else {
                let error = IOError::new(
                    ErrorKind::InvalidData,
                    "Value not supported for a number in JSON format!"
                );
                Err(Box::new(error))
            }
        },
        YamlValue::String(val) => Ok(Some(JsonValue::String(val))),
        YamlValue::Sequence(arr) => {
            let mut values: Vec<JsonValue> = Vec::new();

            for value in arr {
                if let Some(json_val) = yaml_to_json(value)? {
                    values.push(json_val);
                }
            }

            Ok(Some(JsonValue::Array(values)))
        },
        YamlValue::Mapping(mapping) => {
            let mut json_map: JsonMap<String, JsonValue> = JsonMap::new();

            for (k, v) in mapping {
                let key = k.as_str()
                    .ok_or(Box::new(IOError::new(ErrorKind::InvalidData, "Unsupported type of key for json format")))?;

                if let Some(json_val) = yaml_to_json(v)? {
                    json_map.insert(key.to_string(), json_val);
                }
            }

            Ok(Some(JsonValue::Object(json_map)))
        }, 
        YamlValue::Tagged(_) => Ok(None)
    }
}

pub fn yaml_to_toml(value: YamlValue) -> Result<Option<TomlValue>, Box<dyn Error>> {
    match value {
        YamlValue::Bool(val) => Ok(Some(TomlValue::Boolean(val))),
        YamlValue::Number(val) => {
            if let Some(val_i64) = val.as_i64() {
                Ok(Some(TomlValue::Integer(val_i64)))
            } else if let Some(val_f64) = val.as_f64() {
                Ok(Some(TomlValue::Float(val_f64)))
            } else {
                let error = IOError::new(
                    ErrorKind::InvalidData,
                    "Value not supported for a number in TOML format."
                );
                Err(Box::new(error))
            }
        },
        YamlValue::String(val) => {
            if is_iso_8601(&val) {
                Ok(Some(TomlValue::Datetime(Datetime::from_str(&val).unwrap())))
            } else {
                Ok(Some(TomlValue::String(val)))
            }
        },
        YamlValue::Sequence(arr) => {
            let mut values: Vec<TomlValue> = Vec::new();
            for value in arr {
                if let Some(toml_val) = yaml_to_toml(value)? {
                    values.push(toml_val);
                }
            }

            Ok(Some(TomlValue::Array(values)))
        },
        YamlValue::Mapping(mapping) => {
            let mut current_map: Map<String, TomlValue> = Map::new();

            for (k, v) in mapping {
                let key = k.as_str()
                    .ok_or(Box::new(IOError::new(ErrorKind::InvalidData, "Unsupported type of key for toml format")))?;

                if let Some(toml_val) = yaml_to_toml(v)? {
                    current_map.insert(key.to_string(), toml_val);
                }
            }

            Ok(Some(TomlValue::Table(current_map)))
        },
        YamlValue::Tagged(_) => Ok(None),
        YamlValue::Null => Ok(None)
    }
}


