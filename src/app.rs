use std::fs;
use std::path::PathBuf;
use std::process::Stdio;

use anyhow::Result;
use clap::ArgMatches;

use crate::cli::Shell;
use crate::config;
use crate::config::{Config, PartialConfig};
use crate::error::AppError;
use crate::file_operations::FileOperations;
use crate::message::Message;

const NB_ROOT_DIR: &'static str = ".notebooks";

mod cmd {
    pub const NEW: &str = "new";
    pub const OPEN: &str = "open";
    pub const REMOVE: &str = "remove";
    pub const LIST: &str = "list";
    pub const COMPLETIONS: &str = "completions";
    pub const CONFIG: &str = "config";
    pub const CONFIG_GENERATE: &str = "generate";
    pub const CONFIG_LIST: &str = "list";
    pub const CONFIG_GET: &str = "get";
}

mod arg {
    pub const NAME: &str = "name";
    pub const SHELL: &str = "shell";
    pub const VALUE_NAME: &str = "value_name";
    pub const FORCE: &str = "force";
}

pub struct App<FS: FileOperations> {
    pub config: config::Config,
    pub nb_root_dir: PathBuf,
    fs: FS,
}

impl<FS: FileOperations> App<FS> {
    pub fn new(config: config::Config, fs: FS) -> Result<Self> {
        let Some(mut nb_root_dir) = std::env::home_dir() else {
            return Err(AppError::NoHomeDir.into());
        };
        nb_root_dir.push(NB_ROOT_DIR);
        Ok(Self {
            config,
            nb_root_dir,
            fs,
        })
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
        if self.fs.exists(&self.nb_root_dir)? == false {
            self.fs.create_dir(&self.nb_root_dir)?;
        }
        Ok(())
    }

    fn check_default_notebook(&mut self) -> Result<()> {
        let mut default_nb_path = self.nb_root_dir.clone();
        default_nb_path.push(&self.config.default_notebook);
        if self.fs.exists(&default_nb_path)? == false {
            self.fs.create_file(&default_nb_path)?;
        }
        Ok(())
    }

    fn create_notebook(&mut self, name: &str) -> Result<Message> {
        let mut nb_path = self.nb_root_dir.clone();
        nb_path.push(&name);
        if fs::exists(&nb_path)? {
            Err(AppError::AlreadyExists)?;
        }
        self.fs.create_file(&nb_path)?;
        Ok(Message::CreatedNoteBook)
    }

    fn delete_node_book(&mut self, name: &str) -> Result<Message> {
        let mut nb_path = self.nb_root_dir.clone();
        nb_path.push(&name);
        if !fs::exists(&nb_path)? {
            Err(AppError::NotFound)?;
        }
        self.fs.delete_file(&nb_path)?;
        Ok(Message::DeletedNoteBook)
    }

    pub fn list_notebooks(&self) -> Result<Message> {
        let files = self.fs.get_files(&self.nb_root_dir)?;
        Ok(Message::ListOfNoteBooks(files))
    }

    fn open_notebook(&mut self, name: &str) -> Result<Message> {
        let mut notebook_path = self.nb_root_dir.to_owned();
        notebook_path.push(&name);
        if !fs::exists(&notebook_path)? {
            Err(AppError::NotFound)?;
        }
        self.fs.open_file(&self.config.editor, &notebook_path)?;
        Ok(Message::EmptyMessage)
    }

    fn get_completion_script(&self, shell: &Shell) -> Result<Message> {
        match shell {
            Shell::Zsh => Ok(Message::CompletionScript(
                include_str!("../completions/_nb").to_owned(),
            )),
        }
    }

    fn generate_config(&self, force: bool) -> Result<Message> {
        let config_file_path = config::config_file();
        let config_exists = self.fs.exists(&config_file_path)?;
        if config_exists && !force {
            return Err(AppError::ConfigAlreadyExists(config_file_path).into());
        }
        let config = Config::default();
        let config_string = config.to_string();
        self.fs.write_file(&config_file_path, &config_string)?;
        Ok(Message::GeneratedConfig(config_file_path))
    }

    fn config_values<T: AsRef<str>>(&self, value_names: &[T]) -> Result<Message> {
        let mut config_values: Vec<(String, String)> = Vec::new();
        let config_file_path = config::config_file();
        let config_exists = self.fs.exists(&config_file_path)?;
        if !config_exists {
            return Ok(Message::ConfigValues(config_values));
        }
        let config = PartialConfig::from_config_file(&self.fs)?;
        for value_name in value_names {
            let value_name = value_name.as_ref();
            let value = match value_name {
                config::value_names::DEFAULT_NOTEBOOK => config.default_notebook.as_ref(),
                config::value_names::EDITOR => config.editor.as_ref(),
                _ => continue,
            };
            if let Some(value) = value {
                config_values.push((value_name.to_owned(), value.to_owned()));
            }
        }
        Ok(Message::ConfigValues(config_values))
    }

    fn all_config_values(&self) -> Result<Message> {
        self.config_values(&config::value_names::ALL)
    }

    pub fn handle_command(&mut self, matches: ArgMatches) -> Result<Message> {
        self.check_editor()?;
        self.check_dir_structure()?;
        self.check_default_notebook()?;

        match matches.subcommand() {
            Some((cmd::NEW, sub_matches)) => {
                let name = sub_matches.get_one::<String>(arg::NAME).unwrap();
                self.create_notebook(name)
            }
            Some((cmd::OPEN, sub_matches)) => {
                let name = sub_matches.get_one::<String>(arg::NAME).unwrap();
                self.open_notebook(name)
            }
            Some((cmd::REMOVE, sub_matches)) => {
                let name = sub_matches.get_one::<String>(arg::NAME).unwrap();
                self.delete_node_book(name)
            }
            Some((cmd::LIST, _sub_matches)) => self.list_notebooks(),
            Some((cmd::COMPLETIONS, sub_matches)) => {
                let shell = sub_matches.get_one::<Shell>(arg::SHELL).unwrap();
                self.get_completion_script(shell)
            }
            Some((cmd::CONFIG, sub_matches)) => match sub_matches.subcommand() {
                Some((cmd::CONFIG_GENERATE, sub_matches)) => {
                    let force = sub_matches.get_flag(arg::FORCE);
                    self.generate_config(force)
                }
                Some((cmd::CONFIG_GET, sub_matches)) => {
                    let values: Vec<String> = sub_matches
                        .get_many(arg::VALUE_NAME)
                        .unwrap()
                        .cloned()
                        .collect();
                    self.config_values(&values)
                }
                Some((cmd::CONFIG_LIST, _sub_matches)) => self.all_config_values(),
                _ => Err(AppError::CommandNotHandled)?,
            },
            _ => Err(AppError::CommandNotHandled)?,
        }
    }
}
