use std::error::Error;
use std::io::{Error as IOError, ErrorKind};
use std::str::FromStr;

use serde_json::Value as JsonValue;
use serde_yaml_ng::Value as YamlValue;
use toml::Value as TomlValue;

use toml::map::Map;
use toml::value::Datetime;

use serde_yaml_ng::Mapping;
use serde_yaml_ng::Number as YamlNumber;

use crate::utils::is_iso_8601;

pub fn json_to_toml(value: JsonValue) -> Result<Option<TomlValue>, Box<dyn Error>> {
    match value {
        JsonValue::Bool(val) => Ok(Some(TomlValue::Boolean(val))),
        JsonValue::Number(val) => {
            if let Some(val_i64) = val.as_i64() {
                Ok(Some(TomlValue::Integer(val_i64)))
            } else if let Some(val_f64) = val.as_f64() {
                Ok(Some(TomlValue::Float(val_f64)))
            } else {
                let error = IOError::new(
                    ErrorKind::InvalidData,
                    "Value not supported for a number in TOML format.!",
                );
                Err(Box::new(error))
            }
        }
        JsonValue::String(s) => {
            if is_iso_8601(&s) {
                Ok(Some(TomlValue::Datetime(Datetime::from_str(&s)?)))
            } else {
                Ok(Some(TomlValue::String(s)))
            }
        }
        JsonValue::Object(map) => {
            let mut current_map: Map<String, TomlValue> = Map::new();
            for (k, v) in map {
                if let Some(toml_value) = json_to_toml(v)? {
                    current_map.insert(k, toml_value);
                }
            }
            Ok(Some(TomlValue::Table(current_map)))
        }
        JsonValue::Array(arr) => {
            let mut values: Vec<toml::Value> = Vec::new();
            for value in arr {
                if let Some(toml_value) = json_to_toml(value)? {
                    values.push(toml_value)
                }
            }
            Ok(Some(TomlValue::Array(values)))
        }
        JsonValue::Null => Ok(None),
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
                    "Value not supported for a number in YAML format.",
                );
                Err(Box::new(error))
            }
        }
        JsonValue::String(val) => Ok(YamlValue::String(val)),
        JsonValue::Object(map) => {
            let mut current_mapping: Mapping = Mapping::new();
            for (k, v) in map {
                let yaml_val = json_to_yaml(v)?;
                current_mapping.insert(YamlValue::String(k), yaml_val);
            }

            Ok(YamlValue::Mapping(current_mapping))
        }
        JsonValue::Array(arr) => {
            let mut values: Vec<YamlValue> = Vec::new();
            for val in arr {
                let yaml_val = json_to_yaml(val)?;
                values.push(yaml_val);
            }

            Ok(YamlValue::Sequence(values))
        }
        JsonValue::Null => Ok(YamlValue::Null),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn json_to_toml_converts_bool_true() {
        let json = json!(true);
        let result = json_to_toml(json).unwrap();
        assert_eq!(result, Some(TomlValue::Boolean(true)));
    }

    #[test]
    fn json_to_toml_converts_bool_false() {
        let json = json!(false);
        let result = json_to_toml(json).unwrap();
        assert_eq!(result, Some(TomlValue::Boolean(false)));
    }

    #[test]
    fn json_to_toml_converts_i64() {
        let json = json!(42);
        let result = json_to_toml(json).unwrap();
        assert_eq!(result, Some(TomlValue::Integer(42)));
    }

    #[test]
    fn json_to_toml_converts_f64() {
        let json = json!(3.14);
        let result = json_to_toml(json).unwrap();
        assert_eq!(result, Some(TomlValue::Float(3.14)));
    }

    #[test]
    fn json_to_toml_converts_plain_string() {
        let json = json!("hello world");
        let result = json_to_toml(json).unwrap();
        assert_eq!(result, Some(TomlValue::String("hello world".to_string())));
    }

    #[test]
    fn json_to_toml_converts_iso8601_string_to_datetime() {
        let json = json!("2023-10-25T12:00:00Z");
        let result = json_to_toml(json).unwrap();
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), TomlValue::Datetime(_)));
    }

    #[test]
    fn json_to_toml_converts_empty_object() {
        let json = json!({});
        let result = json_to_toml(json).unwrap();
        let expected_map = Map::new();
        assert_eq!(result, Some(TomlValue::Table(expected_map)));
    }

    #[test]
    fn json_to_toml_converts_nested_object() {
        let json = json!({"key": "value", "nested": {"inner": 1}});
        let result = json_to_toml(json).unwrap();

        let mut inner_map = Map::new();
        inner_map.insert("inner".to_string(), TomlValue::Integer(1));

        let mut expected_map = Map::new();
        expected_map.insert("key".to_string(), TomlValue::String("value".to_string()));
        expected_map.insert("nested".to_string(), TomlValue::Table(inner_map));

        assert_eq!(result, Some(TomlValue::Table(expected_map)));
    }

    #[test]
    fn json_to_toml_converts_empty_array() {
        let json = json!([]);
        let result = json_to_toml(json).unwrap();
        assert_eq!(result, Some(TomlValue::Array(vec![])));
    }

    #[test]
    fn json_to_toml_converts_array_with_multiple_types() {
        let json = json!([1, "two", true, 3.0]);
        let result = json_to_toml(json).unwrap();

        let expected_array = vec![
            TomlValue::Integer(1),
            TomlValue::String("two".to_string()),
            TomlValue::Boolean(true),
            TomlValue::Float(3.0),
        ];

        assert_eq!(result, Some(TomlValue::Array(expected_array)));
    }

    #[test]
    fn json_to_toml_converts_null_to_none() {
        let json = json!(null);
        let result = json_to_toml(json).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn json_to_toml_filters_out_nulls_from_object() {
        let json = json!({"valid": 1, "invalid": null});
        let result = json_to_toml(json).unwrap();

        let mut expected_map = Map::new();
        expected_map.insert("valid".to_string(), TomlValue::Integer(1));

        assert_eq!(result, Some(TomlValue::Table(expected_map)));
    }

    #[test]
    fn json_to_toml_filters_out_nulls_from_array() {
        let json = json!([1, null, 2]);
        let result = json_to_toml(json).unwrap();

        let expected_array = vec![TomlValue::Integer(1), TomlValue::Integer(2)];

        assert_eq!(result, Some(TomlValue::Array(expected_array)));
    }

    #[test]
    fn json_to_yaml_converts_bool_true() {
        let json = json!(true);
        let result = json_to_yaml(json).unwrap();
        assert_eq!(result, YamlValue::Bool(true));
    }

    #[test]
    fn json_to_yaml_converts_bool_false() {
        let json = json!(false);
        let result = json_to_yaml(json).unwrap();
        assert_eq!(result, YamlValue::Bool(false));
    }

    #[test]
    fn json_to_yaml_converts_i64() {
        let json = json!(-42);
        let result = json_to_yaml(json).unwrap();
        assert_eq!(result, YamlValue::Number(YamlNumber::from(-42i64)));
    }

    #[test]
    fn json_to_yaml_converts_u64() {
        let json = json!(18446744073709551615u64);
        let result = json_to_yaml(json).unwrap();
        assert_eq!(
            result,
            YamlValue::Number(YamlNumber::from(18446744073709551615u64))
        );
    }

    #[test]
    fn json_to_yaml_converts_f64() {
        let json = json!(3.14);
        let result = json_to_yaml(json).unwrap();
        assert_eq!(result, YamlValue::Number(YamlNumber::from(3.14f64)));
    }

    #[test]
    fn json_to_yaml_converts_string() {
        let json = json!("hello world");
        let result = json_to_yaml(json).unwrap();
        assert_eq!(result, YamlValue::String("hello world".to_string()));
    }

    #[test]
    fn json_to_yaml_converts_empty_object() {
        let json = json!({});
        let result = json_to_yaml(json).unwrap();
        assert_eq!(result, YamlValue::Mapping(Mapping::new()));
    }

    #[test]
    fn json_to_yaml_converts_nested_object() {
        let json = json!({"key": "value", "nested": {"inner": true}});
        let result = json_to_yaml(json).unwrap();

        let mut inner_mapping = Mapping::new();
        inner_mapping.insert(
            YamlValue::String("inner".to_string()),
            YamlValue::Bool(true),
        );

        let mut expected_mapping = Mapping::new();
        expected_mapping.insert(
            YamlValue::String("key".to_string()),
            YamlValue::String("value".to_string()),
        );
        expected_mapping.insert(
            YamlValue::String("nested".to_string()),
            YamlValue::Mapping(inner_mapping),
        );

        assert_eq!(result, YamlValue::Mapping(expected_mapping));
    }

    #[test]
    fn json_to_yaml_converts_empty_array() {
        let json = json!([]);
        let result = json_to_yaml(json).unwrap();
        assert_eq!(result, YamlValue::Sequence(vec![]));
    }

    #[test]
    fn json_to_yaml_converts_array_with_multiple_types() {
        let json = json!([1, "two", false]);
        let result = json_to_yaml(json).unwrap();

        let expected_sequence = vec![
            YamlValue::Number(YamlNumber::from(1i64)),
            YamlValue::String("two".to_string()),
            YamlValue::Bool(false),
        ];

        assert_eq!(result, YamlValue::Sequence(expected_sequence));
    }

    #[test]
    fn json_to_yaml_converts_null_to_yaml_null() {
        let json = json!(null);
        let result = json_to_yaml(json).unwrap();
        assert_eq!(result, YamlValue::Null);
    }

    #[test]
    fn json_to_yaml_preserves_nulls_inside_object() {
        let json = json!({"valid": 1, "invalid": null});
        let result = json_to_yaml(json).unwrap();

        let mut expected_mapping = Mapping::new();
        expected_mapping.insert(
            YamlValue::String("valid".to_string()),
            YamlValue::Number(YamlNumber::from(1i64)),
        );
        expected_mapping.insert(YamlValue::String("invalid".to_string()), YamlValue::Null);

        assert_eq!(result, YamlValue::Mapping(expected_mapping));
    }

    #[test]
    fn json_to_yaml_preserves_nulls_inside_array() {
        let json = json!([1, null, 2]);
        let result = json_to_yaml(json).unwrap();

        let expected_sequence = vec![
            YamlValue::Number(YamlNumber::from(1i64)),
            YamlValue::Null,
            YamlValue::Number(YamlNumber::from(2i64)),
        ];

        assert_eq!(result, YamlValue::Sequence(expected_sequence));
    }
}
