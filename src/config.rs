use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub nb_dir: PathBuf,
    pub default_file: String,
    pub editor: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut nb_dir: PathBuf = dirs::home_dir().unwrap();
        nb_dir.push(".note_books");
        let default_file = String::from("nb");
        let editor = String::from("nvim");
        Self {
            nb_dir,
            default_file,
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
