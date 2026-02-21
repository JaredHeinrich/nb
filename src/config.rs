use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub default_notebook: String,
    pub editor: String,
}

impl Default for Config {
    fn default() -> Self {
        let default_notebook = String::from("nb");
        let editor = String::from("nvim");
        Self {
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
