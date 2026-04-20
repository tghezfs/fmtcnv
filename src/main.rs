use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{ Error as IOError, ErrorKind, Write};
use std::path::Path;

use clap::Parser;

mod cli;
use cli::Args;

mod format;
use format::{Format, parse_format};

mod json_mapper;
use json_mapper::{ json_to_toml, json_to_yaml };

mod toml_mapper;
use toml_mapper::{ toml_to_json, toml_to_yaml };

mod yaml_mapper;
use yaml_mapper::{ yaml_to_toml, yaml_to_json };

mod fs_op;
use fs_op::get_out_path;

mod utils;

const VALID_FORMATS: [&str; 4] = ["json", "yaml", "yml", "toml"]; 

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

                match (in_format, out_format) {
                    (Format::Json, Format::Toml) => {
                        let parsed_content: serde_json::Value  = serde_json::from_str(&content)?;

                        if let Some(toml_tree) = json_to_toml(parsed_content)? {
                            let toml_string = toml::to_string(&toml_tree)?;
                            println!("toml string \n\n{}", toml_string);

                            let mut f = OpenOptions::new()
                                .write(true)
                                .create_new(true)
                                .open(out_path)?;

                            f.write_all(toml_string.as_bytes())?;
                        }
                    },
                    (Format::Json, Format::Yaml) => {
                        let parsed_content: serde_json::Value = serde_json::from_str(&content)?;

                        let yaml_tree = json_to_yaml(parsed_content)?;

                        let yaml_string = serde_yaml_ng::to_string(&yaml_tree)?;

                        println!("yaml string \n\n{}", yaml_string);

                        let mut f = OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(out_path)?;

                        f.write_all(yaml_string.as_bytes())?;
                    },
                    (Format::Toml, Format::Json) => {
                        let parsed_content: toml::Value = toml::from_str(&content)?;

                        let json_tree = toml_to_json(parsed_content)?;

                        let json_string = toml::to_string(&json_tree)?;

                        println!("yaml string \n\n{}", json_string);

                        let mut f = OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(out_path)?;

                        f.write_all(json_string.as_bytes())?;
                    },
                    (Format::Toml, Format::Yaml) => {
                        let parsed_content: toml::Value = toml::from_str(&content)?;

                        let yaml_tree = toml_to_yaml(parsed_content);

                        let yaml_string = serde_yaml_ng::to_string(&yaml_tree)?;

                        println!("yaml string \n\n{}", yaml_string);

                        let mut f = OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(out_path)?;

                        f.write_all(yaml_string.as_bytes())?;

                    },
                    (Format::Yaml, Format::Json) => {
                        let parsed_content: serde_yaml_ng::Value = serde_yaml_ng::from_str(&content)?;

                        if let Some(json_tree ) = yaml_to_json(parsed_content)? {
                            let json_string = toml::to_string(&json_tree)?;

                            println!("yaml string \n\n{}", json_string);

                            let mut f = OpenOptions::new()
                                .write(true)
                                .create_new(true)
                                .open(out_path)?;

                            f.write_all(json_string.as_bytes())?;
                        }
                    },
                    (Format::Yaml, Format::Toml) => {
                        let parsed_content: serde_yaml_ng::Value  = serde_yaml_ng::from_str(&content)?;

                        if let Some(toml_tree) = yaml_to_toml(parsed_content)? {
                            let toml_string = toml::to_string(&toml_tree)?;
                            println!("toml string \n\n{}", toml_string);

                            let mut f = OpenOptions::new()
                                .write(true)
                                .create_new(true)
                                .open(out_path)?;

                            f.write_all(toml_string.as_bytes())?;
                        }

                    },
                    (_, _) => {}
                }
            } else {
                todo!()
            }
        },
        None => { todo!() }
    }
    Ok(())
}
