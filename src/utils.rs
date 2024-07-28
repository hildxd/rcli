use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(path);
    match path.exists() && path.is_dir() {
        true => Ok(path.into()),
        false => Err("Path does not exist"),
    }
}
