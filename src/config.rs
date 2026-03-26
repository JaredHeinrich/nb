use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::file_operations::FileOperations;

pub mod value_names {
    pub const DEFAULT_NOTEBOOK: &'static str = "default_notebook";
    pub const EDITOR: &'static str = "editor";

    pub const ALL: [&'static str; 2] = [DEFAULT_NOTEBOOK, EDITOR];
}

fn config_dir() -> PathBuf {
    let mut path = std::env::home_dir().expect("Could not retrieve home directory");
    path.push(".config");
    path
}

pub fn config_file() -> PathBuf {
    let mut path = config_dir();
    path.push("nb");
    path.push("nb.toml");
    path
}

#[derive(Debug, Clone, Serialize)]
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
    pub fn build<FS: FileOperations>(fs: &FS) -> Result<Self> {
        let mut config = Self::default();
        config.apply(PartialConfig::from_config_file(fs)?);
        Ok(config)
    }

    fn apply(&mut self, partial_config: PartialConfig) {
        if let Some(default_notebook) = partial_config.default_notebook {
            self.default_notebook = default_notebook;
        }
        if let Some(editor) = partial_config.editor {
            self.editor = editor;
        }
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        toml::to_string(self).expect("Failed serialization of `Config`")
        
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialConfig {
    pub default_notebook: Option<String>,
    pub editor: Option<String>,
}

impl Default for PartialConfig {
    fn default() -> Self {
        Self {
            default_notebook: None,
            editor: None
        }
    }
}

impl PartialConfig {
    pub fn from_config_file<FS: FileOperations>(fs: &FS) -> Result<Self> {
        let config_file_path = config_file();
        if let Ok(config_toml) = fs.read_file(&config_file_path) {
            return Ok(toml::from_str(&config_toml)?);
        }
        Ok(Self::default())
    }
}
