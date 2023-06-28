use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, num::NonZeroU64};

use crate::Error;

/// Global config object
pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::load().expect("Failed to load config"));

#[derive(Serialize, Deserialize)]
pub struct Servers {
    pub main: NonZeroU64,
    pub staff: NonZeroU64,
    pub testing: NonZeroU64,
}

impl Default for Servers {
    fn default() -> Self {
        Self {
            main: NonZeroU64::new(758641373074423808).unwrap(),
            staff: NonZeroU64::new(870950609291972618).unwrap(),
            testing: NonZeroU64::new(870952645811134475).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub token: String,
    pub servers: Servers,
    pub frontend_url: String,
    pub proxy_url: String,
    pub owners: Vec<NonZeroU64>,
    pub protected_bots: Vec<NonZeroU64>,
    pub github_pat: String,
    pub github_username: String,
    pub github_repo: String,
    pub optional_vercel_deploy_hook: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: String::from(""),
            token: String::from(""),
            servers: Servers::default(),
            roles: Roles::default(),
            channels: Channels::default(),
            frontend_url: String::from("https://infinitybots.gg"),
            proxy_url: String::from("http://127.0.0.1:3219"),
            owners: vec![NonZeroU64::new(510065483693817867).unwrap()],
            protected_bots: vec![
                NonZeroU64::new(1019662370278228028).unwrap(), // Reedwhisker (PTB) - Main Bot
            ],
            github_pat: String::from(""),
            github_username: String::from(""),
            github_repo: String::from("InfinityBotList/Infinity-Next"),
            optional_vercel_deploy_hook: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        // Delete config.yaml.sample if it exists
        if std::path::Path::new("config.yaml.sample").exists() {
            std::fs::remove_file("config.yaml.sample")?;
        }

        // Create config.yaml.sample
        let mut sample = File::create("config.yaml.sample")?;

        // Write default config to config.yaml.sample
        sample.write_all(serde_yaml::to_string(&Config::default())?.as_bytes())?;

        // Open config.yaml
        let file = File::open("config.yaml");

        match file {
            Ok(file) => {
                // Parse config.yaml
                let cfg: Config = serde_yaml::from_reader(file)?;

                // Return config
                Ok(cfg)
            }
            Err(e) => {
                // Print error
                println!("config.yaml could not be loaded: {}", e);

                // Exit
                std::process::exit(1);
            }
        }
    }
}