use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum Message {
    ListOfNoteBooks(Vec<String>),
    CreatedNoteBook,
    DeletedNoteBook,
    CompletionScript(String),
    ConfigValues(Vec<(String, String)>),
    GeneratedConfig(PathBuf),
    ArchivedNotebook((String, String)),
    RestoredNotebook((String, String)),
    Empty,
}
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreatedNoteBook => {
                writeln!(f, "Created notebook")
            }
            Self::DeletedNoteBook => {
                writeln!(f, "Deleted notebook")
            }
            Self::ListOfNoteBooks(file_names) => {
                for file_name in file_names {
                    writeln!(f, "{file_name}")?;
                }
                Ok(())
            }
            Self::CompletionScript(script) => writeln!(f, "{script}"),
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
            Self::GeneratedConfig(path) => writeln!(f, "Generated config_file {path:?}"),
            Self::ArchivedNotebook((original_name, archived_name)) => writeln!(f, "Archived notebook {original_name} to {archived_name}"),
            Self::RestoredNotebook((archived_name, new_name)) => writeln!(f, "Restored notebook {archived_name} to {new_name}"),
            Self::Empty => Ok(()),
        }
    }
}
