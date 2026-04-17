use std::error::Error;
use std::fs;
use std::io::{Error as IOError, ErrorKind};
use std::path::Path;
use std::str::FromStr;

use chrono::{DateTime, NaiveDate, NaiveDateTime};
use clap::Parser;

mod cli;
use cli::Args;

mod parser;
use parser::{Format, parse_format};
use toml::map::Map;

const VALID_FORMATS: [&str; 4] = ["json", "yaml", "yml", "toml"]; 

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    println!("args = {:?}", args);
    let out_format = parse_format(&args.to_format)?;

    let path = Path::new(&args.file);

    if path.metadata()?.is_dir() {
        return Err( Box::new(IOError::new(ErrorKind::IsADirectory, "Cannot parse or read a dir")) );
    }

    let abs_path = fs::canonicalize(path)?;

    match abs_path.extension() {
        Some(ext) => {
            let extension = ext
                .to_string_lossy()
                .to_string()
                .to_lowercase();

            if VALID_FORMATS.contains(&extension.as_str()) {
                let content = fs::read_to_string(abs_path)?;

                let in_format = parse_format(&extension)
                    .expect("Input Format must be valid at this point!");

                match in_format {
                    Format::Json => {
                        let parsed_content: serde_json::Value  = serde_json::from_str(&content)?;

                        match out_format {
                            Format::Toml => {
                                let toml_result = process_parsed_json_to_toml(parsed_content);
                                if let Some(toml_tree) = toml_result {
                                    let toml_string = toml::to_string(&toml_tree)?;
                                    println!("toml string \n\n{}", toml_string);
                                }
                            },
                            Format::Yaml => {}
                            _ => {}
                        }
                    }
                    Format::Yaml => {
                        let parsed_content: serde_yaml_ng::Value = serde_yaml_ng::from_str(&content)?;
                        println!("yaml parsed = {parsed_content:?}");
                    },
                    Format::Toml => {
                        let parsed_content: toml::Value = toml::from_str(&content)?;
                        println!("toml parsed = {parsed_content:?}");
                    }
                };

                match out_format {
                    Format::Json => {},
                    Format::Yaml => {},
                    Format::Toml => {}
                }
            } else {
                todo!()
            }
        },
        None => { todo!() }
    }
    Ok(())

}

fn process_parsed_json_to_toml(value: serde_json::Value) -> Option<toml::Value> {
    match value {
        serde_json::Value::Bool(b) => Some(toml::Value::Boolean(b)),
        serde_json::Value::Number(n) => {
            if n.is_f64() {
                Some(toml::Value::Float(n.as_f64().unwrap()))
            } else {
                Some(toml::Value::Integer(n.as_i64().unwrap()))
            }
        },
        serde_json::Value::String(s) => {
            if DateTime::parse_from_rfc3339(&s).is_ok() || 
                NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S").is_ok() ||
                    NaiveDate::parse_from_str(&s, "%Y-%m-%d").is_ok() {
                Some(toml::Value::Datetime(toml::value::Datetime::from_str(&s).unwrap()))
            } else {
                Some(toml::Value::String(s))
            }
        },
        serde_json::Value::Object(map) => {
            let mut current_map: Map<String, toml::Value> = Map::new();
            for (k, v) in map {
                if let Some(toml_value) = process_parsed_json_to_toml(v) {
                    //root_table.insert(k, toml_value);
                    current_map.insert(k, toml_value);
                }
            }
            Some(toml::Value::Table(current_map))
        },
        serde_json::Value::Array(arr ) => {
            let mut values: Vec<toml::Value> = Vec::new();
            for value in arr {
                if let Some(toml_value) = process_parsed_json_to_toml(value) {
                    values.push(toml_value)
                }
            }
            Some(toml::Value::Array(values))
        },
        serde_json::Value::Null => None
    }
}
