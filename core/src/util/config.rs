use crate::result::*;
use log::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{Read, Write};

fn default_env(name: &str, default: &str) -> String {
    env::var(name).unwrap_or(default.to_string())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub bind_address: String,
    pub bind_port: u16,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            bind_address: "0.0.0.0".to_string(),
            bind_port: 25565,
        }
    }
}

fn load_config(config_file: String) -> Result<Config> {
    let mut file = fs::File::open(config_file)?;
    let mut raw = String::new();
    file.read_to_string(&mut raw)?;
    Ok(serde_json::from_str(&*raw)?)
}

fn init() -> Config {
    let config_file = default_env("BASIN_CONFIG", "./config.json");
    match load_config(config_file.clone()) {
        Ok(config) => return config,
        Err(e) => {
            error!("error loading config: {:?}", e);
            let exists = fs::metadata(config_file.clone()).is_ok();
            if !exists {
                info!("config does not existing, creating default.");
                let mut file = fs::File::create(config_file).expect("could not create config file");
                let config: Config = Default::default();
                file.write_all(
                    &serde_json::to_string_pretty(&config)
                        .expect("failed to generate config")
                        .into_bytes(),
                )
                .expect("failed to write to config file");
                return config;
            } else {
                panic!("could not load or create config.");
            }
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = { init() };
}
