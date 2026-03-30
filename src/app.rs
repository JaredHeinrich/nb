use std::fs;
use std::path::PathBuf;
use std::process::Stdio;

use anyhow::{Ok, Result};

use crate::cli::{
    ArchiveArgs, ArchiveOpenArgs, ArchiveRemoveArgs, ArchiveRestoreArgs, ArchiveSaveArgs, ArchiveSubcommand, Cli, CompletionArgs, ConfigArgs, ConfigGenerateArgs, ConfigGetArgs, ConfigSubcommand, NewArgs, OpenArgs, RemoveArgs, Shell, Subcommand
};
use crate::config;
use crate::config::{Config, PartialConfig};
use crate::error::AppError;
use crate::file_operations::FileOperations;
use crate::message::Message;

const NB_ROOT_DIR: &'static str = ".notebooks";

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

    fn handle_new(&mut self, args: NewArgs) -> Result<Message> {
        let mut nb_path = self.nb_root_dir.clone();
        nb_path.push(&args.name);
        if fs::exists(&nb_path)? {
            Err(AppError::AlreadyExists)?;
        }
        self.fs.create_file(&nb_path)?;
        Ok(Message::CreatedNoteBook)
    }

    fn handle_remove(&mut self, args: RemoveArgs) -> Result<Message> {
        let mut nb_path = self.nb_root_dir.clone();
        nb_path.push(&args.name);
        if !fs::exists(&nb_path)? {
            Err(AppError::NotFound)?;
        }
        self.fs.delete_file(&nb_path)?;
        Ok(Message::DeletedNoteBook)
    }

    fn handle_list(&self) -> Result<Message> {
        let files = self.fs.get_files(&self.nb_root_dir)?;
        Ok(Message::ListOfNoteBooks(files))
    }

    fn handle_open(&mut self, args: OpenArgs) -> Result<Message> {
        let mut notebook_path = self.nb_root_dir.to_owned();
        let notebook = args
            .name
            .as_deref()
            .unwrap_or(&self.config.default_notebook);
        notebook_path.push(notebook);
        if !fs::exists(&notebook_path)? {
            return Err(AppError::NotFound.into());
        }
        self.fs.open_file(&self.config.editor, &notebook_path)?;
        Ok(Message::EmptyMessage)
    }

    fn handle_completions(&self, args: CompletionArgs) -> Result<Message> {
        let script = match args.shell {
            Shell::Zsh => include_str!("../completions/_nb").to_owned(),
        };
        Ok(Message::CompletionScript(script))
    }

    fn handle_config(&self, args: ConfigArgs) -> Result<Message> {
        match args.subcommand {
            ConfigSubcommand::Generate(args) => self.handle_config_generate(args),
            ConfigSubcommand::Get(args) => self.handle_config_get(args),
            ConfigSubcommand::List => self.handle_config_list(),
        }
    }

    fn handle_config_generate(&self, args: ConfigGenerateArgs) -> Result<Message> {
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

    fn handle_config_get(&self, args: ConfigGetArgs) -> Result<Message> {
        Ok(Message::ConfigValues(
            self.get_config_values(&args.value_names)?,
        ))
    }

    fn handle_config_list(&self) -> Result<Message> {
        Ok(Message::ConfigValues(
            self.get_config_values(&config::value_names::ALL)?,
        ))
    }

    fn handle_archive(&self, args: ArchiveArgs) -> Result<Message> {
        match args.subcommand {
            ArchiveSubcommand::Save(args) => self.handle_archive_save(args),
            ArchiveSubcommand::List => self.handle_archive_list(),
            ArchiveSubcommand::Open(args) => self.handle_archive_open(args),
            ArchiveSubcommand::Restore(args) => self.handle_archive_restore(args),
            ArchiveSubcommand::Remove(args) => self.handle_archive_remove(args),
        }
    }

    fn handle_archive_save(&self, args: ArchiveSaveArgs) -> Result<Message> {
        todo!("Not implemented");
    }

    fn handle_archive_list(&self) -> Result<Message> {
        todo!("Not implemented");
    }

    fn handle_archive_open(&self, args: ArchiveOpenArgs) -> Result<Message> {
        todo!("Not implemented");
    }

    fn handle_archive_restore(&self, args: ArchiveRestoreArgs) -> Result<Message> {
        todo!("Not implemented");
    }

    fn handle_archive_remove(&self, args: ArchiveRemoveArgs) -> Result<Message> {
        todo!("Not implemented");
    }

    pub fn handle_command(&mut self, command: Cli) -> Result<Message> {
        self.check_editor()?;
        self.check_dir_structure()?;
        self.check_default_notebook()?;
        match command.subcommand {
            Subcommand::New(args) => self.handle_new(args),
            Subcommand::Open(args) => self.handle_open(args),
            Subcommand::Remove(args) => self.handle_remove(args),
            Subcommand::List => self.handle_list(),
            Subcommand::Completions(args) => self.handle_completions(args),
            Subcommand::Config(args) => self.handle_config(args),
            Subcommand::Archive(args) => self.handle_archive(args),
        }
    }
}
