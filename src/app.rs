use std::fs;
use std::process::Stdio;

use anyhow::Result;
use clap::{ArgMatches, Command};
use clap_complete::Shell;

use crate::cli::build_command_with_config;
use crate::error::AppError;
use crate::file_operations::FileOperations;
use crate::message::Message;
use crate::{config, utils};

pub struct App<FS: FileOperations> {
    pub config: config::Config,
    fs: FS,
}

impl<FS: FileOperations> App<FS> {
    pub fn new(config: config::Config, fs: FS) -> Self {
        Self { config, fs }
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

    fn check_dir_structure(&mut self) -> Result<()> {
        if self.fs.exists(&self.config.nb_root_dir)? == false {
            self.fs.create_dir(&self.config.nb_root_dir)?;
        }
        Ok(())
    }

    fn check_default_notebook(&mut self) -> Result<()> {
        let mut default_nb_path = self.config.nb_root_dir.clone();
        default_nb_path.push(&self.config.default_notebook);
        if self.fs.exists(&default_nb_path)? == false {
            self.fs.create_file(&default_nb_path)?;
        }
        Ok(())
    }

    fn create_notebook(&mut self, name: &str) -> Result<Message> {
        let mut nb_path = self.config.nb_root_dir.clone();
        nb_path.push(&name);
        if fs::exists(&nb_path)? {
            Err(AppError::AlreadyExists)?;
        }
        self.fs.create_file(&nb_path)?;
        Ok(Message::CreatedNoteBook)
    }

    fn delete_node_book(&mut self, name: &str) -> Result<Message> {
        let mut nb_path = self.config.nb_root_dir.clone();
        nb_path.push(&name);
        if !fs::exists(&nb_path)? {
            Err(AppError::NotFound)?;
        }
        self.fs.delete_file(&nb_path)?;
        Ok(Message::DeletedNoteBook)
    }

    pub fn list_notebooks(&self) -> Result<Message> {
        let files = utils::list_notebooks(&self.config, &self.fs)?;
        Ok(Message::ListOfNoteBooks(files))
    }

    fn open_notebook(&mut self, name: &str) -> Result<Message> {
        let mut notebook_path = self.config.nb_root_dir.to_owned();
        notebook_path.push(&name);
        if !fs::exists(&notebook_path)? {
            Err(AppError::NotFound)?;
        }
        self.fs.open_file(&self.config.editor, &notebook_path)?;
        Ok(Message::EmptyMessage)
    }

    fn get_completion_script(&self, shell: &Shell) -> Result<Message> {
        match shell {
            Shell::Zsh => Ok(Message::CompletionScript("echo \"source <(COMPLETE=zsh your_program)\" >> ~/.zshrc".to_owned())),
            Shell::Bash => Ok(Message::CompletionScript("echo \"source <(COMPLETE=bash your_program)\" >> ~/.bashrc".to_owned())),
            Shell::Fish => Ok(Message::CompletionScript("echo \"source (COMPLETE=fish your_program | psub)\" >> ~/.config/fish/config.fish".to_owned())),
            Shell::Elvish => Ok(Message::CompletionScript("echo \"eval (E:COMPLETE=elvish your_program | slurp)\" >> ~/.elvish/rc.elv".to_owned())),
            Shell::PowerShell => Ok(Message::CompletionScript("echo \"COMPLETE=powershell your_program | Invoke-Expression\" >> $PROFILE".to_owned())),
            _ => Err(AppError::UnknownShell.into())
        }
    }

    pub fn handle_command(&mut self, matches: ArgMatches) -> Result<Message> {
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
        build_command_with_config(&self.config, &self.fs)
    }
}
