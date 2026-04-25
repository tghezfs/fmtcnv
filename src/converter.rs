use crate::format::Format;
use std::error::Error;
use std::io::{ Error as IOError, ErrorKind};
use std::fs;
use std::path::Path;

use serde_json::Value as JsonValue;
use serde_yaml_ng::Value as YamlValue;
use toml::Value as TomlValue;

use crate::mapper::json::{ json_to_toml, json_to_yaml };
use crate::mapper::toml::{ toml_to_json, toml_to_yaml };
use crate::mapper::yaml::{ yaml_to_json, yaml_to_toml };

enum ParsedData {
    Json(JsonValue),
    Toml(TomlValue),
    Yaml(YamlValue),
}

pub fn convert(in_format: Format, out_format: Format, path: &Path) -> Result<Option<String>, Box<dyn Error>>  {
    let content = fs::read(path)?;

    if in_format == out_format {
        return Ok(Some(String::from_utf8_lossy(&content).into_owned()));
    }

    let parsed_data = match in_format {
        Format::Json => ParsedData::Json(serde_json::from_slice(&content)?),
        Format::Toml => ParsedData::Toml(toml::from_slice(&content)?),
        Format::Yaml => ParsedData::Yaml(serde_yaml_ng::from_slice(&content)?),
        Format::Unknown => {
            if let Ok(val) = serde_json::from_slice::<JsonValue>(&content) {
                ParsedData::Json(val)
            } else if let Ok(val) = toml::from_slice::<TomlValue>(&content) {
                ParsedData::Toml(val)
            } else if let Ok(val) = serde_yaml_ng::from_slice::<YamlValue>(&content) {
                ParsedData::Yaml(val)
            } else {
                return Err(Box::new(IOError::new(ErrorKind::InvalidInput, "Invalid file format")));
            }
        }
    };

    match (parsed_data, &out_format) {
        (ParsedData::Json(json), Format::Toml) => {
            transform_and_serialize(
                json, 
                json_to_toml, 
                |tree| Ok(toml::to_string(tree)?)
            )
        },
        (ParsedData::Json(json), Format::Yaml) => {
            let yaml_tree = json_to_yaml(json)?;
            let yaml_string = serde_yaml_ng::to_string(&yaml_tree)?;
            return Ok(Some(yaml_string));
        },
        (ParsedData::Toml(toml_value), Format::Json) => {
            let json_tree = toml_to_json(toml_value)?;
            let json_string = serde_json::to_string(&json_tree)?;
            return Ok(Some(json_string));
        },
        (ParsedData::Toml(toml_value), Format::Yaml) => {
            let yaml_tree = toml_to_yaml(toml_value);
            let yaml_string = serde_yaml_ng::to_string(&yaml_tree)?;
            return Ok(Some(yaml_string));
        },
        (ParsedData::Yaml(yaml), Format::Json) => {
            transform_and_serialize(
                yaml, 
                yaml_to_json,
                |tree| Ok(serde_json::to_string(tree)?)
            )
        },
        (ParsedData::Yaml(yaml), Format::Toml) => {
            transform_and_serialize(
                yaml, 
                yaml_to_toml,
                |tree| Ok(toml::to_string(tree)?)
            )
        },
        _  => return Ok(Some(String::from_utf8_lossy(&content).into_owned()))
    }
}

fn transform_and_serialize<T, S, F>(
    input: T, 
    mapper_fn: F, 
    serialize_fn: impl FnOnce(&S) -> Result<String, Box<dyn Error>>
) -> Result<Option<String>, Box<dyn Error>> 
where
    T: serde::Serialize,
    F: FnOnce(T) -> Result<Option<S>, Box<dyn Error>>
{
    match mapper_fn(input)? {
        Some(tree) => Ok(Some(serialize_fn(&tree)?)),
        None => {
            eprintln!("⚠️ Warning: There was a format incompatibility.\n\
                    Some data could not be converted correctly.");
            Ok(None)
        }
    }
}
