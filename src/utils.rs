use anyhow::Result;

use crate::{config::Config, file_operations::FileOperations};

pub fn list_notebooks<FS: FileOperations>(config: &Config, fs: &FS) -> Result<Vec<String>> {
    let nb_dir = &config.nb_root_dir;
    fs.get_files(nb_dir)
}
