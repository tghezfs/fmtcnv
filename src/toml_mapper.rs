use std::error::Error;
use std::io::{ Error as IOError, ErrorKind };

use toml::Value as TomlValue; 
use serde_json::Value as JsonValue;
use serde_yaml_ng::Value as YamlValue;

use serde_json::Map as JsonMap;
use serde_json::Number as JsonNumber;

use serde_yaml_ng::Number as YamlNumber;
use serde_yaml_ng::Mapping;

pub fn toml_to_json(value: TomlValue) -> Result<JsonValue, Box<dyn Error>> {
    match value {
        TomlValue::Boolean(val) => Ok(JsonValue::Bool(val)),
        TomlValue::Float(val) => {
            let number = JsonNumber::from_f64(val)
                .ok_or(Box::new(
                    IOError::new(
                    ErrorKind::InvalidData, 
                    "Value not supported for a number in JSON format.!"
                    )
                ))?;
            
            Ok(JsonValue::Number(number))
        },
        TomlValue::Integer(val) => Ok(JsonValue::Number(JsonNumber::from(val))),
        TomlValue::String(val) => Ok(JsonValue::String(val)),
        TomlValue::Datetime(val) => Ok(JsonValue::String(val.to_string())),
        TomlValue::Table(map) => {
            let mut json_map: JsonMap<String, serde_json::Value> = JsonMap::new();

            for (k, v) in map {
                let json_val = toml_to_json(v)?;

                json_map.insert(k, json_val);
            }

            Ok(JsonValue::Object(json_map))
        },
        TomlValue::Array(arr) => {
            let mut values: Vec<JsonValue> = Vec::new();
            for value in arr {
                let json_val = toml_to_json(value)?;
                values.push(json_val);
            }

            Ok(JsonValue::Array(values))
        }
    }
}

pub fn toml_to_yaml(value: TomlValue) -> YamlValue {
    match value {
        TomlValue::Boolean(val) => YamlValue::Bool(val),
        TomlValue::String(val) => YamlValue::String(val),
        TomlValue::Datetime(val) => YamlValue::String(val.to_string()),
        TomlValue::Integer(val) => YamlValue::Number(YamlNumber::from(val)),
        TomlValue::Float(val) => YamlValue::Number(YamlNumber::from(val)),
        TomlValue::Array(arr) => {
            let mut values: Vec<YamlValue> = Vec::new();

            for value in arr {
                let yaml_val = toml_to_yaml(value);

                values.push(yaml_val);
            }

            YamlValue::Sequence(values)
        },
        TomlValue::Table(map) => {
            let mut current_mapping: Mapping = Mapping::new();

            for (k, v) in map {
                let yaml_val = toml_to_yaml(v);

                current_mapping.insert(YamlValue::String(k), yaml_val);
            }

            YamlValue::Mapping(current_mapping)
        }
    }
}
