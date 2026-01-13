use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreationError {
    AlreadyExists,
}
impl Display for CreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists => writeln!(f, "The note book already exists."),
        }
    }
}

#[derive(Error, Debug)]
pub enum OpenError {
    NotFound,
}
impl Display for OpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => writeln!(f, "The note book does not exists."),
        }
    }
}
