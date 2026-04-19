use std::error::Error;
use std::io::{ErrorKind, Error as IOError};
use std::str::FromStr;

use chrono::{ DateTime, NaiveDateTime, NaiveDate };

use serde_yaml_ng::Mapping;
use serde_yaml_ng::value::Number as YamlNumber;
use toml::map::Map;
use serde_json::Map as JsonMap;
use serde_json::value::Number as JsonNumber;

pub enum Format {
    Json,
    Yaml,
    Toml
}

pub fn parse_format(format: &str) -> Result<Format, Box<dyn Error>> {
    match format.to_lowercase().as_str() {
        "json" => Ok(Format::Json),
        "yaml" | "yml" => Ok(Format::Yaml),
        "toml" => Ok(Format::Toml),
        _ => Err(Box::new(IOError::new(
                    ErrorKind::InvalidInput, "Invalid Format!")))
    }
}


pub fn json_to_toml(value: serde_json::Value) -> Option<toml::Value> {
    match value {
        serde_json::Value::Bool(b) => Some(toml::Value::Boolean(b)),
        serde_json::Value::Number(n) => {
            if n.is_f64() {
                Some(toml::Value::Float(n.as_f64().unwrap()))
            } else {
                Some(toml::Value::Integer(n.as_i64().unwrap()))
            }
        },
        serde_json::Value::String(s) => {
            if DateTime::parse_from_rfc3339(&s).is_ok() || 
                NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S").is_ok() ||
                    NaiveDate::parse_from_str(&s, "%Y-%m-%d").is_ok() {
                Some(toml::Value::Datetime(toml::value::Datetime::from_str(&s).unwrap()))
            } else {
                Some(toml::Value::String(s))
            }
        },
        serde_json::Value::Object(map) => {
            let mut current_map: Map<String, toml::Value> = Map::new();
            for (k, v) in map {
                if let Some(toml_value) = json_to_toml(v) { 
                    current_map.insert(k, toml_value);
                }
            }
            Some(toml::Value::Table(current_map))
        },
        serde_json::Value::Array(arr ) => {
            let mut values: Vec<toml::Value> = Vec::new();
            for value in arr {
                if let Some(toml_value) = json_to_toml(value) {
                    values.push(toml_value)
                }
            }
            Some(toml::Value::Array(values))
        },
        serde_json::Value::Null => None
    }
}

pub fn json_to_yaml(value: serde_json::Value) -> serde_yaml_ng::Value {
    match value {
        serde_json::Value::Bool(val) => serde_yaml_ng::Value::Bool(val),
        serde_json::Value::Number(val) => {
            if val.is_i64() {
                serde_yaml_ng::Value::Number(YamlNumber::from(val.as_i64().unwrap()))
            } else if val.is_u64() {
                serde_yaml_ng::Value::Number(YamlNumber::from(val.as_u64().unwrap()))
            } else if val.is_f64() {
                serde_yaml_ng::Value::Number(YamlNumber::from(val.as_f64().unwrap()))
            } else {
                // This case might return an error
                serde_yaml_ng::Value::String(val.to_string()) 
            }
        },
        serde_json::Value::String(val) => serde_yaml_ng::Value::String(val),
        serde_json::Value::Object(map) => {
            let mut current_mapping: Mapping = Mapping::new();
            for (k, v) in map {
                let yaml_val = json_to_yaml(v);
                current_mapping.insert(
                    serde_yaml_ng::Value::String(k), 
                    yaml_val
                );
            }

            serde_yaml_ng::Value::Mapping(current_mapping) 
        },
        serde_json::Value::Array(arr) => {
            let mut values: Vec<serde_yaml_ng::Value> = Vec::new();
            for val in arr {
                let yaml_val = json_to_yaml(val);
                values.push(yaml_val);
            }

            serde_yaml_ng::Value::Sequence(values)
        },
        serde_json::Value::Null => serde_yaml_ng::Value::Null
    }
}

pub fn toml_to_json(value: toml::Value ) -> serde_json::Value {
    match value {
        toml::Value::Boolean(val) => serde_json::Value::Bool(val),
        toml::Value::Float(val) => serde_json::Value::Number(JsonNumber::from_f64(val).unwrap()),
        toml::Value::Integer(val) => serde_json::Value::Number(JsonNumber::from(val)),
        toml::Value::String(val) => serde_json::Value::String(val),
        toml::Value::Datetime(val) => serde_json::Value::String(val.to_string()),
        toml::Value::Table(map) => {
            let mut json_map: JsonMap<String, serde_json::Value> = JsonMap::new();

            for (k, v) in map {
                let json_val = toml_to_json(v);

                json_map.insert(k, json_val);
            }

            serde_json::Value::Object(json_map)
        },
        toml::Value::Array(arr) => {
            let mut values: Vec<serde_json::Value> = Vec::new();
            for value in arr {
                let json_val = toml_to_json(value);
                values.push(json_val);
            }

            serde_json::Value::Array(values)
        }
    }
}

