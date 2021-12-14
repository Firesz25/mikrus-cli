use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::view::promt;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub srv: String,
    pub key: String,
}

impl Config {
    pub fn new(srv: String, key: String) -> Self {
        Config { srv, key }
    }

    pub fn from_file() -> Result<Self> {
        let path = directories::ProjectDirs::from("", "", "mikrus-cli").unwrap();
        let file = std::fs::read_to_string(path.config_dir().join("config.ron")).unwrap();
        Self::from_str(&file)
    }

    fn from_str(str: &str) -> Result<Self> {
        let config: Config = match ron::from_str(str) {
            Ok(config) => config,
            Err(_) => {
                println!("Please input server number");
                let srv = promt().unwrap();
                println!("Please input key");
                let key = promt().unwrap();
                let config = Config::new(srv, key);
                println!("save config to file? (y/n)");
                let save = promt().unwrap();
                if save == "y" {
                    config.save()?;
                }
                config
            }
        };
        Ok(config)
    }

    fn to_file(&self, path: PathBuf) -> Result<()> {
        let file = ron::to_string(self).unwrap();
        std::fs::write(path, file).unwrap();
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let path = directories::ProjectDirs::from("", "", "mikrus-cli").unwrap();
        self.to_file(path.config_dir().join("config.ron")).unwrap();
        Ok(())
    }
}

pub fn create_config_file() -> std::io::Result<()> {
    let dirs = directories::ProjectDirs::from("", "", "mikrus-cli").unwrap();
    std::fs::create_dir_all(dirs.config_dir()).unwrap();
    if !dirs.config_dir().join("config.ron").exists() {
        std::fs::File::create(dirs.config_dir().join("config.ron")).unwrap();
    }
    Ok(())
}
