use thiserror::Error;

use std::path::PathBuf;
use std::{fs, io};

use crate::cli::error::CliError;

pub fn read_file(file_path: &str) -> Result<String, CliError> {
    let result = fs::read_to_string(file_path)
        .map_err(|e| FindFileError::FileNotFound(file_path.to_string(), e.to_string()))?;
    Ok(result)
}

///
/// Tries to create the directory. If the directory already exist,
/// it will return Ok(()). If the creation fails, it'll return an error
pub fn try_create_dir(path: &PathBuf) -> Result<(), io::Error> {
    // Try to create the directory
    std::fs::create_dir_all(path)
}

///
/// Errors during the binary file search
#[derive(Error, Debug)]
pub enum FindFileError {
    #[error("Executable directory was not found. Reason {}", .0)]
    ExecutableDirectoryNotFound(String),
    #[error("We couldn't read the file: {}. Reason: {}", .0, .1)]
    FileNotFound(String, String),
}
