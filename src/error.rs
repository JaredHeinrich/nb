use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    AlreadyExists,
    NotFound,
    CommandNotHandled,
}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists => writeln!(f, "The note book already exists."),
            Self::NotFound => writeln!(f, "The note book does not exists."),
            Self::CommandNotHandled => writeln!(f, "Internal Error: Command not handled."),
        }
    }
}
