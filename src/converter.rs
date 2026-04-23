use crate::format::Format;
use std::error::Error;
use std::io::{Error as IOError, ErrorKind};
use std::fs;
use std::path::Path;

use serde_json::Value as JsonValue;
use serde_yaml_ng::Value as YamlValue;
use toml::Value as TomlValue;

use crate::mapper::json::{ json_to_toml, json_to_yaml };
use crate::mapper::toml::{ toml_to_json, toml_to_yaml };
use crate::mapper::yaml::{ yaml_to_json, yaml_to_toml };

pub fn convert(in_format: Format, out_format: Format, path: &Path) -> Result<Option<String>, Box<dyn Error>>  {
    let content = fs::read_to_string(path)?;
    match (in_format, &out_format) {
        (Format::Json, Format::Toml) => {
            let parsed_content: JsonValue = serde_json::from_str(&content)?;

            if let Some(toml_tree) = json_to_toml(parsed_content)? {
                let toml_string = toml::to_string(&toml_tree)?;
                println!("toml string \n\n{}", toml_string);
                return Ok(Some(toml_string));
            } else {
                eprintln!("⚠️ Warning: There was a format incompatibility.\n\
                            Some data could not be converted correctly.");
                return Ok(None);
            }
        },
        (Format::Json, Format::Yaml) => {
            let parsed_content: JsonValue = serde_json::from_str(&content)?;
            let yaml_tree = json_to_yaml(parsed_content)?;
            let yaml_string = serde_yaml_ng::to_string(&yaml_tree)?;
            println!("yaml string \n\n{}", yaml_string);
            return Ok(Some(yaml_string));
        },
        (Format::Toml, Format::Json) => {
            let parsed_content: TomlValue = toml::from_str(&content)?;
            let json_tree = toml_to_json(parsed_content)?;
            let json_string = toml::to_string(&json_tree)?;
            println!("yaml string \n\n{}", json_string);
            return Ok(Some(json_string));
        },
        (Format::Toml, Format::Yaml) => {
            let parsed_content: TomlValue = toml::from_str(&content)?;
            let yaml_tree = toml_to_yaml(parsed_content);
            let yaml_string = serde_yaml_ng::to_string(&yaml_tree)?;
            println!("yaml string \n\n{}", yaml_string);
            return Ok(Some(yaml_string));
        },
        (Format::Yaml, Format::Json) => {
            let parsed_content: YamlValue = serde_yaml_ng::from_str(&content)?;

            if let Some(json_tree ) = yaml_to_json(parsed_content)? {
                let json_string = toml::to_string(&json_tree)?;
                println!("yaml string \n\n{}", json_string);
                return Ok(Some(json_string));
            } else {
                eprintln!("⚠️ Warning: There was a format incompatibility.\n\
                            Some data could not be converted correctly.");
                return Ok(None);
            }
        },
        (Format::Yaml, Format::Toml) => {
            let parsed_content: YamlValue = serde_yaml_ng::from_str(&content)?;

            if let Some(toml_tree) = yaml_to_toml(parsed_content)? {
                let toml_string = toml::to_string(&toml_tree)?;
                println!("toml string \n\n{}", toml_string);
                return Ok(Some(toml_string));
            } else {
                eprintln!("⚠️ Warning: There was a format incompatibility.\n\
                            Some data could not be converted correctly.");
                return Ok(None);
            }
        },
        (Format::Unknown, _) => {
            // This needs to be improved because it's doing double the work.
            if serde_json::from_str::<JsonValue>(&content).is_ok() {
                return convert(Format::Json, out_format, path);
            }

            if toml::from_str::<TomlValue>(&content).is_ok() {
                return convert(Format::Toml, out_format, path);
            }

            if serde_yaml_ng::from_str::<YamlValue>(&content).is_ok() {
                return convert(Format::Yaml, out_format, path);
            }

            let error = IOError::new(
                ErrorKind::InvalidInput,
                format!("Invalid file format")
            );

            return Err(Box::new(error));

        },
        (_, _) => return Ok(Some(content.to_string()))
    }
}

