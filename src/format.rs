use std::error::Error;
use std::io::{Error as IOError, ErrorKind};
use std::path::Path;

#[derive(PartialEq, Debug)]
pub enum Format {
    Json,
    Yaml,
    Toml,
    Unknown,
}

impl Format {
    pub fn ext(&self) -> &str {
        match self {
            Format::Json => "json",
            Format::Yaml => "yaml",
            Format::Toml => "toml",
            Format::Unknown => "",
        }
    }
}

pub fn parse_output_format(format: &str) -> Result<Format, Box<dyn Error>> {
    match format.to_lowercase().as_str() {
        "json" => Ok(Format::Json),
        "yaml" | "yml" => Ok(Format::Yaml),
        "toml" => Ok(Format::Toml),
        _ => Err(Box::new(IOError::new(
            ErrorKind::InvalidInput,
            "Invalid Input Format",
        ))),
    }
}

pub fn get_format_by_path(path: &Path) -> Format {
    match path.extension().and_then(|s| s.to_str()) {
        Some("json") => Format::Json,
        Some("toml") => Format::Toml,
        Some("yaml") | Some("yml") => Format::Yaml,
        _ => Format::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use std::path::PathBuf;

    #[test]
    fn test_parse_json_lowercase() {
        assert_eq!(parse_output_format("json").unwrap(), Format::Json);
    }

    #[test]
    fn test_parse_json_uppercase() {
        assert_eq!(parse_output_format("JSON").unwrap(), Format::Json);
    }

    #[test]
    fn test_parse_json_mixed_case() {
        assert_eq!(parse_output_format("Json").unwrap(), Format::Json);
    }

    #[test]
    fn test_parse_yaml_lowercase() {
        assert_eq!(parse_output_format("yaml").unwrap(), Format::Yaml);
    }

    #[test]
    fn test_parse_yml_alias() {
        assert_eq!(parse_output_format("yml").unwrap(), Format::Yaml);
    }

    #[test]
    fn test_parse_yaml_uppercase() {
        assert_eq!(parse_output_format("YAML").unwrap(), Format::Yaml);
    }

    #[test]
    fn test_parse_toml_lowercase() {
        assert_eq!(parse_output_format("toml").unwrap(), Format::Toml);
    }

    #[test]
    fn test_parse_toml_uppercase() {
        assert_eq!(parse_output_format("TOML").unwrap(), Format::Toml);
    }

    #[test]
    fn test_parse_unknown_format() {
        let result = parse_output_format("xml");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_string() {
        let result = parse_output_format("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_whitespace_only() {
        let result = parse_output_format("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_spaces() {
        let result = parse_output_format(" json ");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_random_string() {
        let result = parse_output_format("foobar");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_format_by_path_json_extension() {
        assert_eq!(get_format_by_path(Path::new("config.json")), Format::Json);
    }

    #[test]
    fn test_get_format_by_path_toml_extension() {
        assert_eq!(get_format_by_path(Path::new("config.toml")), Format::Toml);
    }

    #[test]
    fn test_get_format_by_path_yaml_extension() {
        assert_eq!(get_format_by_path(Path::new("config.yaml")), Format::Yaml);
    }

    #[test]
    fn test_get_format_by_path_yml_extension() {
        assert_eq!(get_format_by_path(Path::new("config.yml")), Format::Yaml);
    }

    #[test]
    fn test_get_format_by_path_uppercase_extension() {
        assert_eq!(
            get_format_by_path(Path::new("config.JSON")),
            Format::Unknown
        );
    }

    #[test]
    fn test_get_format_by_path_no_extension() {
        assert_eq!(get_format_by_path(Path::new("config")), Format::Unknown);
    }

    #[test]
    fn test_get_format_by_path_empty_extension() {
        assert_eq!(get_format_by_path(Path::new("config.")), Format::Unknown);
    }

    #[test]
    fn test_get_format_by_path_unknown_extension() {
        assert_eq!(get_format_by_path(Path::new("config.xml")), Format::Unknown);
    }

    #[test]
    fn test_get_format_by_path_with_directories() {
        assert_eq!(
            get_format_by_path(Path::new("/home/user/config.json")),
            Format::Json
        );
    }

    #[test]
    fn test_get_format_by_path_multiple_dots() {
        assert_eq!(
            get_format_by_path(Path::new("archive.tar.gz")),
            Format::Unknown
        );
    }

    #[test]
    fn test_get_format_by_path_only_extension() {
        assert_eq!(get_format_by_path(Path::new(".json")), Format::Unknown);
    }

    #[test]
    fn test_get_format_by_path_empty_path() {
        assert_eq!(get_format_by_path(Path::new("")), Format::Unknown);
    }
}
