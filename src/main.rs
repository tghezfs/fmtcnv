use std::error::Error;
use std::io::{Error as IOError, ErrorKind, Write};
use std::path::Path;

use clap::Parser;
use tempfile::NamedTempFile;

mod cli;
use cli::Args;

mod format;
use format::{Format, get_format_by_path, parse_output_format};

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
        return Err(Box::new(IOError::new(
            ErrorKind::IsADirectory,
            "Expected a file, but got a directory.",
        )));
    }

    let in_filename = path
        .file_stem()
        .expect("File name must be valid at this point.")
        .to_string_lossy()
        .to_string();

    let full_out_name = format!("{}.{}", in_filename, out_format.ext());

    let out_path = get_out_path(&args.out_file, &full_out_name)?;

    if let Some(result_string) = convert(in_format, out_format, path)? {
        let out_dir = out_path.parent().expect("Out Dir must be valid at this point.");
        let mut tmp_file = NamedTempFile::new_in(out_dir)?;
        tmp_file.write_all(result_string.as_bytes())?;
        tmp_file.persist(out_path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    //use clap::Parser;
    use std::fs;

    #[test]
    fn main_errors_on_unknown_format_for_directory() {
        let dir = tempfile::tempdir().unwrap();
        let dir_path = dir.path().to_path_buf();

        let in_format = get_format_by_path(&dir_path);
        let out_format = Format::Json;

        let result = run_main_logic(&dir_path, in_format, out_format);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Expected a file, but got a directory"));
    }

    #[test]
    fn main_errors_if_output_path_generation_fails() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let file_path = file.path().to_path_buf();
        fs::write(&file_path, "{}").unwrap();

        let in_format = get_format_by_path(&file_path);
        let out_format = Format::Json;

        let result = run_main_logic(&file_path, in_format, out_format);

        assert!(result.is_err());
    }

    #[test]
    fn main_skips_file_creation_on_none_conversion() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let file_path = file.path().to_path_buf();
        fs::write(&file_path, "null").unwrap();

        let out_dir = tempfile::tempdir().unwrap();
        let out_path = out_dir.path().join("output.toml");

        let in_format = get_format_by_path(&file_path);
        let out_format = Format::Toml;

        let result = run_main_logic_with_out(&file_path, in_format, out_format, Some(out_path.clone()));

        assert!(result.is_ok());
        assert!(!out_path.exists());
    }

    #[test]
    fn main_creates_file_on_successful_conversion() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let file_path = file.path().to_path_buf();
        fs::write(&file_path, r#"{"key": "value"}"#).unwrap();

        let out_dir = tempfile::tempdir().unwrap();
        let out_path = out_dir.path().join("output.toml");

        let in_format = get_format_by_path(&file_path);
        let out_format = Format::Toml;

        let result = run_main_logic_with_out(&file_path, in_format, out_format, Some(out_path.clone()));

        assert!(result.is_ok());
        assert!(out_path.exists());
        
        let contents = fs::read_to_string(&out_path).unwrap();
        assert!(contents.contains("key = \"value\""));
    }

    fn run_main_logic(path: &Path, in_format: Format, out_format: Format) -> Result<(), Box<dyn Error>> {
        if in_format == Format::Unknown && path.metadata()?.is_dir() {
            return Err(Box::new(IOError::new(
                ErrorKind::IsADirectory,
                "Expected a file, but got a directory.",
            )));
        }

        let in_filename = path.file_name().unwrap().to_string_lossy().to_string();
        let out_path = get_out_path(&None, &in_filename)?;

        if let Some(result_string) = convert(in_format, out_format, path)? {
            let mut tmp_file = NamedTempFile::new()?;
            tmp_file.write_all(result_string.as_bytes())?;
            tmp_file.persist(out_path)?;
        }

        Ok(())
    }

    fn run_main_logic_with_out(path: &Path, in_format: Format, out_format: Format, forced_out: Option<std::path::PathBuf>) -> Result<(), Box<dyn Error>> {
        if in_format == Format::Unknown && path.metadata()?.is_dir() {
            return Err(Box::new(IOError::new(
                ErrorKind::IsADirectory,
                "Expected a file, but got a directory.",
            )));
        }

        let in_filename = path.file_name().unwrap().to_string_lossy().to_string();
        let out_path = match forced_out {
            Some(p) => p,
            None => get_out_path(&None, &in_filename)?
        };

        if let Some(result_string) = convert(in_format, out_format, path)? {
            let mut tmp_file = NamedTempFile::new()?;
            tmp_file.write_all(result_string.as_bytes())?;
            tmp_file.persist(out_path)?;
        }

        Ok(())
    }
}
