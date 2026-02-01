use std::fs;
use std::process::Stdio;

use anyhow::Result;
use clap::builder::{EnumValueParser, PossibleValue};
use clap::{Arg, ArgMatches, Command, ValueEnum};

use crate::error::AppError;
use crate::message::Message;
use crate::{config, file_operations};

#[derive(Clone, PartialEq, Debug)]
enum Shell {
    Zsh
}
impl ValueEnum for Shell {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Zsh => PossibleValue::new("zsh")
        })
    }
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Zsh
        ]
    }
}

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

    pub fn get_command(&self) -> Command {
        Command::new("nb")
            .version("0.1.0")
            .about("CLI note book manager")
            .subcommand_required(true)
            .subcommand(
                Command::new("new").about("Create a new note book").arg(
                    Arg::new("name")
                        .value_name("NAME")
                        .value_parser(|s: &str| {
                            let trimmed = s.trim();
                            if trimmed.is_empty() {
                                return Err("Name must not be empty".to_string());
                            }
                            Ok(trimmed.to_string())
                        })
                        .help("Name of the note book to be created.")
                        .required(true),
                ),
            )
            .subcommand(
                Command::new("open").about("Open a note book").arg(
                    Arg::new("name")
                        .value_name("NAME")
                        .value_parser(|s: &str| {
                            let trimmed = s.trim();
                            if trimmed.is_empty() {
                                return Err("Name must not be empty".to_string());
                            }
                            Ok(trimmed.to_string())
                        })
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
            .subcommand(
                Command::new("completions")
                    .about("Generate the completion script for a specific shell")
                    .arg(
                        Arg::new("shell")
                            .short('s')
                            .long("shell")
                            .value_name("SHELL")
                            .value_parser(EnumValueParser::<Shell>::new())
                            .help("Shell for which the completion is generated.")
                            .required(true),
                    )
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;


    #[test]
    fn test_nb_no_subcommand() {
        let app = App::new(Config::default());
        assert!(app.get_command().try_get_matches_from(["nb"]).is_err());
    }

    #[test]
    fn test_nb_invalid_subcommands() {
        let app = App::new(Config::default());
        assert!(app.get_command().try_get_matches_from(["nb", " "]).is_err());
        assert!(app.get_command().try_get_matches_from(["nb", "test"]).is_err());
        assert!(app.get_command().try_get_matches_from(["nb", "-t"]).is_err());
        assert!(app.get_command().try_get_matches_from(["nb", "--test"]).is_err());
    }

    #[test]
    fn test_new_no_name() {
        let app = App::new(Config::default());
        assert!(app.get_command().try_get_matches_from(["nb", "new"]).is_err());
        assert!(app.get_command().try_get_matches_from(["nb", "new", ""]).is_err());
        assert!(app.get_command().try_get_matches_from(["nb", "new", " "]).is_err());
        assert!(app.get_command().try_get_matches_from(["nb", "new", "\r"]).is_err());
        assert!(app.get_command().try_get_matches_from(["nb", "new", "\n"]).is_err());
        assert!(app.get_command().try_get_matches_from(["nb", "new", "\t"]).is_err());
    }

    #[test]
    fn test_new_multiple_names() {
        let app = App::new(Config::default());
        assert!(app.get_command().try_get_matches_from(["nb", "new", "a", "b"]).is_err());
    }

    #[test]
    fn test_new() {
        let app = App::new(Config::default());
        let matches = app.get_command().get_matches_from(["nb", "new", "my_notebook"]);
        assert!(matches.try_get_one::<String>("name").is_err());
        let (subcommand, matches) = matches.subcommand().unwrap();
        assert_eq!(subcommand, "new");
        assert_eq!(matches.get_one::<String>("name").unwrap(), "my_notebook");
    }

    #[test]
    fn test_open_empty_name() {
        let app = App::new(Config::default());
        assert!(app.get_command().try_get_matches_from(["nb", "open", ""]).is_err());
        assert!(app.get_command().try_get_matches_from(["nb", "open", " "]).is_err());
        assert!(app.get_command().try_get_matches_from(["nb", "open", "\r"]).is_err());
        assert!(app.get_command().try_get_matches_from(["nb", "open", "\n"]).is_err());
        assert!(app.get_command().try_get_matches_from(["nb", "open", "\t"]).is_err());
    }

    #[test]
    fn test_open_multiple_names() {
        let app = App::new(Config::default());
        assert!(app.get_command().try_get_matches_from(["nb", "open", "a", "b"]).is_err());
    }

    #[test]
    fn test_open_no_name() {
        let config = Config::default();
        let app = App::new(config.clone());
        let matches = app.get_command().get_matches_from(["nb", "open"]);
        let (subcommand, matches) = matches.subcommand().unwrap();
        assert_eq!(subcommand, "open");
        assert_eq!(*matches.get_one::<String>("name").unwrap(), config.default_notebook);
    }

    #[test]
    fn test_open() {
        let app = App::new(Config::default());
        let matches = app.get_command().try_get_matches_from(["nb", "open", "my_notebook"]).unwrap();
        assert!(matches.try_get_one::<String>("name").is_err());
        let (subcommand, matches) = matches.subcommand().unwrap();
        assert_eq!(subcommand, "open");
        assert_eq!(matches.get_one::<String>("name").unwrap(), "my_notebook");
    }

    #[test]
    fn test_completions_no_shell() {
        let app = App::new(Config::default());
        assert!(app.get_command().try_get_matches_from(["nb", "completions"]).is_err());
    }

    #[test]
    fn test_completions_no_argument_name() {
        let app = App::new(Config::default());
        assert!(app.get_command().try_get_matches_from(["nb", "completions", "zsh"]).is_err());
    }

    #[test]
    fn test_completions() {
        let app = App::new(Config::default());
        let matches = app.get_command().get_matches_from(["nb", "completions", "-s", "zsh"]);
        let (subcommand, matches) = matches.subcommand().unwrap();
        assert_eq!(subcommand, "completions");
        assert_eq!(*matches.get_one::<Shell>("shell").unwrap(), Shell::Zsh);

        let matches = app.get_command().get_matches_from(["nb", "completions", "--shell", "zsh"]);
        let (subcommand, matches) = matches.subcommand().unwrap();
        assert_eq!(subcommand, "completions");
        assert_eq!(*matches.get_one::<Shell>("shell").unwrap(), Shell::Zsh);
    }
}
