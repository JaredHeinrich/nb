use std::fs;

use anyhow::{Ok, Result};
use clap::{Arg, ArgMatches, Command};

use crate::error::{ CreationError, OpenError};
use crate::{config, file_operations};
use crate::message::Message;

pub struct App{
    config: config::Config
}

impl App{
    pub fn new(config: config::Config) -> Self{
        App { config }
    }

    fn check_state(&self) -> Result<()> {
        // ensure existence of nb_dir
        if fs::exists(&self.config.nb_dir)? == false {
            file_operations::create_dir(&self.config.nb_dir)?;
        }
        // ensure existence of default_file
        let mut default_nb_path = self.config.nb_dir.clone();
        default_nb_path.push(&self.config.default_file);
        if fs::exists(&default_nb_path)? == false {
            file_operations::create_file(&default_nb_path)?;
        }
        Ok(())
    }

    fn create_note_book(&self, name: &str) -> Result<Message> {
        let mut nb_path = self.config.nb_dir.clone();
        nb_path.push(&name);
        if fs::exists(&nb_path)? {
            Err(CreationError::AlreadyExists)?;
        }
        file_operations::create_file(&nb_path)?;
        Ok(Message::CreatedNoteBook)
    }

    fn list_note_books(&self) -> Result<Message> {
        let nb_dir = &self.config.nb_dir;
        let files = file_operations::get_files(nb_dir)?;
        Ok(Message::ListOfNoteBooks(files))
    }

    fn open_note_book(&self, name: &str) -> Result<Message> {
        let mut note_book_path = self.config.nb_dir.to_owned();
        note_book_path.push(&name);
        if !fs::exists(&note_book_path)? {
            Err(OpenError::NotFound)?;
        }
        file_operations::open_file(&self.config.editor, &note_book_path)?;
        Ok(Message::EmptyMessage)
    }


    pub fn handle_command(&self, matches: ArgMatches) -> Result<Message> {
        self.check_state()?;

        match matches.subcommand() {
            Some(("list", _sub_matches)) => self.list_note_books(),
            Some(("new", sub_matches)) => {
                let name = sub_matches.get_one::<String>("name").unwrap();
                self.create_note_book(name)
            },
            _ => {
                let name = matches.get_one::<String>("name").unwrap();
                self.open_note_book(name)
            }
        }
    }

    pub fn get_command(&self) -> Command {
        Command::new("nb")
            .version("0.1.0")
            .about("CLI note book manager")
            .arg(Arg::new("name")
                .value_name("NAME")
                .help("Name of the note book to open")
                .default_value(self.config.default_file.clone())
            )
            .subcommand(Command::new("new")
                .about("Create a new note book")
                .arg(Arg::new("name")
                    .value_name("NAME")
                    .help("Name of the note book that should be created.")
                    .required(true)
                )
            )
            .subcommand(Command::new("list")
                .about("List existing note books")
            )
    }
}
