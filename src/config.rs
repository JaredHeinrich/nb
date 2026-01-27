use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub nb_root_dir: PathBuf,
    pub default_notebook: String,
    pub editor: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut nb_root_dir: PathBuf = dirs::home_dir().unwrap();
        nb_root_dir.push(".notebooks");
        let default_notebook = String::from("nb");
        let editor = String::from("nvim");
        Self {
            nb_root_dir,
            default_notebook,
            editor,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        confy::change_config_strategy(confy::ConfigStrategy::App);
        confy::load("nb", "nb").unwrap()
    }
}
