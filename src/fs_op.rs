use std::{env, fs};
use std::path::{Path, PathBuf};
use std::error::Error;

pub fn get_out_path(path: &Option<String>, default_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    match path {
        Some(out_path) => {
            let o_path = Path::new(out_path);

            let final_path = if o_path.file_name().is_some() {
                o_path.to_path_buf()
            } else {
                o_path.join(default_name)
            };

            if let Some(parent) = o_path.parent() {
                fs::create_dir_all(parent)?;
            }

            Ok(final_path)
        },
        None => {
            let cwd = env::current_dir()?;
            Ok(cwd.join(default_name))
        }
    }
}
