use std::{fmt::Display, path::PathBuf};

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
                writeln!(f, "A config file already exists {path:?}.")?;
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
}

impl Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotAFile(path) => writeln!(f, "\"{path:?}\" is not a file."),
        }
    }
}
