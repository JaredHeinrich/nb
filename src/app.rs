use std::path::PathBuf;
use std::process::Stdio;

use anyhow::{Ok, Result};
use chrono::Local;

use crate::cli;
use crate::config;
use crate::config::{Config, PartialConfig};
use crate::error::AppError;
use crate::file_operations::FileOperations;
use crate::message::Message;

const NB_ROOT_DIR: &str = ".nb";
const NOTEBOOK_DIR_NAME: &str = "notebooks";
const ARCHIVE_DIR_NAME: &str = "archive";

#[derive(Clone, Copy)]
enum NotebookType {
    Active,
    Archived,
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

    fn notebook_dir(&self) -> PathBuf {
        let mut notebook_dir = self.nb_root_dir.clone();
        notebook_dir.push(NOTEBOOK_DIR_NAME);
        notebook_dir
    }

    fn archive_dir(&self) -> PathBuf {
        let mut notebook_dir = self.nb_root_dir.clone();
        notebook_dir.push(ARCHIVE_DIR_NAME);
        notebook_dir
    }

    fn get_dir_path(&self, nb_type: NotebookType) -> PathBuf {
        match nb_type {
            NotebookType::Active => self.notebook_dir(),
            NotebookType::Archived => self.archive_dir(),
        }
    }

    fn get_nb_path(&self, name: &str, nb_type: NotebookType) -> PathBuf {
        let mut path = self.get_dir_path(nb_type);
        path.push(name);
        path
    }

