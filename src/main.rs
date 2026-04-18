use std::error::Error;
use std::fs;
use std::io::{Error as IOError, ErrorKind};
use std::path::Path;

use clap::Parser;

mod cli;
use cli::Args;

mod parser;
use parser::{Format, parse_format, json_to_toml};

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

    let _out_path = Path::new(&args.out_file);

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
                                if let Some(toml_tree) = json_to_toml(parsed_content) {
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
