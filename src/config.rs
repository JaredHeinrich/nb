use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config{
    pub notebook_dir: PathBuf
}

impl Default for Config{
    fn default() -> Self {
        let mut notebook_dir: PathBuf = dirs::home_dir().unwrap();
        notebook_dir.push(".nb");
        Self { notebook_dir }
    }
}

impl Config{
    pub fn load() -> Self{
        let mut config_path: PathBuf = dirs::home_dir().unwrap();
        config_path.push(".config");
        config_path.push("nb");
        config_path.push("nb.toml");
        confy::load_path(config_path).unwrap()
    } 
}
