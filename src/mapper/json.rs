use std::error::Error;
use std::io::{ Error as IOError, ErrorKind };
use std::str::FromStr;

use serde_json::Value as JsonValue;
use toml::Value as TomlValue;
use serde_yaml_ng::Value as YamlValue;

use toml::map::Map;
use toml::value::Datetime;

use serde_yaml_ng::Number as YamlNumber;
use serde_yaml_ng::Mapping;

use crate::utils::is_iso_8601;

pub fn json_to_toml(value: JsonValue) -> Result<Option<TomlValue>, Box<dyn Error>> {
    match value {
        JsonValue::Bool(val) =>Ok(Some(TomlValue::Boolean(val))),
        JsonValue::Number(val) => {
            if let Some(val_i64) = val.as_i64() {
                Ok(Some(TomlValue::Integer(val_i64)))
            } else if let Some(val_f64) = val.as_f64() {
                Ok(Some(TomlValue::Float(val_f64)))
            } else {
                let error = IOError::new(
                    ErrorKind::InvalidData, 
                    "Value not supported for a number in TOML format.!"
                );
                Err(Box::new(error))
            }
        },
        JsonValue::String(s) => {
            if is_iso_8601(&s) {
                Ok(Some(TomlValue::Datetime(Datetime::from_str(&s)?)))
            } else {
                Ok(Some(TomlValue::String(s)))
            }
        },
        JsonValue::Object(map) => {
            let mut current_map: Map<String, TomlValue> = Map::new();
            for (k, v) in map {
                if let Some(toml_value) = json_to_toml(v)? { 
                    current_map.insert(k, toml_value);
                }
            }
            Ok(Some(TomlValue::Table(current_map)))
        },
        JsonValue::Array(arr ) => {
            let mut values: Vec<toml::Value> = Vec::new();
            for value in arr {
                if let Some(toml_value) = json_to_toml(value)? {
                    values.push(toml_value)
                }
            }
            Ok(Some(TomlValue::Array(values)))
        },
        JsonValue::Null => Ok(None)
    }
}

pub fn json_to_yaml(value: JsonValue) -> Result<YamlValue, Box<dyn Error>> {
    match value {
        JsonValue::Bool(val) => Ok(YamlValue::Bool(val)),
        JsonValue::Number(val) => {
            
            if let Some(val_i64) = val.as_i64() {
                Ok(YamlValue::Number(YamlNumber::from(val_i64)))
            } else if let Some(val_u64) = val.as_u64() {
                Ok(YamlValue::Number(YamlNumber::from(val_u64)))
            } else if let Some(val_f64) = val.as_f64() {
                Ok(YamlValue::Number(YamlNumber::from(val_f64)))
            } else {
                let error = IOError::new(
                    ErrorKind::InvalidData, 
                    "Value not supported for a number in YAML format.!"
                );
                Err(Box::new(error))
            }
        },
        JsonValue::String(val) => Ok(YamlValue::String(val)),
        JsonValue::Object(map) => {
            let mut current_mapping: Mapping = Mapping::new();
            for (k, v) in map {
                let yaml_val = json_to_yaml(v)?;
                current_mapping.insert(
                    YamlValue::String(k), 
                    yaml_val
                );
            }

            Ok(YamlValue::Mapping(current_mapping))
        },
        JsonValue::Array(arr) => {
            let mut values: Vec<YamlValue> = Vec::new();
            for val in arr {
                let yaml_val = json_to_yaml(val)?;
                values.push(yaml_val);
            }

            Ok(YamlValue::Sequence(values))
        },
        JsonValue::Null => Ok(YamlValue::Null)
    }
}

