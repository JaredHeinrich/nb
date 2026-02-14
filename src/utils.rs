use anyhow::Result;

use crate::{config::Config, file_operations};

pub fn list_notebooks(config: &Config) -> Result<Vec<String>> {
    let nb_dir = &config.nb_root_dir;
    file_operations::get_files(nb_dir)
}
