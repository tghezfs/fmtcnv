use std::error::Error;
use std::io::{ Error as IOError, ErrorKind, Write};
use std::path::Path;

use clap::Parser;
use tempfile::NamedTempFile;

mod cli;
use cli::Args;

mod format;
use format::{Format, parse_output_format, get_format_by_path };

mod converter;
use converter::convert;

mod mapper;

mod fs_op;
use fs_op::get_out_path;

mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let out_format: Format = parse_output_format(&args.to_format)?;
    let path = Path::new(&args.file);

    let in_format = get_format_by_path(path);

    if in_format == Format::Unknown && path.metadata()?.is_dir() {
        return Err(Box::new(IOError::new(ErrorKind::IsADirectory, "Expected a file, but got a directory.")))
    }
    
    let in_filename = path
        .file_name()
        .expect("File name must be valid at this point!")
        .to_string_lossy()
        .to_string();

    let out_path = get_out_path(&args.out_file, &in_filename)?;

    println!("out_path = {:?}", out_path);
    if let Some(result_string) = convert(in_format, out_format, path)? {
        let mut tmp_file = NamedTempFile::new()?;
        tmp_file.write_all(result_string.as_bytes())?;
        tmp_file.persist(out_path)?;
    }

    Ok(())
}
