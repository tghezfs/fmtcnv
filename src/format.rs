use std::error::Error;
use std::io::{ErrorKind, Error as IOError};

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