    // TODO remove if not needed
    // fn name_exists(&self, name: &str) -> Result<bool> {
    //     let active_path = self.get_nb_path(name, NotebookType::Active);
    //     let archived_path = self.get_nb_path(name, NotebookType::Archived);
    //     Ok(self.fs.exists(&active_path)? || self.fs.exists(&archived_path)?)
    // }

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
        let nb_root_dir = &self.nb_root_dir;
        if !self.fs.exists(nb_root_dir)? {
            self.fs.create_dir(nb_root_dir)?;
        }
        let active_dir = self.get_dir_path(NotebookType::Active);
        if !self.fs.exists(&active_dir)? {
            self.fs.create_dir(&active_dir)?;
        }
        let archive_dir = self.get_dir_path(NotebookType::Archived);
        if !self.fs.exists(&archive_dir)? {
            self.fs.create_dir(&archive_dir)?;
        }
        Ok(())
    }

    fn check_default_notebook(&mut self) -> Result<()> {
        let nb_name = self.config.default_notebook.as_str();
        let nb_path = self.get_nb_path(nb_name, NotebookType::Active);
        if !self.fs.exists(&nb_path)? {
            self.fs.create_file(&nb_path)?;
        }
        Ok(())
    }

    fn open_notebook(&mut self, name: &str, nb_type: NotebookType) -> Result<Message> {
        let path = self.get_nb_path(name, nb_type);
        if !self.fs.exists(&path)? {
            return Err(AppError::NotFound.into());
        }
        self.fs.open_file(&self.config.editor, &path)?;
        Ok(Message::Empty)
    }

    fn get_config_values<T: AsRef<str>>(&self, value_names: &[T]) -> Result<Vec<(String, String)>> {
        let mut config_values: Vec<(String, String)> = Vec::new();
        let config_file_path = config::config_file();
        let config_exists = self.fs.exists(&config_file_path)?;
        if !config_exists {
            return Ok(config_values);
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
        Ok(config_values)
    }

    fn handle_new(&mut self, args: cli::NewArgs) -> Result<Message> {
        let name = args.name.as_str();
        let path = self.get_nb_path(name, NotebookType::Active);
        if self.fs.exists(&path)? {
            return Err(AppError::AlreadyExists.into());
        }
        self.fs.create_file(&path)?;
        Ok(Message::CreatedNoteBook)
    }

    fn handle_remove(&mut self, args: cli::RemoveArgs) -> Result<Message> {
        let name = args.name.as_str();
        let path = self.get_nb_path(name, NotebookType::Active);
        if !self.fs.exists(&path)? {
            return Err(AppError::NotFound.into());
        }
        self.fs.delete_file(&path)?;
        Ok(Message::DeletedNoteBook)
    }

    fn handle_list(&self) -> Result<Message> {
        let files = self
            .fs
            .get_files(&self.get_dir_path(NotebookType::Active))?;
        Ok(Message::ListOfNoteBooks(files))
    }

    fn handle_open(&mut self, args: cli::OpenArgs) -> Result<Message> {
        let name = args
            .name
            .as_deref()
            .unwrap_or(&self.config.default_notebook)
            .to_owned();
        self.open_notebook(&name, NotebookType::Active)
    }

    fn handle_completions(&self, args: cli::CompletionArgs) -> Result<Message> {
        let script = match args.shell {
            cli::Shell::Zsh => include_str!("../completions/_nb").to_owned(),
        };
        Ok(Message::CompletionScript(script))
    }

    fn handle_config(&mut self, args: cli::ConfigArgs) -> Result<Message> {
        match args.subcommand {
            cli::ConfigSubcommand::Generate(args) => self.handle_config_generate(args),
            cli::ConfigSubcommand::Get(args) => self.handle_config_get(args),
            cli::ConfigSubcommand::List => self.handle_config_list(),
        }
    }

    fn handle_config_generate(&mut self, args: cli::ConfigGenerateArgs) -> Result<Message> {
        let config_file_path = config::config_file();
        let config_exists = self.fs.exists(&config_file_path)?;
        if config_exists && !args.force {
            return Err(AppError::ConfigAlreadyExists(config_file_path).into());
        }
        let config = Config::default();
        let config_string = config.to_string();
        self.fs.write_file(&config_file_path, &config_string)?;
        Ok(Message::GeneratedConfig(config_file_path))
    }

    fn handle_config_get(&self, args: cli::ConfigGetArgs) -> Result<Message> {
        Ok(Message::ConfigValues(
            self.get_config_values(&args.value_names)?,
        ))
    }

    fn handle_config_list(&self) -> Result<Message> {
        Ok(Message::ConfigValues(
            self.get_config_values(&config::value_names::ALL)?,
        ))
    }

    fn handle_archive(&mut self, args: cli::ArchiveArgs) -> Result<Message> {
        match args.subcommand {
            cli::ArchiveSubcommand::Save(args) => self.handle_archive_save(args),
            cli::ArchiveSubcommand::List => self.handle_archive_list(),
            cli::ArchiveSubcommand::Open(args) => self.handle_archive_open(args),
            cli::ArchiveSubcommand::Restore(args) => self.handle_archive_restore(args),
            cli::ArchiveSubcommand::Remove(args) => self.handle_archive_remove(args),
        }
    }

    fn handle_archive_save(&mut self, args: cli::ArchiveSaveArgs) -> Result<Message> {
        let active_path = self.get_nb_path(args.name.as_str(), NotebookType::Active);
        if !self.fs.exists(&active_path)? {
            return Err(AppError::NotFound.into());
        }
        let time_stamp = Local::now().format("%d-%m-%Y-%H:%M:%S").to_string();
        let archived_name = format!("{}_{time_stamp}", args.name);
        let archived_path = self.get_nb_path(archived_name.as_str(), NotebookType::Archived);
        if self.fs.exists(&archived_path)? {
            todo!("Same archive file already exists case not handled yet.");
        }
        self.fs.copy(&active_path, &archived_path)?;
        self.fs.delete_file(&active_path)?;
        Ok(Message::ArchivedNotebook((args.name, archived_name)))
    }

    fn handle_archive_list(&self) -> Result<Message> {
        let files = self
            .fs
            .get_files(&self.get_dir_path(NotebookType::Archived))?;
        Ok(Message::ListOfNoteBooks(files))
    }

    fn handle_archive_open(&mut self, args: cli::ArchiveOpenArgs) -> Result<Message> {
        self.open_notebook(args.name.as_str(), NotebookType::Archived)
    }

    fn handle_archive_restore(&mut self, args: cli::ArchiveRestoreArgs) -> Result<Message> {
        let new_name = args.new_name.unwrap_or_else(|| {
            match args.archive_name.rsplit_once('_') {
                Some((name, _time_stamp)) => name,
                None => args.archive_name.as_str(),
            }
            .to_owned()
        });
        let path = self.get_nb_path(new_name.as_str(), NotebookType::Active);
        if self.fs.exists(&path)? {
            // TODO use specific error type
            return Err(AppError::AlreadyExists.into());
        }
        let archived_path = self.get_nb_path(args.archive_name.as_str(), NotebookType::Archived);
        self.fs.copy(&archived_path, &path)?;
        Ok(Message::RestoredNotebook((args.archive_name, new_name)))
    }

    fn handle_archive_remove(&mut self, args: cli::ArchiveRemoveArgs) -> Result<Message> {
        let name = args.name;
        let path = self.get_nb_path(name.as_str(), NotebookType::Archived);
        if !self.fs.exists(&path)? {
            return Err(AppError::NotFound.into());
        }
        self.fs.delete_file(&path)?;
        Ok(Message::DeletedNoteBook)
    }

    pub fn handle_command(&mut self, command: cli::Cli) -> Result<Message> {
        self.check_editor()?;
        self.check_dir_structure()?;
        self.check_default_notebook()?;
        match command.subcommand {
            cli::Subcommand::New(args) => self.handle_new(args),
            cli::Subcommand::Open(args) => self.handle_open(args),
            cli::Subcommand::Remove(args) => self.handle_remove(args),
            cli::Subcommand::List => self.handle_list(),
            cli::Subcommand::Completions(args) => self.handle_completions(args),
            cli::Subcommand::Config(args) => self.handle_config(args),
            cli::Subcommand::Archive(args) => self.handle_archive(args),
        }
    }
}
