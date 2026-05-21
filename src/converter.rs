use crate::format::Format;
use std::error::Error;
use std::fs;
use std::io::{Error as IOError, ErrorKind};
use std::path::Path;

use serde_json::Value as JsonValue;
use serde_yaml_ng::Value as YamlValue;
use toml::Value as TomlValue;

use crate::mapper::json::{json_to_toml, json_to_yaml};
use crate::mapper::toml::{toml_to_json, toml_to_yaml};
use crate::mapper::yaml::{yaml_to_json, yaml_to_toml};

#[derive(Debug)]
enum ParsedData {
    Json(JsonValue),
    Toml(TomlValue),
    Yaml(YamlValue),
}

pub fn convert(
    in_format: Format,
    out_format: Format,
    path: &Path,
) -> Result<Option<String>, Box<dyn Error>> {
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
                return Err(Box::new(IOError::new(
                    ErrorKind::InvalidInput,
                    "Invalid file format",
                )));
            }
        }
    };

    match (parsed_data, &out_format) {
        (ParsedData::Json(json), Format::Toml) => {
            transform_and_serialize(json, json_to_toml, |tree| Ok(toml::to_string(tree)?))
        }
        (ParsedData::Json(json), Format::Yaml) => {
            let yaml_tree = json_to_yaml(json)?;
            let yaml_string = serde_yaml_ng::to_string(&yaml_tree)?;
            return Ok(Some(yaml_string));
        }
        (ParsedData::Toml(toml_value), Format::Json) => {
            let json_tree = toml_to_json(toml_value)?;
            let json_string = serde_json::to_string(&json_tree)?;
            return Ok(Some(json_string));
        }
        (ParsedData::Toml(toml_value), Format::Yaml) => {
            let yaml_tree = toml_to_yaml(toml_value);
            let yaml_string = serde_yaml_ng::to_string(&yaml_tree)?;
            return Ok(Some(yaml_string));
        }
        (ParsedData::Yaml(yaml), Format::Json) => {
            transform_and_serialize(yaml, yaml_to_json, |tree| Ok(serde_json::to_string(tree)?))
        }
        (ParsedData::Yaml(yaml), Format::Toml) => {
            transform_and_serialize(yaml, yaml_to_toml, |tree| Ok(toml::to_string(tree)?))
        }
        _ => return Ok(Some(String::from_utf8_lossy(&content).into_owned())),
    }
}

