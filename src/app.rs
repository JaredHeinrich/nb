use std::fs;
use std::process::Stdio;

use anyhow::{Ok, Result};
use clap::{Arg, ArgMatches, Command};

use crate::error::AppError;
use crate::message::Message;
use crate::{config, file_operations};

pub struct App {
    config: config::Config,
}

impl App {
    pub fn new(config: config::Config) -> Self {
        App { config }
    }

    fn check_editor(&self) -> Result<()> {
        let res = std::process::Command::new(&self.config.editor)
            .arg("-v")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|_| ())
            .map_err(|_| AppError::EditorNotInstalled(self.config.editor.to_owned()).into());
        res
    }

    fn check_dir_structure(&self) -> Result<()> {
        if fs::exists(&self.config.nb_root_dir)? == false {
            file_operations::create_dir(&self.config.nb_root_dir)?;
        }
        Ok(())
    }

    fn check_default_notebook(&self) -> Result<()> {
        let mut default_nb_path = self.config.nb_root_dir.clone();
        default_nb_path.push(&self.config.default_notebook);
        if fs::exists(&default_nb_path)? == false {
            file_operations::create_file(&default_nb_path)?;
        }
        Ok(())
    }

    fn create_notebook(&self, name: &str) -> Result<Message> {
        let mut nb_path = self.config.nb_root_dir.clone();
        nb_path.push(&name);
        if fs::exists(&nb_path)? {
            Err(AppError::AlreadyExists)?;
        }
        file_operations::create_file(&nb_path)?;
        Ok(Message::CreatedNoteBook)
    }

    fn delete_node_book(&self, name: &str) -> Result<Message> {
        let mut nb_path = self.config.nb_root_dir.clone();
        nb_path.push(&name);
        if !fs::exists(&nb_path)? {
            Err(AppError::NotFound)?;
        }
        file_operations::delete_file(&nb_path)?;
        Ok(Message::DeletedNoteBook)
    }

    fn list_notebooks(&self) -> Result<Message> {
        let nb_dir = &self.config.nb_root_dir;
        let files = file_operations::get_files(nb_dir)?;
        Ok(Message::ListOfNoteBooks(files))
    }

    fn open_notebook(&self, name: &str) -> Result<Message> {
        let mut notebook_path = self.config.nb_root_dir.to_owned();
        notebook_path.push(&name);
        if !fs::exists(&notebook_path)? {
            Err(AppError::NotFound)?;
        }
        file_operations::open_file(&self.config.editor, &notebook_path)?;
        Ok(Message::EmptyMessage)
    }

    pub fn handle_command(&self, matches: ArgMatches) -> Result<Message> {
        self.check_editor()?;
        self.check_dir_structure()?;
        self.check_default_notebook()?;

        match matches.subcommand() {
            Some(("new", sub_matches)) => {
                let name = sub_matches.get_one::<String>("name").unwrap();
                self.create_notebook(name)
            }
            Some(("open", sub_matches)) => {
                let name = sub_matches.get_one::<String>("name").unwrap();
                self.open_notebook(name)
            }
            Some(("remove", sub_matches)) => {
                let name = sub_matches.get_one::<String>("name").unwrap();
                self.delete_node_book(name)
            }
            Some(("list", _sub_matches)) => self.list_notebooks(),
            _ => Err(AppError::CommandNotHandled)?,
        }
    }

    pub fn get_command(&self) -> Command {
        Command::new("nb")
            .version("0.1.0")
            .about("CLI note book manager")
            .subcommand_required(true)
            .subcommand(
                Command::new("new").about("Create a new note book").arg(
                    Arg::new("name")
                        .value_name("NAME")
                        .help("Name of the note book to be created.")
                        .required(true),
                ),
            )
            .subcommand(
                Command::new("open").about("Open a note book").arg(
                    Arg::new("name")
                        .value_name("NAME")
                        .help("Name of the note book to open.")
                        .default_value(self.config.default_notebook.clone()),
                ),
            )
            .subcommand(
                Command::new("remove")
                    .visible_alias("rm")
                    .about("Delete a note book")
                    .arg(
                        Arg::new("name")
                            .value_name("NAME")
                            .help("Name of the note book to be deleted.")
                            .required(true),
                    ),
            )
            .subcommand(
                Command::new("list")
                    .visible_alias("ls")
                    .about("List existing note books"),
            )
    }
}
