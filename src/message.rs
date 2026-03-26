use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum Message {
    ListOfNoteBooks(Vec<String>),
    CreatedNoteBook,
    DeletedNoteBook,
    CompletionScript(String),
    ConfigValues(Vec<(String, String)>),
    GeneratedConfig(PathBuf),
    EmptyMessage,
}
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreatedNoteBook => {
                writeln!(f, "Created note book")
            }
            Self::DeletedNoteBook => {
                writeln!(f, "Deleted note book")
            }
            Self::ListOfNoteBooks(file_names) => {
                for file_name in file_names {
                    writeln!(f, "{file_name}")?;
                }
                Ok(())
            }
            Self::CompletionScript(script) => writeln!(f, "{script}"),
            Self::GeneratedConfig(path) => writeln!(f, "Generated config_file {path:?}"),
            Self::ConfigValues(config_values) => {
                let col1_width = config_values
                    .iter()
                    .map(|(n, _)| n.len())
                    .max()
                    .unwrap_or(0);
                for (name, value) in config_values {
                    writeln!(f, "{:<width$} : {}", name, value, width = col1_width)?;
                }
                Ok(())
            }
            Self::EmptyMessage => write!(f, ""),
        }
    }
}
