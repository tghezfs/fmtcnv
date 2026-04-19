use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Error as IOError, ErrorKind, Write};
use std::path::Path;

use clap::Parser;

mod cli;
use cli::Args;

mod parser;
use parser::{Format, parse_format, json_to_toml};

mod fs_op;
use fs_op::get_out_path;

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

                        if let Some(toml_tree) = json_to_toml(parsed_content) {
                            let toml_string = toml::to_string(&toml_tree)?;
                            println!("toml string \n\n{}", toml_string);

                            let mut f = OpenOptions::new()
                                .write(true)
                                .create_new(true)
                                .open(out_path)?;

                            f.write_all(toml_string.as_bytes())?;
                        }
                    },
                    (Format::Json, Format::Yaml) => {},
                    (Format::Toml, Format::Json) => {},
                    (Format::Toml, Format::Yaml) => {},
                    (Format::Yaml, Format::Json) => {},
                    (Format::Yaml, Format::Toml) => {},
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
