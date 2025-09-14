use std::{fs, path::PathBuf};

pub fn get_notebooks(notebook_dir: &PathBuf) -> std::io::Result<Vec<String>>{
    let file_iter = fs::read_dir(notebook_dir)?;
    let notebooks: Vec<String> = file_iter
        .map(|file| {file.unwrap().file_name().into_string().unwrap()})
        .collect();
    Ok(notebooks)
}

pub fn create_notebook(notebook_path: &PathBuf) -> std::io::Result<()>{
    fs::File::create_new(notebook_path)?;
    Ok(())
}
