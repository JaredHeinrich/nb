use crate::config;

pub struct App{
    config: config::Config
}
impl App{
    pub fn new(config: config::Config) -> Self{
        App { config }
    }

    pub fn list_notebooks(&self) -> Message
    

    fn create_notebook(&self, name: &str) -> std::io::Result<()>{
        let mut notebook_path: PathBuf = self.config.notebook_dir.to_owned();
        notebook_path.push(name);
        fs::File::create_new(notebook_path)?;
        Ok(())
    }
}
