use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum Message {
    Notebook(Vec<String>),
    Archive(Vec<String>),
    CreatedNote,
    DeletedNote,
    CompletionScript(String),
    ConfigValues(Vec<(String, String)>),
    GeneratedConfig(PathBuf),
    ArchivedNote((String, String)),
    RestoredNote((String, String)),
    Empty,
}
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreatedNote => {
                writeln!(f, "Created note")
            }
            Self::DeletedNote => {
                writeln!(f, "Deleted note")
            }
            Self::Notebook(notes) => {
                for name in notes {
                    writeln!(f, "{name}")?;
                }
                Ok(())
            }
            Self::Archive(notes) => {
                for name in notes {
                    writeln!(f, "{name}")?;
                }
                Ok(())
            }
            Self::CompletionScript(script) => writeln!(f, "{script}"),
            Self::ConfigValues(config_values) => {
                let name_col_width = config_values
                    .iter()
                    .map(|(n, _)| n.len())
                    .max()
                    .unwrap_or(0);
                for (name, value) in config_values {
                    writeln!(f, "{name:<name_col_width$} : {value}")?;
                }
                Ok(())
            }
            Self::GeneratedConfig(path) => writeln!(f, "Generated config file {}", path.display()),
            Self::ArchivedNote((original_name, archived_name)) => {
                writeln!(f, "Archived note {original_name} to {archived_name}")
            }
            Self::RestoredNote((archived_name, new_name)) => {
                writeln!(f, "Restored note {archived_name} to {new_name}")
            }
            Self::Empty => Ok(()),
        }
    }
}
