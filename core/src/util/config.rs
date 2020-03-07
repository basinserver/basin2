use basin2_lib::result::*;
use log::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{Read, Write};
use pkg_version::*;
use super::encrypt;
use openssl::rsa::Rsa;
use openssl::pkey::Private;

fn default_env(name: &str, default: &str) -> String {
    env::var(name).unwrap_or(default.to_string())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub bind_address: String,
    pub bind_port: u16,
    pub server_description: String,
    pub max_players: u32,
    pub online_mode: bool,
    pub compression_threshold: Option<u32>,
    pub difficulty: String,
    pub world_format: String,
    pub world_directory: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            bind_address: "0.0.0.0".to_string(),
            bind_port: 25565,
            server_description: "A Basin Server".to_string(),
            max_players: 100,
            online_mode: true,
            compression_threshold: Some(256),
            difficulty: "normal".to_string(),
            world_format: "anvil".to_string(),
            world_directory: "./world/".to_string(),
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
    pub static ref PUBLIC_KEY: (Rsa<Private>, Vec<u8>) = { encrypt::init_encryption() };
    pub static ref MC_VERSION: String = { format!("1.{}.{}", pkg_version_major!(), pkg_version_minor!()) };
}
