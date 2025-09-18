use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreationError {
    TodoListAlreadyExists
}
impl Display for CreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TodoListAlreadyExists => writeln!(f, "The Todo List already exists.")
        }
    }
}

#[derive(Error, Debug)]
pub enum OpenError {
    TodoListNotFound
}
impl Display for OpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TodoListNotFound => writeln!(f, "The Todo List does not exists.")
        }
    }
}