fn transform_and_serialize<T, S, F>(
    input: T,
    mapper_fn: F,
    serialize_fn: impl FnOnce(&S) -> Result<String, Box<dyn Error>>,
) -> Result<Option<String>, Box<dyn Error>>
where
    T: serde::Serialize,
    F: FnOnce(T) -> Result<Option<S>, Box<dyn Error>>,
{
    match mapper_fn(input)? {
        Some(tree) => Ok(Some(serialize_fn(&tree)?)),
        None => {
            eprintln!(
                "⚠️ Warning: There was a format incompatibility.\n\
                    Some data could not be converted correctly."
            );
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn convert_same_format_json_returns_original_content() {
        let content = r#"{"key": "value"}"#;
        let file = create_temp_file(content);
        let result = convert(Format::Json, Format::Json, file.path()).unwrap();
        assert_eq!(result, Some(content.to_string()));
    }

    #[test]
    fn convert_same_format_toml_returns_original_content() {
        let content = "key = 'value'\n";
        let file = create_temp_file(content);
        let result = convert(Format::Toml, Format::Toml, file.path()).unwrap();
        assert_eq!(result, Some(content.to_string()));
    }

    #[test]
    fn convert_same_format_yaml_returns_original_content() {
        let content = "key: value\n";
        let file = create_temp_file(content);
        let result = convert(Format::Yaml, Format::Yaml, file.path()).unwrap();
        assert_eq!(result, Some(content.to_string()));
    }

    #[test]
    fn convert_json_to_toml() {
        let content = r#"{"key": "value", "num": 10}"#;
        let file = create_temp_file(content);
        let result = convert(Format::Json, Format::Toml, file.path()).unwrap().unwrap();
        assert!(result.contains("key = \"value\""));
        assert!(result.contains("num = 10"));
    }

    #[test]
    fn convert_json_to_yaml() {
        let content = r#"{"key": "value", "num": 10}"#;
        let file = create_temp_file(content);
        let result = convert(Format::Json, Format::Yaml, file.path()).unwrap().unwrap();
        assert!(result.contains("key: value"));
        assert!(result.contains("num: 10"));
    }

    #[test]
    fn convert_toml_to_json() {
        let content = "key = 'value'\nnum = 10\n";
        let file = create_temp_file(content);
        let result = convert(Format::Toml, Format::Json, file.path()).unwrap().unwrap();
        assert_eq!(result, r#"{"key":"value","num":10}"#);
    }

    #[test]
    fn convert_toml_to_yaml() {
        let content = "key = 'value'\nnum = 10\n";
        let file = create_temp_file(content);
        let result = convert(Format::Toml, Format::Yaml, file.path()).unwrap().unwrap();
        assert!(result.contains("key: value"));
        assert!(result.contains("num: 10"));
    }

    #[test]
    fn convert_yaml_to_json() {
        let content = "key: value\nnum: 10\n";
        let file = create_temp_file(content);
        let result = convert(Format::Yaml, Format::Json, file.path()).unwrap().unwrap();
        assert_eq!(result, r#"{"key":"value","num":10}"#);
    }

    #[test]
    fn convert_yaml_to_toml() {
        let content = "key: value\nnum: 10\n";
        let file = create_temp_file(content);
        let result = convert(Format::Yaml, Format::Toml, file.path()).unwrap().unwrap();
        assert!(result.contains("key = \"value\""));
        assert!(result.contains("num = 10"));
    }

    #[test]
    fn convert_unknown_format_detects_json() {
        let content = r#"{"key": "value"}"#;
        let file = create_temp_file(content);
        let result = convert(Format::Unknown, Format::Toml, file.path()).unwrap().unwrap();
        assert!(result.contains("key = \"value\""));
    }

    #[test]
    fn convert_unknown_format_detects_toml() {
        let content = "key = 'value'\n";
        let file = create_temp_file(content);
        let result = convert(Format::Unknown, Format::Json, file.path()).unwrap().unwrap();
        assert_eq!(result, r#"{"key":"value"}"#);
    }

    #[test]
    fn convert_unknown_format_detects_yaml() {
        let content = "key: value\n";
        let file = create_temp_file(content);
        let result = convert(Format::Unknown, Format::Json, file.path()).unwrap().unwrap();
        assert_eq!(result, r#"{"key":"value"}"#);
    }

    #[test]
    fn convert_unknown_format_fails_on_invalid_content() {
        let content = "{\n\tinvalid: [unterminated, ";
        let file = create_temp_file(content);
        let result = convert(Format::Unknown, Format::Json, file.path());
        assert!(result.is_err());
    }

    #[test]
    fn convert_non_existent_file_fails() {
        let path = Path::new("/tmp/this_file_does_not_exist_12345.json");
        let result = convert(Format::Json, Format::Toml, path);
        assert!(result.is_err());
    }

    #[test]
    fn convert_invalid_json_fails() {
        let content = "{invalid json}";
        let file = create_temp_file(content);
        let result = convert(Format::Json, Format::Toml, file.path());
        assert!(result.is_err());
    }

    #[test]
    fn convert_invalid_toml_fails() {
        let content = "invalid [[ toml";
        let file = create_temp_file(content);
        let result = convert(Format::Toml, Format::Json, file.path());
        assert!(result.is_err());
    }

    #[test]
    fn convert_invalid_yaml_fails() {
        let content = "invalid: [yaml: {]";
        let file = create_temp_file(content);
        let result = convert(Format::Yaml, Format::Json, file.path());
        assert!(result.is_err());
    }

    #[test]
    fn convert_json_null_to_toml_returns_none() {
        let content = "null";
        let file = create_temp_file(content);
        let result = convert(Format::Json, Format::Toml, file.path()).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn convert_yaml_null_to_toml_returns_none() {
        let content = "null\n";
        let file = create_temp_file(content);
        let result = convert(Format::Yaml, Format::Toml, file.path()).unwrap();
        assert_eq!(result, None);
    }
}
