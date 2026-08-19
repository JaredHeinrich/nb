use std::{ffi::OsString, fmt::Display, path::PathBuf};

use anyhow::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    AlreadyExists,
    NotFound,
    ConfigAlreadyExists(PathBuf),
}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists => writeln!(f, "The notebook already exists."),
            Self::NotFound => writeln!(f, "The notebook does not exists."),
            Self::ConfigAlreadyExists(path) => {
                writeln!(f, "A config file already exists {}.", path.display())?;
                writeln!(f, "To overwrite it with the default use --force.")
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum SystemError {
    CommandNotInstalled(String),
    NoHomeDir,
}

impl Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandNotInstalled(command) => {
                writeln!(f, "The command \"{command}\" is not installed.")
            }
            Self::NoHomeDir => writeln!(f, "No home directory could be found"),
        }
    }
}

#[derive(Error, Debug)]
pub enum FileSystemError {
    NotAFile(PathBuf),
    FileNameNoUTF8(OsString),
}

impl Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotAFile(path) => writeln!(f, "\"{}\" is not a file.", path.display()),
            #[allow(clippy::unnecessary_debug_formatting)]
            Self::FileNameNoUTF8(file_name) => {
                writeln!(f, "File name {file_name:?} is no valid UTF-8.")
            }
        }
    }
}

#[derive(Error, Debug)]
pub struct InternalError<E = Error>(pub E);

impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Internal Error, you may open an issue on Github: \n {}",
            self.0
        )
    }
}
