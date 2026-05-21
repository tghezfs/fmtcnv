use std::error::Error;
use std::io::{Error as IOError, ErrorKind};

use serde_json::Value as JsonValue;
use serde_yaml_ng::Value as YamlValue;
use toml::Value as TomlValue;

use serde_json::Map as JsonMap;
use serde_json::Number as JsonNumber;

use serde_yaml_ng::Mapping;
use serde_yaml_ng::Number as YamlNumber;

pub fn toml_to_json(value: TomlValue) -> Result<JsonValue, Box<dyn Error>> {
    match value {
        TomlValue::Boolean(val) => Ok(JsonValue::Bool(val)),
        TomlValue::Float(val) => {
            let number = JsonNumber::from_f64(val).ok_or(Box::new(IOError::new(
                ErrorKind::InvalidData,
                "Value not supported for a number in JSON format.!",
            )))?;

            Ok(JsonValue::Number(number))
        }
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
        }
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
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use toml::map::Map;
    use toml::value::Datetime;

    #[test]
    fn toml_to_json_converts_bool_true() {
        let toml = TomlValue::Boolean(true);
        let result = toml_to_json(toml).unwrap();
        assert_eq!(result, JsonValue::Bool(true));
    }

    #[test]
    fn toml_to_json_converts_bool_false() {
        let toml = TomlValue::Boolean(false);
        let result = toml_to_json(toml).unwrap();
        assert_eq!(result, JsonValue::Bool(false));
    }

    #[test]
    fn toml_to_json_converts_integer() {
        let toml = TomlValue::Integer(42);
        let result = toml_to_json(toml).unwrap();
        assert_eq!(result, JsonValue::Number(JsonNumber::from(42)));
    }

    #[test]
    fn toml_to_json_converts_float() {
        let toml = TomlValue::Float(3.14);
        let result = toml_to_json(toml).unwrap();
        assert_eq!(
            result,
            JsonValue::Number(JsonNumber::from_f64(3.14).unwrap())
        );
    }

    #[test]
    fn toml_to_json_fails_on_nan_float() {
        let toml = TomlValue::Float(f64::NAN);
        let result = toml_to_json(toml);
        assert!(result.is_err());
    }

    #[test]
    fn toml_to_json_fails_on_infinity_float() {
        let toml = TomlValue::Float(f64::INFINITY);
        let result = toml_to_json(toml);
        assert!(result.is_err());
    }

    #[test]
    fn toml_to_json_converts_string() {
        let toml = TomlValue::String("hello world".to_string());
        let result = toml_to_json(toml).unwrap();
        assert_eq!(result, JsonValue::String("hello world".to_string()));
    }

    #[test]
    fn toml_to_json_converts_datetime_to_string() {
        let dt = Datetime::from_str("2023-10-25T12:00:00Z").unwrap();
        let toml = TomlValue::Datetime(dt);
        let result = toml_to_json(toml).unwrap();
        assert_eq!(
            result,
            JsonValue::String("2023-10-25T12:00:00Z".to_string())
        );
    }

    #[test]
    fn toml_to_json_converts_empty_table() {
        let toml = TomlValue::Table(Map::new());
        let result = toml_to_json(toml).unwrap();
        assert_eq!(result, JsonValue::Object(JsonMap::new()));
    }

    #[test]
    fn toml_to_json_converts_nested_table() {
        let mut inner_map = Map::new();
        inner_map.insert("inner".to_string(), TomlValue::Integer(1));

        let mut outer_map = Map::new();
        outer_map.insert("key".to_string(), TomlValue::String("value".to_string()));
        outer_map.insert("nested".to_string(), TomlValue::Table(inner_map));

        let toml = TomlValue::Table(outer_map);
        let result = toml_to_json(toml).unwrap();

        let mut expected_inner_json = JsonMap::new();
        expected_inner_json.insert("inner".to_string(), JsonValue::Number(JsonNumber::from(1)));

        let mut expected_outer_json = JsonMap::new();
        expected_outer_json.insert("key".to_string(), JsonValue::String("value".to_string()));
        expected_outer_json.insert("nested".to_string(), JsonValue::Object(expected_inner_json));

        assert_eq!(result, JsonValue::Object(expected_outer_json));
    }

    #[test]
    fn toml_to_json_converts_empty_array() {
        let toml = TomlValue::Array(vec![]);
        let result = toml_to_json(toml).unwrap();
        assert_eq!(result, JsonValue::Array(vec![]));
    }

    #[test]
    fn toml_to_json_converts_array_with_multiple_types() {
        let toml = TomlValue::Array(vec![
            TomlValue::Integer(1),
            TomlValue::String("two".to_string()),
            TomlValue::Boolean(true),
            TomlValue::Float(3.0),
        ]);
        let result = toml_to_json(toml).unwrap();

        let expected_array = vec![
            JsonValue::Number(JsonNumber::from(1)),
            JsonValue::String("two".to_string()),
            JsonValue::Bool(true),
            JsonValue::Number(JsonNumber::from_f64(3.0).unwrap()),
        ];

        assert_eq!(result, JsonValue::Array(expected_array));
    }

    #[test]
    fn toml_to_yaml_converts_bool_true() {
        let toml = TomlValue::Boolean(true);
        let result = toml_to_yaml(toml);
        assert_eq!(result, YamlValue::Bool(true));
    }

    #[test]
    fn toml_to_yaml_converts_bool_false() {
        let toml = TomlValue::Boolean(false);
        let result = toml_to_yaml(toml);
        assert_eq!(result, YamlValue::Bool(false));
    }

    #[test]
    fn toml_to_yaml_converts_integer() {
        let toml = TomlValue::Integer(42);
        let result = toml_to_yaml(toml);
        assert_eq!(result, YamlValue::Number(YamlNumber::from(42)));
    }

    #[test]
    fn toml_to_yaml_converts_float() {
        let toml = TomlValue::Float(3.14);
        let result = toml_to_yaml(toml);
        assert_eq!(result, YamlValue::Number(YamlNumber::from(3.14)));
    }

    #[test]
    fn toml_to_yaml_converts_nan_float() {
        let toml = TomlValue::Float(f64::NAN);
        let result = toml_to_yaml(toml);
        assert_eq!(result, YamlValue::Number(YamlNumber::from(f64::NAN)));
    }

    #[test]
    fn toml_to_yaml_converts_string() {
        let toml = TomlValue::String("hello world".to_string());
        let result = toml_to_yaml(toml);
        assert_eq!(result, YamlValue::String("hello world".to_string()));
    }

    #[test]
    fn toml_to_yaml_converts_datetime_to_string() {
        let dt = Datetime::from_str("2023-10-25T12:00:00Z").unwrap();
        let toml = TomlValue::Datetime(dt);
        let result = toml_to_yaml(toml);
        assert_eq!(
            result,
            YamlValue::String("2023-10-25T12:00:00Z".to_string())
        );
    }

    #[test]
    fn toml_to_yaml_converts_empty_table() {
        let toml = TomlValue::Table(Map::new());
        let result = toml_to_yaml(toml);
        assert_eq!(result, YamlValue::Mapping(Mapping::new()));
    }

    #[test]
    fn toml_to_yaml_converts_nested_table() {
        let mut inner_map = Map::new();
        inner_map.insert("inner".to_string(), TomlValue::Boolean(false));

        let mut outer_map = Map::new();
        outer_map.insert("key".to_string(), TomlValue::Integer(1));
        outer_map.insert("nested".to_string(), TomlValue::Table(inner_map));

        let toml = TomlValue::Table(outer_map);
        let result = toml_to_yaml(toml);

        let mut expected_inner_mapping = Mapping::new();
        expected_inner_mapping.insert(
            YamlValue::String("inner".to_string()),
            YamlValue::Bool(false),
        );

        let mut expected_outer_mapping = Mapping::new();
        expected_outer_mapping.insert(
            YamlValue::String("key".to_string()),
            YamlValue::Number(YamlNumber::from(1)),
        );
        expected_outer_mapping.insert(
            YamlValue::String("nested".to_string()),
            YamlValue::Mapping(expected_inner_mapping),
        );

        assert_eq!(result, YamlValue::Mapping(expected_outer_mapping));
    }

    #[test]
    fn toml_to_yaml_converts_empty_array() {
        let toml = TomlValue::Array(vec![]);
        let result = toml_to_yaml(toml);
        assert_eq!(result, YamlValue::Sequence(vec![]));
    }

    #[test]
    fn toml_to_yaml_converts_array_with_multiple_types() {
        let toml = TomlValue::Array(vec![
            TomlValue::String("one".to_string()),
            TomlValue::Float(2.5),
            TomlValue::Boolean(true),
        ]);
        let result = toml_to_yaml(toml);

        let expected_sequence = vec![
            YamlValue::String("one".to_string()),
            YamlValue::Number(YamlNumber::from(2.5)),
            YamlValue::Bool(true),
        ];

        assert_eq!(result, YamlValue::Sequence(expected_sequence));
    }
}
