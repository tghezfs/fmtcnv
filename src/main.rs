use std::error::Error;
use std::fs;
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
    println!("args = {:?}", args);
    let out_format: Format = parse_output_format(&args.to_format)?;

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
    let in_format: Format = get_format_by_path(&abs_path);

    if let Some(result_string) = convert(in_format, out_format, &abs_path)? {
        tmp_file.write_all(result_string.as_bytes())?;
        tmp_file.persist(out_path)?;
    }

    Ok(())
}
