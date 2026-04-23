use anyhow::Error;
use anyhow::Result;
use std::{
    fs::{self, File},
    path::Path,
    process::Command,
};

use crate::error::FileSystemError;

pub trait FileOperations {
    fn get_files(&self, dir: &Path) -> Result<Vec<String>>;
    fn delete_file(&mut self, path: &Path) -> Result<()>;
    fn create_file(&mut self, path: &Path) -> Result<()>;
    fn create_dir(&mut self, path: &Path) -> Result<()>;
    fn open_file(&mut self, editor_command: &str, path: &Path) -> Result<()>;
    fn exists(&self, path: &Path) -> Result<bool>;
    fn read_file(&self, path: &Path) -> Result<String>;
    fn write_file(&mut self, path: &Path, value: &str) -> Result<()>;
    fn copy(&mut self, source_path: &Path, destination_path: &Path) -> Result<()>;
}

pub struct FileSystem;
impl FileOperations for FileSystem {
    fn get_files(&self, dir: &Path) -> Result<Vec<String>> {
        let files: Vec<String> = fs::read_dir(dir)?
            .map(|file| file.unwrap().file_name().into_string().unwrap())
            .filter(|file| !file.starts_with("."))
            .collect();
        Ok(files)
    }

    fn delete_file(&mut self, path: &Path) -> Result<()> {
        fs::remove_file(path).map_err(Error::from)
    }

    fn create_file(&mut self, path: &Path) -> Result<()> {
        File::create_new(path).map_err(Error::from).map(|_| ())
    }

    fn create_dir(&mut self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).map_err(Error::from)
    }

    fn open_file(&mut self, editor_command: &str, path: &Path) -> Result<()> {
        if !path.is_file() {
            return Err(FileSystemError::NotAFile(path.to_path_buf()).into());
        }
        Command::new(editor_command)
            .arg(path.as_os_str())
            .status()
            .map(|_| ())
            .map_err(|e| e.into())
    }

    fn exists(&self, path: &Path) -> Result<bool> {
        fs::exists(path).map_err(Into::into)
    }

    fn read_file(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path).map_err(Into::into)
    }

    // TODO remove create_dir_all
    fn write_file(&mut self, file_path: &Path, content: &str) -> Result<()> {
        let mut dir_path = file_path.to_path_buf();
        dir_path.pop();
        fs::create_dir_all(dir_path)?;
        fs::write(file_path, content).map_err(Into::into)
    }

    fn copy(&mut self, source_path: &Path, destination_path: &Path) -> Result<()> {
        fs::copy(source_path, destination_path).map(|_|()).map_err(Into::into)
    }
}
