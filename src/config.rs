use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::file_operations::FileOperations;

pub mod value_names {
    pub const EDITOR: &str = "editor";

    pub const ALL: [&str; 1] = [EDITOR];
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
    pub editor: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
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
        if let Some(editor) = partial_config.editor {
            self.editor = editor;
        }
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Config {
    fn to_string(&self) -> String {
        toml::to_string(self).expect("Failed serialization of `Config`")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PartialConfig {
    pub editor: Option<String>,
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
