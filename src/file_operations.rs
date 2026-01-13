use anyhow::Error;
use anyhow::Result;
use std::{
    fs::{self, File},
    path::PathBuf,
    process::Command,
};

pub fn get_files(dir: &PathBuf) -> Result<Vec<String>> {
    let file_iter = fs::read_dir(dir)?;
    let files: Vec<String> = file_iter
        .map(|file| file.unwrap().file_name().into_string().unwrap())
        .collect();
    Ok(files)
}

pub fn create_file(path: &PathBuf) -> Result<File> {
    File::create_new(path).map_err(Error::from)
}

pub fn create_dir(path: &PathBuf) -> Result<()> {
    fs::create_dir_all(path).map_err(Error::from)
}

pub fn open_file(editor_command: &str, path: &PathBuf) -> Result<()> {
    Command::new(editor_command)
        .arg(path.as_os_str())
        .status()
        .map(|_| ())
        .map_err(|e| e.into())
}
