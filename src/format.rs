use std::error::Error;
use std::io::{ErrorKind, Error as IOError};
use std::path::Path;

#[derive(PartialEq, Debug)]
pub enum Format {
    Json,
    Yaml,
    Toml,
    Unknown
}

pub fn parse_output_format(format: &str) -> Result<Format, Box<dyn Error>> {
    match format.to_lowercase().as_str() {
        "json" => Ok(Format::Json),
        "yaml" | "yml" => Ok(Format::Yaml),
        "toml" => Ok(Format::Toml),
        _ => Err(Box::new(IOError::new(
                    ErrorKind::InvalidInput, "Invalid Input Format!")))
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
