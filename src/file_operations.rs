use anyhow::Error;
use anyhow::Result;
use std::{
    fs::{self, File},
    path::PathBuf,
    process::Command,
};

use crate::error::FileSystemError;

pub trait FileOperations {
    fn get_files(&self, dir: &PathBuf) -> Result<Vec<String>>;
    fn delete_file(&mut self, path: &PathBuf) -> Result<()>;
    fn create_file(&mut self, path: &PathBuf) -> Result<()>;
    fn create_dir(&mut self, path: &PathBuf) -> Result<()>;
    fn open_file(&mut self, editor_command: &str, path: &PathBuf) -> Result<()>;
    fn exists(&self, path: &PathBuf) -> Result<bool>;
}

pub struct FileSystem;
impl FileOperations for FileSystem {
    fn get_files(&self, dir: &PathBuf) -> Result<Vec<String>> {
        let files: Vec<String> = fs::read_dir(dir)?
            .map(|file| file.unwrap().file_name().into_string().unwrap())
            .filter(|file| !file.starts_with("."))
            .collect();
        Ok(files)
    }

    fn delete_file(&mut self, path: &PathBuf) -> Result<()> {
        fs::remove_file(path).map_err(Error::from)
    }

    fn create_file(&mut self, path: &PathBuf) -> Result<()> {
        File::create_new(path).map_err(Error::from).map(|_| ())
    }

    fn create_dir(&mut self, path: &PathBuf) -> Result<()> {
        fs::create_dir_all(path).map_err(Error::from)
    }

    fn open_file(&mut self, editor_command: &str, path: &PathBuf) -> Result<()> {
        if !path.is_file() {
            return Err(FileSystemError::NotAFile(path.clone()).into());
        }
        Command::new(editor_command)
            .arg(path.as_os_str())
            .status()
            .map(|_| ())
            .map_err(|e| e.into())
    }
    fn exists(&self, path: &PathBuf) -> Result<bool> {
        fs::exists(path).map_err(Into::into)
    }
}
