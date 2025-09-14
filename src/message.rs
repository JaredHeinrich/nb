use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum Message{
    NotebookDirNotExisting(PathBuf),
    ListOfNotebooks(Vec<String>),
    CreatedNotebook(String),
    NotebookAlreadyExists(String),
    MissingSubcommand,
}
impl Display for Message{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ListOfNotebooks(list) => {
                todo!("not implemented")
            }
            _ => todo!("message display not implemented")
            
        }
    }
}
