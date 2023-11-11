use std::path::PathBuf;

pub use carpenter_derive::ConfigManager;
use directories::ProjectDirs;

pub struct ConfigPath { 
    pub inner: PathBuf,
} 

impl ConfigPath {
    pub fn new(organization: &str, appication: &str) -> Self {
        let project_dir = ProjectDirs::from("com", organization,  appication).unwrap();
        Self { 
            inner: project_dir.config_dir().to_path_buf(),
        }
    }
}