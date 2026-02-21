use clap::{
    builder::{EnumValueParser, PossibleValue},
    Arg, Command, ValueEnum,
};

#[derive(Clone, PartialEq, Debug)]
pub enum Shell {
    Zsh,
}
impl ValueEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Shell::Zsh,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Shell::Zsh => PossibleValue::new("zsh"),
        })
    }
}

pub fn build_command() -> Command {
    Command::new("nb")
        .version("0.1.0")
        .about("CLI note book manager")
        .subcommand_required(true)
        .flatten_help(true)
        .disable_help_subcommand(true)
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
                    .help("Name of the note book to open."),
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
                .about("Completion script for specific shell")
                .arg(
                    Arg::new("shell")
                        .short('s')
                        .long("shell")
                        .value_name("SHELL")
                        .value_parser(EnumValueParser::<Shell>::new())
                        .help("Shell for which to return the completion script")
                        .action(clap::ArgAction::Set)
                        .required(true),
                ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nb_no_subcommand() {
        let cmd = build_command();
        assert!(cmd.try_get_matches_from(["nb"]).is_err());
    }

    #[test]
    fn test_nb_invalid_subcommands() {
        let cmd = build_command();
        assert!(cmd.clone().try_get_matches_from(["nb", " "]).is_err());
        assert!(cmd.clone().try_get_matches_from(["nb", "test"]).is_err());
        assert!(cmd.clone().try_get_matches_from(["nb", "-t"]).is_err());
        assert!(cmd.clone().try_get_matches_from(["nb", "--test"]).is_err());
    }

    #[test]
    fn test_new_no_name() {
        let cmd = build_command();
        assert!(cmd.clone().try_get_matches_from(["nb", "new"]).is_err());
        assert!(cmd.clone().try_get_matches_from(["nb", "new", ""]).is_err());
        assert!(cmd
            .clone()
            .try_get_matches_from(["nb", "new", " "])
            .is_err());
        assert!(cmd
            .clone()
            .try_get_matches_from(["nb", "new", "\r"])
            .is_err());
        assert!(cmd
            .clone()
            .try_get_matches_from(["nb", "new", "\n"])
            .is_err());
        assert!(cmd
            .clone()
            .try_get_matches_from(["nb", "new", "\t"])
            .is_err());
    }

    #[test]
    fn test_new_multiple_names() {
        let cmd = build_command();
        assert!(cmd.try_get_matches_from(["nb", "new", "a", "b"]).is_err());
    }

    #[test]
    fn test_new() {
        let cmd = build_command();
        let matches = cmd.clone().get_matches_from(["nb", "new", "my_notebook"]);
        assert!(matches.try_get_one::<String>("name").is_err());
        let (subcommand, matches) = matches.subcommand().unwrap();
        assert_eq!(subcommand, "new");
        assert_eq!(matches.get_one::<String>("name").unwrap(), "my_notebook");
    }

    #[test]
    fn test_open_multiple_names() {
        let cmd = build_command();
        assert!(cmd.try_get_matches_from(["nb", "open", "a", "b"]).is_err());
    }

    #[test]
    fn test_open() {
        let cmd = build_command();
        let matches = cmd
            .clone()
            .try_get_matches_from(["nb", "open", "my_notebook"])
            .unwrap();
        assert!(matches.try_get_one::<String>("name").is_err());
        let (subcommand, matches) = matches.subcommand().unwrap();
        assert_eq!(subcommand, "open");
        assert_eq!(matches.get_one::<String>("name").unwrap(), "my_notebook");
    }

    #[test]
    fn test_completions_no_shell() {
        let cmd = build_command();
        assert!(cmd.try_get_matches_from(["nb", "completions"]).is_err());
    }

    #[test]
    fn test_completions_no_argument_name() {
        let cmd = build_command();
        assert!(cmd
            .try_get_matches_from(["nb", "completions", "zsh"])
            .is_err());
    }

    #[test]
    fn test_completions() {
        let cmd = build_command();
        let matches = cmd
            .clone()
            .get_matches_from(["nb", "completions", "-s", "zsh"]);
        let (subcommand, matches) = matches.subcommand().unwrap();
        assert_eq!(subcommand, "completions");
        assert_eq!(*matches.get_one::<Shell>("shell").unwrap(), Shell::Zsh);

        let matches = cmd
            .clone()
            .get_matches_from(["nb", "completions", "--shell", "zsh"]);
        let (subcommand, matches) = matches.subcommand().unwrap();
        assert_eq!(subcommand, "completions");
        assert_eq!(*matches.get_one::<Shell>("shell").unwrap(), Shell::Zsh);
    }
}
