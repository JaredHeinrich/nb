use std::fs;
use std::process::Stdio;

use anyhow::Result;
use clap::{ArgMatches, Command};
use clap_complete::Shell;

use crate::cli::build_command_with_config;
use crate::error::AppError;
use crate::message::Message;
use crate::{config, file_operations, utils};

#[derive(Default)]
pub struct App {
    pub config: config::Config,
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

    pub fn list_notebooks(&self) -> Result<Message> {
        let files = utils::list_notebooks(&self.config)?;
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

    fn get_completion_script(&self, _shell: &Shell) -> Result<Message> {
        todo!("Return completion script for specified shell");
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
            Some(("completions", sub_matches)) => {
                let shell = sub_matches.get_one::<Shell>("shell").unwrap();
                self.get_completion_script(shell)
            }
            _ => Err(AppError::CommandNotHandled)?,
        }
    }
    pub fn build_command(&self) -> Result<Command> {
        build_command_with_config(&self.config)
    }
}
