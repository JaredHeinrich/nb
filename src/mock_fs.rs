use std::path::PathBuf;

use anyhow::{anyhow, Result};

use crate::file_operations::FileOperations;

fn extract_file_name(root_dir_path: &PathBuf, file_path: &PathBuf) -> Result<String> {
    if !file_path.starts_with(root_dir_path) {
        return Err(anyhow!("File not in nb root directory"));
    }
    let root_dir_path_len = root_dir_path.iter().count();
    let file_path_len = file_path.iter().count();
    if root_dir_path_len + 1 != file_path_len {
        return Err(anyhow!("Path does not point to file in nb root directory"));
    }
    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_owned();
    Ok(file_name)
}

pub struct MockFileSystem {
    opened_files: Vec<PathBuf>,
    notebook_root_dir: PathBuf,
    files: Vec<String>,
}

impl MockFileSystem {
    pub fn new(notebook_root_dir: PathBuf, notebooks: Vec<String>) -> Self {
        Self {
            opened_files: Vec::new(),
            notebook_root_dir,
            files: notebooks,
        }
    }

    pub fn opened_files(&self) -> &Vec<PathBuf> {
        &self.opened_files
    }

    fn is_file(&self, path: &PathBuf) -> bool {
        if let Ok(file_name) = extract_file_name(&self.notebook_root_dir, path) {
            if self.files.contains(&file_name) {
                return true;
            }
        }
        false
    }

    fn is_dir(&self, path: &PathBuf) -> bool {
        if *path == self.notebook_root_dir {
            return true;
        }
        false
    }
}

impl FileOperations for MockFileSystem {
    fn get_files(&self, dir: &PathBuf) -> Result<Vec<String>> {
        if *dir == self.notebook_root_dir {
            return Ok(self.files.clone());
        }
        Err(anyhow!("Directory does not exist"))
    }

    fn delete_file(&mut self, path: &PathBuf) -> Result<()> {
        let file_name = extract_file_name(&self.notebook_root_dir, path)?;
        let file_index = self.files.iter().position(|f| *f == file_name);
        if let Some(file_index) = file_index {
            let _ = self.files.remove(file_index);
            return Ok(());
        }
        return Err(anyhow!("File does not exist"));
    }

    fn create_file(&mut self, path: &PathBuf) -> Result<()> {
        extract_file_name(&self.notebook_root_dir, path).map(|file_name| {
            self.files.push(file_name);
        })
    }

    fn create_dir(&mut self, path: &PathBuf) -> Result<()> {
        if *path == self.notebook_root_dir {
            return Err(anyhow!("{:?} already exists", self.notebook_root_dir));
        }
        Err(anyhow!("can't create directories in mock fs"))
    }

    fn open_file(&mut self, _editor_command: &str, path: &PathBuf) -> Result<()> {
        if !self.is_file(path) {
            return Err(anyhow!("Can't open because its not a file"));
        }
        self.opened_files.push(path.clone());
        Ok(())
    }

    fn exists(&self, path: &PathBuf) -> Result<bool> {
        if *path == self.notebook_root_dir {
            return Ok(true);
        }
        if let Ok(file_name) = extract_file_name(&self.notebook_root_dir, path) {
            if self.files.contains(&file_name) {
                return Ok(true);
            }
        }
        return Ok(false);
    }
}
