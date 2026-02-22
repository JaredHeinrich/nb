use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::file_operations::FileOperations;

#[derive(Debug, Clone)]
pub struct Config {
    pub default_notebook: String,
    pub editor: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_notebook: String::from("nb"),
            editor: String::from("nvim"),
        }
    }
}

impl Config {
    pub fn load_or_default<FS: FileOperations>(fs: &FS) -> Self {
        let config_file = ConfigFile::load(fs);
        let mut config = Self::default();
        if let Ok(config_file) = config_file {
            if let Some(default_notebook) = config_file.default_notebook {
                config.default_notebook = default_notebook;
            }
            if let Some(editor) = config_file.editor {
                config.editor = editor;
            }

        }
        config
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigFile {
    pub default_notebook: Option<String>,
    pub editor: Option<String>,
}

impl ConfigFile {
    fn config_dir() -> PathBuf {
        let mut path = std::env::home_dir().expect("Could not retrieve home directory");
        path.push(".config");
        path
    }

    fn config_file() -> PathBuf {
        let mut path = Self::config_dir();
        path.push("nb");
        path.push("nb.toml");
        path
    }

    pub fn load<FS: FileOperations>(fs: &FS) -> Result<Self> {
        let config_file = Self::config_file();
        let content = fs.read_to_string(&config_file)?;
        let config: ConfigFile = toml::from_str(&content)?;
        Ok(config)
        
    }

    pub fn write_config() -> Result<()> {
        todo!("Not yet implemented");
    }
}
