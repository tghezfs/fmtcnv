use std::{env, fs};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::io::{ Error as IOError, ErrorKind };

use crate::utils::is_valid_path_str;

pub fn get_out_path(path: &Option<String>, default_name: &str) -> Result<PathBuf, Box<dyn Error>> {
     let final_path = match path {
        Some(out_path) => {

            if !is_valid_path_str(out_path) {
                return Err(Box::new(IOError::new( ErrorKind::InvalidInput, "The output path or file must have valid name." )));
            }

            let o_path = Path::new(out_path);

            let final_path = if out_path.ends_with("/") {
                o_path.join(default_name)
            } else {
                o_path.to_path_buf()
            };

            if let Some(parent) = final_path.parent() {
                fs::create_dir_all(parent)?;
            }

            final_path
        },
        None => {
            let cwd = env::current_dir()?;
            cwd.join(default_name)
        }
    };
    
    if final_path.is_file() {
        return Err(Box::new(IOError::new(ErrorKind::AlreadyExists, "The output path already contains a file with the same name.")))
    }

    Ok(final_path)

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_with_some_path_with_filename() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("output.txt");
        let path_str = file_path.to_str().unwrap().to_string();
        
        let result = get_out_path(&Some(path_str), "default.txt").unwrap();
        
        assert_eq!(result, file_path);
    }

    #[test]
    fn test_with_some_path_without_filename() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().to_str().unwrap().to_string() + "/";
        
        let result = get_out_path(&Some(dir_path.clone()), "default.txt").unwrap();
        
        assert_eq!(result, Path::new(&dir_path).join("default.txt"));
    }

    #[test]
    fn test_with_none_path() {
        let cwd = env::current_dir().unwrap();
        
        let result = get_out_path(&None, "default.txt").unwrap();
        
        assert_eq!(result, cwd.join("default.txt"));
    }

    #[test]
    fn test_creates_parent_directories() {
        let temp_dir = TempDir::new().unwrap();
        let nested_dir = temp_dir.path().join("subdir1").join("subdir2");
        let file_path = nested_dir.join("output.txt");
        let path_str = file_path.to_str().unwrap().to_string();
        
        assert!(!nested_dir.exists());
        
        let result = get_out_path(&Some(path_str), "default.txt").unwrap();
        
        assert_eq!(result, file_path);
        assert!(nested_dir.exists());
    }

    #[test]
    fn test_with_existing_directory_no_filename() {
        let temp_dir = TempDir::new().unwrap();
        let existing_dir = temp_dir.path().join("existing");
        fs::create_dir_all(&existing_dir).unwrap();
        let dir_path = existing_dir.to_str().unwrap().to_string() + "/";
        
        let result = get_out_path(&Some(dir_path), "output.txt").unwrap();
        
        assert_eq!(result, existing_dir.join("output.txt"));
    }

    #[test]
    fn test_with_invalid_path_characters() {
        let result = get_out_path(&Some("\0invalid".to_string()), "default.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_with_different_default_names() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().to_str().unwrap().to_string() + "/";
        
        let result = get_out_path(&Some(dir_path), "custom.json").unwrap(); 
        assert_eq!(result, temp_dir.path().join("custom.json"));
    }

    #[test]
    fn test_with_deeply_nested_nonexistent_directories() {
        let temp_dir = TempDir::new().unwrap();
        let deep_nested = temp_dir.path().join("a").join("b").join("c").join("d");
        let file_path = deep_nested.join("result.txt");
        let path_str = file_path.to_str().unwrap().to_string();
        
        assert!(!deep_nested.exists());
        
        let result = get_out_path(&Some(path_str), "default.txt").unwrap();
        
        assert_eq!(result, file_path);
        assert!(deep_nested.exists());
    }


    #[test]
    fn test_with_only_filename_no_directory() {
        let result = get_out_path(&Some("just_file.txt".to_string()), "default.txt").unwrap();

        assert_eq!(result, PathBuf::from("just_file.txt"));
    }

    #[test]
    fn test_none_with_different_default_names() {
        let cwd = env::current_dir().unwrap();
        
        let result = get_out_path(&None, "backup.bak").unwrap();
        
        assert_eq!(result, cwd.join("backup.bak"));
    }

    #[test]
    fn test_creates_directory_when_trailing_slash() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().join("newdir");
        let path_str = dir_path.to_str().unwrap().to_string() + "/";
        
        assert!(!dir_path.exists());
        
        let result = get_out_path(&Some(path_str), "output.txt").unwrap();
        
        assert_eq!(result, dir_path.join("output.txt"));
        assert!(dir_path.exists());
    }

    #[test]
    fn test_rejects_existing_file_as_output() {
        let temp_dir = TempDir::new().unwrap();
        let existing_file = temp_dir.path().join("existing.txt");
        fs::write(&existing_file, "content").unwrap();
        let path_str = existing_file.to_str().unwrap().to_string();
        
        let result = get_out_path(&Some(path_str), "default.txt");
        
        assert!(result.is_err());
    }

    #[test]
    fn test_rejects_path_with_existing_file_in_directory_position() {
        let temp_dir = TempDir::new().unwrap();
        let existing_file = temp_dir.path().join("isfile");
        fs::write(&existing_file, "content").unwrap();
        let path_str = existing_file.to_str().unwrap().to_string() + "/";
        
        let result = get_out_path(&Some(path_str), "output.txt");
        
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_path_with_spaces() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("my file.txt");
        let path_str = file_path.to_str().unwrap().to_string();
        
        let result = get_out_path(&Some(path_str), "default.txt").unwrap();
        
        assert_eq!(result, file_path);
    }

    #[test]
    fn test_valid_path_with_unicode() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("año_reporte.txt");
        let path_str = file_path.to_str().unwrap().to_string();
        
        let result = get_out_path(&Some(path_str), "default.txt").unwrap();
        
        assert_eq!(result, file_path);
    }

    #[test]
    fn test_rejects_path_with_tab() {
        let result = get_out_path(&Some("file\tname.txt".to_string()), "default.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_rejects_path_with_newline() {
        let result = get_out_path(&Some("file\nname.txt".to_string()), "default.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_rejects_path_with_carriage_return() {
        let result = get_out_path(&Some("file\rname.txt".to_string()), "default.txt");
        assert!(result.is_err());
    }
}