pub fn toml_to_yaml(value: toml::Value ) -> serde_yaml_ng::Value {
    match value {
        toml::Value::Boolean(val) => serde_yaml_ng::Value::Bool(val),
        toml::Value::String(val) => serde_yaml_ng::Value::String(val),
        toml::Value::Datetime(val) => serde_yaml_ng::Value::String(val.to_string()),
        toml::Value::Integer(val) => serde_yaml_ng::Value::Number(YamlNumber::from(val)),
        toml::Value::Float(val) => serde_yaml_ng::Value::Number(YamlNumber::from(val)),
        toml::Value::Array(arr) => {
            let mut values: Vec<serde_yaml_ng::Value> = Vec::new();

            for value in arr {
                let yaml_val = toml_to_yaml(value);

                values.push(yaml_val);
            }

            serde_yaml_ng::Value::Sequence(values)
        },
        toml::Value::Table(map) => {
            let mut current_mapping: Mapping = Mapping::new();

            for (k, v) in map {
                let yaml_val = toml_to_yaml(v);

                current_mapping.insert(serde_yaml_ng::Value::String(k), yaml_val);
            }

            serde_yaml_ng::Value::Mapping(current_mapping)
        }
    }
}

pub fn yaml_to_json(value: serde_yaml_ng::Value ) -> Option<serde_json::Value> {
    match value {
        serde_yaml_ng::Value::Null => Some(serde_json::Value::Null),
        serde_yaml_ng::Value::Bool(val) => Some(serde_json::Value::Bool(val)),
        serde_yaml_ng::Value::Number(val) => {
            if val.is_i64() {
                Some(serde_json::Value::Number(JsonNumber::from(val.as_i64().unwrap())))
            } else if val.is_u64() {
                Some(serde_json::Value::Number(JsonNumber::from(val.as_u64().unwrap())))
            } else if val.is_f64() {
                Some(serde_json::Value::Number(JsonNumber::from_f64(val.as_f64().unwrap()).unwrap()))
            } else {
                //Instead, this may return an error
                Some(serde_json::Value::String(val.to_string()))
            }
        },
        serde_yaml_ng::Value::String(val) => Some(serde_json::Value::String(val)),
        serde_yaml_ng::Value::Sequence(arr) => {
            let mut values: Vec<serde_json::Value> = Vec::new();

            for value in arr {
                if let Some(json_val) = yaml_to_json(value) {
                    values.push(json_val);
                }
            }

            Some(serde_json::Value::Array(values))
        },
        serde_yaml_ng::Value::Mapping(mapping) => {
            let mut json_map: JsonMap<String, serde_json::Value> = JsonMap::new();

            for (k, v) in mapping {
                let key = k.as_str().unwrap();

                if let Some(json_val) = yaml_to_json(v) {
                    json_map.insert(key.to_string(), json_val);
                }
            }

            Some(serde_json::Value::Object(json_map))
        }, 
        serde_yaml_ng::Value::Tagged(_) => None
    }
}

pub fn yaml_to_toml(value: serde_yaml_ng::Value) -> Option<toml::Value> {
    match value {
        serde_yaml_ng::Value::Bool(val) => Some(toml::Value::Boolean(val)),
        serde_yaml_ng::Value::Number(val) => {
            if val.is_f64() {
                Some(toml::Value::Float(val.as_f64().unwrap()))      
            } else {
                Some(toml::Value::Integer(val.as_i64().unwrap()))
            }
        },
        serde_yaml_ng::Value::String(val) => {
            if DateTime::parse_from_rfc3339(&val).is_ok() || 
                NaiveDateTime::parse_from_str(&val, "%Y-%m-%dT%H:%M:%S").is_ok() ||
                    NaiveDate::parse_from_str(&val, "%Y-%m-%d").is_ok() {
                Some(toml::Value::Datetime(toml::value::Datetime::from_str(&val).unwrap()))
            } else {
                Some(toml::Value::String(val))
            }
        },
        serde_yaml_ng::Value::Sequence(arr) => {
            let mut values: Vec<toml::Value> = Vec::new();
            for value in arr {
                if let Some(toml_val) = yaml_to_toml(value) {
                    values.push(toml_val);
                }
            }

            Some(toml::Value::Array(values))
        },
        serde_yaml_ng::Value::Mapping(mapping) => {
            let mut current_map: Map<String, toml::Value> = Map::new();

            for (k, v) in mapping {
                let key = k.as_str().unwrap();

                if let Some(toml_val) = yaml_to_toml(v) {
                    current_map.insert(key.to_string(), toml_val);
                }
            }

            Some(toml::Value::Table(current_map))
        },
        serde_yaml_ng::Value::Tagged(_) => None,
        serde_yaml_ng::Value::Null => None
    }
}


