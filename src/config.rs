use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config{
    pub todo_list_dir: PathBuf,
    pub main_file: String,
    pub editor: String,
}

impl Default for Config{
    fn default() -> Self {
        let mut todo_list_dir: PathBuf = dirs::home_dir().unwrap();
        todo_list_dir.push(".todo");
        let main_file = String::from("todo");
        let editor = String::from("nvim");
        Self { todo_list_dir, main_file, editor }
    }
}

impl Config{
    pub fn load() -> Self{
        let mut config_path: PathBuf = dirs::home_dir().unwrap();
        config_path.push(".config");
        config_path.push("todo");
        config_path.push("todo.toml");
        confy::load_path(config_path).unwrap()
    } 
}
