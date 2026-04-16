use std::error::Error;
use std::fs;
use std::io::{Error as IOError, ErrorKind};
use std::path::Path;

mod cli;
use clap::Parser;
use cli::Args;

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
                let _content = fs::read_to_string(abs_path)?;

                let in_format = parse_format(&extension)
                    .expect("Input Format must be valid at this point!");

                match in_format {
                    Format::Json => {},
                    Format::Yaml => {},
                    Format::Toml => {}
                }

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

enum Format {
    Json,
    Yaml,
    Toml
}

fn parse_format(format: &str) -> Result<Format, Box<dyn Error>> {
    match format.to_lowercase().as_str() {
        "json" => Ok(Format::Json),
        "yaml" | "yml" => Ok(Format::Yaml),
        "toml" => Ok(Format::Toml),
        _ => Err(Box::new(IOError::new(
                    ErrorKind::InvalidInput, "Invalid Format!")))
    }
}
