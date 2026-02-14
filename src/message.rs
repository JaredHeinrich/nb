use std::fmt::Display;

#[derive(Debug)]
pub enum Message {
    ListOfNoteBooks(Vec<String>),
    CreatedNoteBook,
    DeletedNoteBook,
    CompletionScript(String),
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
            Self::CompletionScript(s) => {
                writeln!(f, "To activate completions execute:")?;
                writeln!(f, "{s}")
            },
            Self::EmptyMessage => write!(f, ""),
        }
    }
}
