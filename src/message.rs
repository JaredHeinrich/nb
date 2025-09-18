use std::{fmt::Display};


#[derive(Debug)]
pub enum Message{
    ListOfTodoLists(Vec<String>),
    CreatedTodoList,
    EmptyMessage,
}
impl Display for Message{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreatedTodoList => {
                writeln!(f, "Created Todo List")
            }
            Self::ListOfTodoLists(file_names) => {
                for file_name in file_names {
                    writeln!(f, "{file_name}")?;
                }
                Ok(())
            }
            Self::EmptyMessage => write!(f, "")
        }
    }
}
