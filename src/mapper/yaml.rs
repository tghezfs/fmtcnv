use std::error::Error;
use std::io::{Error as IOError, ErrorKind};
use std::str::FromStr;

use serde_json::Value as JsonValue;
use serde_yaml_ng::Value as YamlValue;

use toml::Value as TomlValue;
use toml::map::Map;
use toml::value::Datetime;

use serde_json::Map as JsonMap;
use serde_json::Number as JsonNumber;

use crate::utils::is_iso_8601;

pub fn yaml_to_json(value: YamlValue) -> Result<Option<JsonValue>, Box<dyn Error>> {
    match value {
        YamlValue::Null => Ok(Some(JsonValue::Null)),
        YamlValue::Bool(val) => Ok(Some(JsonValue::Bool(val))),
        YamlValue::Number(val) => {
            if let Some(val_i64) = val.as_i64() {
                Ok(Some(JsonValue::Number(JsonNumber::from(val_i64))))
            } else if let Some(val_u64) = val.as_u64() {
                Ok(Some(JsonValue::Number(JsonNumber::from(val_u64))))
            } else if let Some(val_f64) = val.as_f64() {
                let number = JsonNumber::from_f64(val_f64).ok_or(Box::new(IOError::new(
                    ErrorKind::InvalidData,
                    "Value not supported for a number in JSON format.!",
                )))?;

                Ok(Some(JsonValue::Number(number)))
            } else {
                let error = IOError::new(
                    ErrorKind::InvalidData,
                    "Value not supported for a number in JSON format!",
                );
                Err(Box::new(error))
            }
        }
        YamlValue::String(val) => Ok(Some(JsonValue::String(val))),
        YamlValue::Sequence(arr) => {
            let mut values: Vec<JsonValue> = Vec::new();

            for value in arr {
                if let Some(json_val) = yaml_to_json(value)? {
                    values.push(json_val);
                }
            }

            Ok(Some(JsonValue::Array(values)))
        }
        YamlValue::Mapping(mapping) => {
            let mut json_map: JsonMap<String, JsonValue> = JsonMap::new();

            for (k, v) in mapping {
                let key = k.as_str().ok_or(Box::new(IOError::new(
                    ErrorKind::InvalidData,
                    "Unsupported type of key for json format",
                )))?;

                if let Some(json_val) = yaml_to_json(v)? {
                    json_map.insert(key.to_string(), json_val);
                }
            }

            Ok(Some(JsonValue::Object(json_map)))
        }
        YamlValue::Tagged(_) => Ok(None),
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
                    "Value not supported for a number in TOML format.",
                );
                Err(Box::new(error))
            }
        }
        YamlValue::String(val) => {
            if is_iso_8601(&val) {
                Ok(Some(TomlValue::Datetime(Datetime::from_str(&val)?)))
            } else {
                Ok(Some(TomlValue::String(val)))
            }
        }
        YamlValue::Sequence(arr) => {
            let mut values: Vec<TomlValue> = Vec::new();
            for value in arr {
                if let Some(toml_val) = yaml_to_toml(value)? {
                    values.push(toml_val);
                }
            }

            Ok(Some(TomlValue::Array(values)))
        }
        YamlValue::Mapping(mapping) => {
            let mut current_map: Map<String, TomlValue> = Map::new();

            for (k, v) in mapping {
                let key = k.as_str().ok_or(Box::new(IOError::new(
                    ErrorKind::InvalidData,
                    "Unsupported type of key for toml format",
                )))?;

                if let Some(toml_val) = yaml_to_toml(v)? {
                    current_map.insert(key.to_string(), toml_val);
                }
            }

            Ok(Some(TomlValue::Table(current_map)))
        }
        YamlValue::Tagged(_) => Ok(None),
        YamlValue::Null => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml_ng::Mapping;
    use serde_yaml_ng::Number as YamlNumber;

    #[test]
    fn yaml_to_json_converts_null() {
        let yaml = YamlValue::Null;
        let result = yaml_to_json(yaml).unwrap();
        assert_eq!(result, Some(JsonValue::Null));
    }

    #[test]
    fn yaml_to_json_converts_bool_true() {
        let yaml = YamlValue::Bool(true);
        let result = yaml_to_json(yaml).unwrap();
        assert_eq!(result, Some(JsonValue::Bool(true)));
    }

    #[test]
    fn yaml_to_json_converts_bool_false() {
        let yaml = YamlValue::Bool(false);
        let result = yaml_to_json(yaml).unwrap();
        assert_eq!(result, Some(JsonValue::Bool(false)));
    }

    #[test]
    fn yaml_to_json_converts_i64() {
        let yaml = YamlValue::Number(YamlNumber::from(-42i64));
        let result = yaml_to_json(yaml).unwrap();
        assert_eq!(result, Some(JsonValue::Number(JsonNumber::from(-42i64))));
    }

    #[test]
    fn yaml_to_json_converts_u64() {
        let yaml = YamlValue::Number(YamlNumber::from(18446744073709551615u64));
        let result = yaml_to_json(yaml).unwrap();
        assert_eq!(
            result,
            Some(JsonValue::Number(JsonNumber::from(18446744073709551615u64)))
        );
    }

    #[test]
    fn yaml_to_json_converts_f64() {
        let yaml = YamlValue::Number(YamlNumber::from(3.14f64));
        let result = yaml_to_json(yaml).unwrap();
        assert_eq!(
            result,
            Some(JsonValue::Number(JsonNumber::from_f64(3.14).unwrap()))
        );
    }

    #[test]
    fn yaml_to_json_fails_on_nan_float() {
        let yaml = YamlValue::Number(YamlNumber::from(f64::NAN));
        let result = yaml_to_json(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn yaml_to_json_fails_on_infinity_float() {
        let yaml = YamlValue::Number(YamlNumber::from(f64::INFINITY));
        let result = yaml_to_json(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn yaml_to_json_converts_string() {
        let yaml = YamlValue::String("hello world".to_string());
        let result = yaml_to_json(yaml).unwrap();
        assert_eq!(result, Some(JsonValue::String("hello world".to_string())));
    }

    #[test]
    fn yaml_to_json_converts_empty_sequence() {
        let yaml = YamlValue::Sequence(vec![]);
        let result = yaml_to_json(yaml).unwrap();
        assert_eq!(result, Some(JsonValue::Array(vec![])));
    }

    #[test]
    fn yaml_to_json_converts_sequence_with_multiple_types() {
        let yaml = YamlValue::Sequence(vec![
            YamlValue::Number(YamlNumber::from(1i64)),
            YamlValue::String("two".to_string()),
            YamlValue::Bool(false),
        ]);
        let result = yaml_to_json(yaml).unwrap();

        let expected_array = vec![
            JsonValue::Number(JsonNumber::from(1i64)),
            JsonValue::String("two".to_string()),
            JsonValue::Bool(false),
        ];

        assert_eq!(result, Some(JsonValue::Array(expected_array)));
    }

    #[test]
    fn yaml_to_json_converts_empty_mapping() {
        let yaml = YamlValue::Mapping(Mapping::new());
        let result = yaml_to_json(yaml).unwrap();
        assert_eq!(result, Some(JsonValue::Object(JsonMap::new())));
    }

    #[test]
    fn yaml_to_json_converts_nested_mapping() {
        let mut inner_mapping = Mapping::new();
        inner_mapping.insert(
            YamlValue::String("inner".to_string()),
            YamlValue::Bool(true),
        );

        let mut outer_mapping = Mapping::new();
        outer_mapping.insert(
            YamlValue::String("key".to_string()),
            YamlValue::String("value".to_string()),
        );
        outer_mapping.insert(
            YamlValue::String("nested".to_string()),
            YamlValue::Mapping(inner_mapping),
        );

        let yaml = YamlValue::Mapping(outer_mapping);
        let result = yaml_to_json(yaml).unwrap();

        let mut expected_inner_json = JsonMap::new();
        expected_inner_json.insert("inner".to_string(), JsonValue::Bool(true));

        let mut expected_outer_json = JsonMap::new();
        expected_outer_json.insert("key".to_string(), JsonValue::String("value".to_string()));
        expected_outer_json.insert("nested".to_string(), JsonValue::Object(expected_inner_json));

        assert_eq!(result, Some(JsonValue::Object(expected_outer_json)));
    }

    #[test]
    fn yaml_to_json_fails_on_non_string_key_in_mapping() {
        let mut mapping = Mapping::new();
        mapping.insert(
            YamlValue::Number(YamlNumber::from(1i64)),
            YamlValue::String("value".to_string()),
        );

        let yaml = YamlValue::Mapping(mapping);
        let result = yaml_to_json(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn yaml_to_json_converts_tagged_to_none() {
        let tagged_value = serde_yaml_ng::value::TaggedValue {
            tag: serde_yaml_ng::value::Tag::new("!!str"),
            value: YamlValue::String("test".to_string()),
        };
        let yaml = YamlValue::Tagged(Box::new(tagged_value));
        let result = yaml_to_json(yaml).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn yaml_to_toml_converts_bool_true() {
        let yaml = YamlValue::Bool(true);
        let result = yaml_to_toml(yaml).unwrap();
        assert_eq!(result, Some(TomlValue::Boolean(true)));
    }

    #[test]
    fn yaml_to_toml_converts_bool_false() {
        let yaml = YamlValue::Bool(false);
        let result = yaml_to_toml(yaml).unwrap();
        assert_eq!(result, Some(TomlValue::Boolean(false)));
    }

    #[test]
    fn yaml_to_toml_converts_i64() {
        let yaml = YamlValue::Number(YamlNumber::from(-42i64));
        let result = yaml_to_toml(yaml).unwrap();
        assert_eq!(result, Some(TomlValue::Integer(-42)));
    }

    #[test]
    fn yaml_to_toml_converts_f64() {
        let yaml = YamlValue::Number(YamlNumber::from(3.14f64));
        let result = yaml_to_toml(yaml).unwrap();
        assert_eq!(result, Some(TomlValue::Float(3.14)));
    }

    #[test]
    fn yaml_to_toml_converts_plain_string() {
        let yaml = YamlValue::String("hello world".to_string());
        let result = yaml_to_toml(yaml).unwrap();
        assert_eq!(result, Some(TomlValue::String("hello world".to_string())));
    }

    #[test]
    fn yaml_to_toml_converts_iso8601_string_to_datetime() {
        let yaml = YamlValue::String("2023-10-25T12:00:00Z".to_string());
        let result = yaml_to_toml(yaml).unwrap();
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), TomlValue::Datetime(_)));
    }

    #[test]
    fn yaml_to_toml_converts_empty_sequence() {
        let yaml = YamlValue::Sequence(vec![]);
        let result = yaml_to_toml(yaml).unwrap();
        assert_eq!(result, Some(TomlValue::Array(vec![])));
    }

    #[test]
    fn yaml_to_toml_converts_sequence_with_multiple_types() {
        let yaml_seq = YamlValue::Sequence(vec![
            YamlValue::Number(YamlNumber::from(1i64)),
            YamlValue::String("two".to_string()),
            YamlValue::Bool(true),
        ]);
        let result = yaml_to_toml(yaml_seq).unwrap();

        let expected_array = vec![
            TomlValue::Integer(1),
            TomlValue::String("two".to_string()),
            TomlValue::Boolean(true),
        ];

        assert_eq!(result, Some(TomlValue::Array(expected_array)));
    }

    #[test]
    fn yaml_to_toml_converts_empty_mapping() {
        let yaml = YamlValue::Mapping(Mapping::new());
        let result = yaml_to_toml(yaml).unwrap();
        assert_eq!(result, Some(TomlValue::Table(Map::new())));
    }

    #[test]
    fn yaml_to_toml_converts_nested_mapping() {
        let mut inner_mapping = Mapping::new();
        inner_mapping.insert(
            YamlValue::String("inner".to_string()),
            YamlValue::Number(YamlNumber::from(100)),
        );

        let mut outer_mapping = Mapping::new();
        outer_mapping.insert(
            YamlValue::String("key".to_string()),
            YamlValue::String("value".to_string()),
        );
        outer_mapping.insert(
            YamlValue::String("nested".to_string()),
            YamlValue::Mapping(inner_mapping),
        );

        let yaml = YamlValue::Mapping(outer_mapping);
        let result = yaml_to_toml(yaml).unwrap();

        let mut expected_inner_map = Map::new();
        expected_inner_map.insert("inner".to_string(), TomlValue::Integer(100));

        let mut expected_outer_map = Map::new();
        expected_outer_map.insert("key".to_string(), TomlValue::String("value".to_string()));
        expected_outer_map.insert("nested".to_string(), TomlValue::Table(expected_inner_map));

        assert_eq!(result, Some(TomlValue::Table(expected_outer_map)));
    }

    #[test]
    fn yaml_to_toml_fails_on_non_string_key_in_mapping() {
        let mut mapping = Mapping::new();
        mapping.insert(
            YamlValue::Bool(true),
            YamlValue::String("value".to_string()),
        );

        let yaml = YamlValue::Mapping(mapping);
        let result = yaml_to_toml(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn yaml_to_toml_converts_null_to_none() {
        let yaml = YamlValue::Null;
        let result = yaml_to_toml(yaml).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn yaml_to_toml_converts_tagged_to_none() {
        let yaml = YamlValue::Tagged(Box::new(serde_yaml_ng::value::TaggedValue {
            tag: serde_yaml_ng::value::Tag::new("!!str"),
            value: YamlValue::String("test".to_string()),
        }));
        let result = yaml_to_toml(yaml).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn yaml_to_toml_filters_out_nulls_from_sequence() {
        let yaml = YamlValue::Sequence(vec![
            YamlValue::Number(YamlNumber::from(1)),
            YamlValue::Null,
            YamlValue::Number(YamlNumber::from(2)),
        ]);
        let result = yaml_to_toml(yaml).unwrap();

        let expected_array = vec![TomlValue::Integer(1), TomlValue::Integer(2)];

        assert_eq!(result, Some(TomlValue::Array(expected_array)));
    }

    #[test]
    fn yaml_to_toml_filters_out_nulls_from_mapping() {
        let mut mapping = Mapping::new();
        mapping.insert(
            YamlValue::String("valid".to_string()),
            YamlValue::Number(YamlNumber::from(1)),
        );
        mapping.insert(YamlValue::String("invalid".to_string()), YamlValue::Null);

        let yaml = YamlValue::Mapping(mapping);
        let result = yaml_to_toml(yaml).unwrap();

        let mut expected_map = Map::new();
        expected_map.insert("valid".to_string(), TomlValue::Integer(1));

        assert_eq!(result, Some(TomlValue::Table(expected_map)));
    }
}
