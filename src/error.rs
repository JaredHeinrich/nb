use std::{fmt::Display, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    AlreadyExists,
    NotFound,
    CommandNotHandled,
    EditorNotInstalled(String),
    UnknownShell,
}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists => writeln!(f, "The note book already exists."),
            Self::NotFound => writeln!(f, "The note book does not exists."),
            Self::CommandNotHandled => writeln!(f, "Internal Error: Command not handled."),
            Self::EditorNotInstalled(editor) => {
                writeln!(f, "The configured editor \"{editor}\" is not installed.")
            }
            Self::UnknownShell => writeln!(f, "Shell not recognized."),
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
