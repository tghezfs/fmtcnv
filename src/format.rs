use std::error::Error;
use std::io::{ErrorKind, Error as IOError};
use std::path::Path;

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

pub fn get_format_by_path(path: &Path) -> Option<Format> {
    match path.extension().and_then(|s| s.to_str()) {
        Some("json") => Some(Format::Json),
        Some("toml") => Some(Format::Toml),
        Some("yaml") | Some("yml") => Some(Format::Yaml),
        _ => None
    }
}
