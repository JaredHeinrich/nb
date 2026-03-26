use clap::Subcommand as ClapSubcommand;
use clap::{Args, Parser, ValueEnum};

#[derive(Parser)]
#[command(version = "0.1.0")]
#[command(about = "CLI note book manager")]
#[command(disable_help_subcommand = true)]
#[command(flatten_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(ClapSubcommand)]
pub enum Subcommand {
    #[command(about = "Create a new note book")]
    New(NewArgs),

    #[command(about = "Open a note book")]
    Open(OpenArgs),

    #[command(about = "Delete a note book")]
    #[clap(alias = "rm")]
    Remove(RemoveArgs),

    #[command(about = "List existing note books")]
    #[clap(alias = "ls")]
    List,

    #[command(about = "Access config via cli")]
    Config(ConfigArgs),

    #[command(about = "Completion script for specific shell")]
    Completions(CompletionArgs),
}

#[derive(Args)]
pub struct NewArgs {
    #[arg(help = "Name of the note book to be created.")]
    #[arg(value_parser=non_empty_trimmed)]
    pub name: String,
}

#[derive(Args)]
pub struct OpenArgs {
    #[arg(help = "Name of the note book to open.")]
    pub name: Option<String>,
}

#[derive(Args)]
pub struct RemoveArgs {
    #[arg(help = "Name of the note book to be deleted.")]
    pub name: String,
}

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub subcommand: ConfigSubcommand,
}

#[derive(ClapSubcommand)]
pub enum ConfigSubcommand {
    #[command(about = "Generate a default config file")]
    Generate(ConfigGenerateArgs),

    #[command(about = "Get specific config values")]
    Get(ConfigGetArgs),

    #[command(about = "List all config values")]
    List,
}

#[derive(Args)]
pub struct ConfigGenerateArgs {
    #[arg(help = "Overwrite the config file if one already exists")]
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args)]
pub struct ConfigGetArgs {
    #[arg(help = "Values to get from the config")]
    #[arg(value_name = "VALUE_NAME")]
    #[arg(required = true)]
    pub value_names: Vec<String>,
}

#[derive(Args)]
pub struct CompletionArgs {
    #[arg(help = "Shell for which to return the completion script")]
    #[arg(short, long)]
    pub shell: Shell,
}

#[derive(ValueEnum, Clone, PartialEq, Debug)]
pub enum Shell {
    Zsh,
}

fn non_empty_trimmed(s: &str) -> Result<String, String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        Err("Name must not be empty".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nb_no_subcommand() {
        assert!(Cli::try_parse_from(["nb"]).is_err());
    }

    #[test]
    fn test_nb_invalid_subcommands() {
        assert!(Cli::try_parse_from(["nb", " "]).is_err());
        assert!(Cli::try_parse_from(["nb", "test"]).is_err());
        assert!(Cli::try_parse_from(["nb", "-t"]).is_err());
        assert!(Cli::try_parse_from(["nb", "--test"]).is_err());
    }

    #[test]
    fn test_new_no_name() {
        assert!(Cli::try_parse_from(["nb", "new"]).is_err());
        assert!(Cli::try_parse_from(["nb", "new", ""]).is_err());
        assert!(Cli::try_parse_from(["nb", "new", " "]).is_err());
        assert!(Cli::try_parse_from(["nb", "new", "\r"]).is_err());
        assert!(Cli::try_parse_from(["nb", "new", "\n"]).is_err());
        assert!(Cli::try_parse_from(["nb", "new", "\t"]).is_err());
    }

    #[test]
    fn test_new_multiple_names() {
        assert!(Cli::try_parse_from(["nb", "new", "a", "b"]).is_err());
    }

    #[test]
    fn test_new() {
        let cli = Cli::parse_from(["nb", "new", "my_notebook"]);
        let Subcommand::New(args) = cli.subcommand else {
            panic!()
        };
        assert_eq!(args.name, "my_notebook");
    }

    #[test]
    fn test_open_multiple_names() {
        assert!(Cli::try_parse_from(["nb", "open", "a", "b"]).is_err());
    }

    #[test]
    fn test_open() {
        let cli = Cli::parse_from(["nb", "open", "my_notebook"]);
        let Subcommand::Open(args) = cli.subcommand else {
            panic!()
        };
        assert_eq!(args.name.as_deref(), Some("my_notebook"));
    }

    #[test]
    fn test_completions_no_shell() {
        assert!(Cli::try_parse_from(["nb", "completions"]).is_err());
    }

    #[test]
    fn test_completions_no_argument_name() {
        assert!(Cli::try_parse_from(["nb", "completions", "zsh"]).is_err());
    }

    #[test]
    fn test_completions() {
        let cli = Cli::parse_from(["nb", "completions", "-s", "zsh"]);
        let Subcommand::Completions(args) = cli.subcommand else {
            panic!()
        };
        assert_eq!(args.shell, Shell::Zsh);

        let cli = Cli::parse_from(["nb", "completions", "--shell", "zsh"]);
        let Subcommand::Completions(args) = cli.subcommand else {
            panic!()
        };
        assert_eq!(args.shell, Shell::Zsh);
    }

    #[test]
    fn test_config_wrong_subcommand() {
        assert!(Cli::try_parse_from(["nb", "config"]).is_err());
        assert!(Cli::try_parse_from(["nb", "config", "test"]).is_err());
    }

    #[test]
    fn test_config_generate() {
        assert!(Cli::try_parse_from(["nb", "config", "generate", "--test"]).is_err());

        let cli = Cli::parse_from(["nb", "config", "generate"]);
        let Subcommand::Config(config_args) = cli.subcommand else {
            panic!()
        };
        let ConfigSubcommand::Generate(generate_args) = config_args.subcommand else {
            panic!()
        };
        assert!(!generate_args.force);

        let cli = Cli::parse_from(["nb", "config", "generate", "--force"]);
        let Subcommand::Config(config_args) = cli.subcommand else {
            panic!()
        };
        let ConfigSubcommand::Generate(generate_args) = config_args.subcommand else {
            panic!()
        };
        assert!(generate_args.force);

        let cli = Cli::parse_from(["nb", "config", "generate", "-f"]);
        let Subcommand::Config(config_args) = cli.subcommand else {
            panic!()
        };
        let ConfigSubcommand::Generate(generate_args) = config_args.subcommand else {
            panic!()
        };
        assert!(generate_args.force);
    }

    #[test]
    fn test_config_get() {
        assert!(Cli::try_parse_from(["nb", "config", "get"]).is_err());

        let cli = Cli::parse_from(["nb", "config", "get", "value_name"]);
        let Subcommand::Config(config_args) = cli.subcommand else {
            panic!()
        };
        let ConfigSubcommand::Get(get_args) = config_args.subcommand else {
            panic!()
        };
        assert_eq!(get_args.value_names, ["value_name"]);

        let cli = Cli::parse_from(["nb", "config", "get", "value_name_1", "value_name_2"]);
        let Subcommand::Config(config_args) = cli.subcommand else {
            panic!()
        };
        let ConfigSubcommand::Get(get_args) = config_args.subcommand else {
            panic!()
        };
        assert_eq!(get_args.value_names, ["value_name_1", "value_name_2"]);
    }

    #[test]
    fn test_config_list() {
        assert!(Cli::try_parse_from(["nb", "config", "list", "test"]).is_err());

        let cli = Cli::parse_from(["nb", "config", "list"]);
        let Subcommand::Config(config_args) = cli.subcommand else {
            panic!()
        };
        assert!(matches!(config_args.subcommand, ConfigSubcommand::List));
    }
}
