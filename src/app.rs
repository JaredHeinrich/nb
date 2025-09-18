use std::fs;
use std::path::PathBuf;

use anyhow::{Ok, Result};
use clap::{Arg, ArgMatches, Command};

use crate::error::{CreationError, OpenError};
use crate::{config, file_operations};
use crate::message::Message;

pub struct App{
    config: config::Config
}

impl App{
    pub fn new(config: config::Config) -> Self{
        App { config }
    }

    pub fn create_todo_list_dir(&self) -> Result<()> {
        if fs::exists(&self.config.todo_list_dir)? == false {
            file_operations::create_dir(&self.config.todo_list_dir)?;
        }
        Ok(())
    }

    fn create_todo_list(&self, name: &str) -> Result<Message> {
        let mut todo_list_path: PathBuf = self.config.todo_list_dir.to_owned();
        todo_list_path.push(&name);
        if fs::exists(&todo_list_path)? {
            Err(CreationError::TodoListAlreadyExists)?;
        }
        file_operations::create_file(&todo_list_path)?;
        Ok(Message::CreatedTodoList)
    }

    fn list_todo_lists(&self) -> Result<Message> {
        let todo_list_dir = &self.config.todo_list_dir;
        let files = file_operations::get_files(todo_list_dir)?;
        Ok(Message::ListOfTodoLists(files))
    }

    fn open_todo_list(&self, name: &str) -> Result<Message> {
        let mut todo_list_path: PathBuf = self.config.todo_list_dir.to_owned();
        todo_list_path.push(&name);
        if !fs::exists(&todo_list_path)? {
            Err(OpenError::TodoListNotFound)?;
        }
        file_operations::open_file(&self.config.editor, &todo_list_path)?;
        Ok(Message::EmptyMessage)
    }


    pub fn handle_command(&self, matches: ArgMatches) -> Result<Message> {
        self.create_todo_list_dir()?;

        match matches.subcommand() {
            Some(("list", _sub_matches)) => self.list_todo_lists(),
            Some(("new", sub_matches)) => {
                let name = sub_matches.get_one::<String>("file_name").unwrap();
                self.create_todo_list(name)
            },
            _ => {
                let name = matches.get_one::<String>("file_name").unwrap();
                self.open_todo_list(name)
            }
        }
    }

    pub fn get_command(&self) -> Command {
        Command::new("todo")
            .version("1.0.0")
            .about("CLI todo list manager")
            .arg(Arg::new("file_name")
                .value_name("FILE_NAME")
                .help("Name of the todo list to open")
                .default_value(self.config.main_file.clone())
            )
            .subcommand(Command::new("new")
                .about("Create a new todo list")
                .arg(Arg::new("file_name")
                    .value_name("FILE_NAME")
                    .help("Name of the todo list that should be created.")
                    .required(true)
                )
            )
            .subcommand(Command::new("list")
                .about("List existing todo lists")
            )
    }
}
