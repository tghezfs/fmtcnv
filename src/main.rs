use std::error::Error;
use std::fs;
use std::io::{ Error as IOError, ErrorKind, Write};
use std::path::Path;

use clap::Parser;

use serde_json::Value as JsonValue;
use toml::Value as TomlValue;
use serde_yaml_ng::Value as YamlValue;

use tempfile::NamedTempFile;

mod cli;
use cli::Args;

mod constants;
use constants::VALID_FORMATS;

mod format;
use format::{Format, parse_format};

mod mapper;
use mapper::json::{ json_to_toml, json_to_yaml };
use mapper::toml::{ toml_to_json, toml_to_yaml };
use mapper::yaml::{ yaml_to_toml, yaml_to_json };

mod fs_op;
use fs_op::get_out_path;

mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    println!("args = {:?}", args);
    let out_format = parse_format(&args.to_format)?;

    let path = Path::new(&args.file);

    let abs_path = fs::canonicalize(path)?;

    if abs_path.metadata()?.is_dir() {
        return Err( Box::new(IOError::new(ErrorKind::IsADirectory, "Cannot parse or read a dir")) );
    }

    let in_filename = path
        .file_name()
        .expect("File name must be valid at this point!")
        .to_string_lossy()
        .to_string();


    let out_path = get_out_path(&args.out_file, &in_filename)?;

    let mut tmp_file = NamedTempFile::new()?;

    match abs_path.extension() {
        Some(ext) => {
            let extension = ext
                .to_string_lossy()
                .to_string()
                .to_lowercase();

            if !VALID_FORMATS.contains(&extension.as_str())      {
                let error = IOError::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid file format: {in_filename}")
                );

                return Err(Box::new(error));
            }

            let content = fs::read_to_string(abs_path)?;

            let in_format = parse_format(&extension)
                .expect("Input Format must be valid at this point!");

            match (in_format, out_format) {
                (Format::Json, Format::Toml) => {
                    let parsed_content: JsonValue = serde_json::from_str(&content)?;

                    if let Some(toml_tree) = json_to_toml(parsed_content)? {
                        let toml_string = toml::to_string(&toml_tree)?;
                        println!("toml string \n\n{}", toml_string);
                        tmp_file.write_all(toml_string.as_bytes())?;
                        tmp_file.persist(out_path)?;
                    } else {
                        eprintln!("⚠️ Warning: There was a format incompatibility.\n\
                                    Some data could not be converted correctly.")
                    }
                },
                (Format::Json, Format::Yaml) => {
                    let parsed_content: JsonValue = serde_json::from_str(&content)?;

                    let yaml_tree = json_to_yaml(parsed_content)?;

                    let yaml_string = serde_yaml_ng::to_string(&yaml_tree)?;

                    println!("yaml string \n\n{}", yaml_string);

                    tmp_file.write_all(yaml_string.as_bytes())?;
                    tmp_file.persist(out_path)?;
                },
                (Format::Toml, Format::Json) => {
                    let parsed_content: TomlValue = toml::from_str(&content)?;

                    let json_tree = toml_to_json(parsed_content)?;

                    let json_string = toml::to_string(&json_tree)?;

                    println!("yaml string \n\n{}", json_string);
                    tmp_file.write_all(json_string.as_bytes())?;
                    tmp_file.persist(out_path)?;
                },
                (Format::Toml, Format::Yaml) => {
                    let parsed_content: TomlValue = toml::from_str(&content)?;

                    let yaml_tree = toml_to_yaml(parsed_content);

                    let yaml_string = serde_yaml_ng::to_string(&yaml_tree)?;

                    println!("yaml string \n\n{}", yaml_string);

                    tmp_file.write_all(yaml_string.as_bytes())?;
                    tmp_file.persist(out_path)?;
                },
                (Format::Yaml, Format::Json) => {
                    let parsed_content: YamlValue = serde_yaml_ng::from_str(&content)?;

                    if let Some(json_tree ) = yaml_to_json(parsed_content)? {
                        let json_string = toml::to_string(&json_tree)?;

                        println!("yaml string \n\n{}", json_string);

                        tmp_file.write_all(json_string.as_bytes())?;
                        tmp_file.persist(out_path)?;
                    } else {
                        eprintln!("⚠️ Warning: There was a format incompatibility.\n\
                                    Some data could not be converted correctly.")
                    }
                },
                (Format::Yaml, Format::Toml) => {
                    let parsed_content: YamlValue = serde_yaml_ng::from_str(&content)?;

                    if let Some(toml_tree) = yaml_to_toml(parsed_content)? {
                        let toml_string = toml::to_string(&toml_tree)?;
                        println!("toml string \n\n{}", toml_string);

                        tmp_file.write_all(toml_string.as_bytes())?;
                        tmp_file.persist(out_path)?;
                    } else {
                        eprintln!("⚠️ Warning: There was a format incompatibility.\n\
                                    Some data could not be converted correctly.")
                    }

                },
                (_, _) => {
                    tmp_file.write_all(content.as_bytes())?;
                    tmp_file.persist(out_path)?;
                }
            }
        },
        None => {
            let content = fs::read_to_string(abs_path)?;

            match serde_json::from_str::<JsonValue>(&content) {
                Ok(parsed_content) => {
                    match out_format {
                        Format::Toml => {
                            if let Some(toml_tree) = json_to_toml(parsed_content)? {
                                let toml_string = toml::to_string(&toml_tree)?;
                                println!("toml string \n\n{}", toml_string);

                                tmp_file.write_all(toml_string.as_bytes())?;
                                tmp_file.persist(out_path)?;

                                return Ok(())
                            } else {
                                eprintln!("⚠️ Warning: There was a format incompatibility.\n\
                                    Some data could not be converted correctly.")
                            }
                        },
                        Format::Yaml => {
                            let yaml_tree = json_to_yaml(parsed_content)?;

                            let yaml_string = serde_yaml_ng::to_string(&yaml_tree)?;

                            println!("yaml string \n\n{}", yaml_string);

                            tmp_file.write_all(yaml_string.as_bytes())?;
                            tmp_file.persist(out_path)?;
                            return Ok(());
                        },
                        Format::Json => {
                            tmp_file.write_all(content.as_bytes())?;
                            tmp_file.persist(out_path)?;
                            return Ok(())   
                        }
                    }
                },
                _ => {} 
            }

            match toml::from_str::<TomlValue>(&content) {
                Ok(parsed_content) => {
                    match out_format {
                        Format::Json => {
                            let json_tree = toml_to_json(parsed_content)?;

                            let json_string = toml::to_string(&json_tree)?;

                            println!("yaml string \n\n{}", json_string);

                            tmp_file.write_all(json_string.as_bytes())?;
                            tmp_file.persist(out_path)?;

                            return Ok(());
                        },
                        Format::Yaml => {
                            let yaml_tree = toml_to_yaml(parsed_content);

                            let yaml_string = serde_yaml_ng::to_string(&yaml_tree)?;

                            println!("yaml string \n\n{}", yaml_string);

                            tmp_file.write_all(yaml_string.as_bytes())?;
                            tmp_file.persist(out_path)?;

                            return Ok(());
                        },
                        Format::Toml => {
                            tmp_file.write_all(content.as_bytes())?;
                            tmp_file.persist(out_path)?;
                            return Ok(())   
                        }
                    }
                },
                _ => {}
            }

            match serde_yaml_ng::from_str::<YamlValue>(&content) {
                Ok(parsed_content) => {
                    match out_format {
                        Format::Json => {
                            if let Some(json_tree ) = yaml_to_json(parsed_content)? {
                                let json_string = toml::to_string(&json_tree)?;

                                println!("yaml string \n\n{}", json_string);
                                tmp_file.write_all(json_string.as_bytes())?;
                                tmp_file.persist(out_path)?;
                                return Ok(())
                            } else {
                                eprintln!("⚠️ Warning: There was a format incompatibility.\n\
                                    Some data could not be converted correctly.")
                            }
                        },
                        Format::Toml => {
                            if let Some(toml_tree) = yaml_to_toml(parsed_content)? {
                                let toml_string = toml::to_string(&toml_tree)?;
                                println!("toml string \n\n{}", toml_string);

                                tmp_file.write_all(toml_string.as_bytes())?;
                                tmp_file.persist(out_path)?; 
                                return Ok(());
                            } else {
                                eprintln!("⚠️ Warning: There was a format incompatibility.\n\
                                    Some data could not be converted correctly.")
                            }
                        },
                        Format::Yaml => {
                            tmp_file.write_all(content.as_bytes())?;
                            tmp_file.persist(out_path)?;
                            return Ok(())   
                        }
                    }
                },
                _ => {}
            }

            let error = IOError::new(
                ErrorKind::InvalidInput,
                format!("Invalid file format {in_filename}")
            );

            return Err(Box::new(error));
        }
    }
    Ok(())
}
