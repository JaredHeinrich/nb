use std::{fmt::Display, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    AlreadyExists,
    NotFound,
    EditorNotInstalled(String),
    ConfigAlreadyExists(PathBuf),
    NoHomeDir,
}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists => writeln!(f, "The notebook already exists."),
            Self::NotFound => writeln!(f, "The notebook does not exists."),
            Self::EditorNotInstalled(editor) => {
                writeln!(f, "The configured editor \"{editor}\" is not installed.")
            }
            Self::ConfigAlreadyExists(path) => {
                writeln!(f, "A config file already exists {path:?}.")?;
                writeln!(f, "To overwrite it with the default use --force.")
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
