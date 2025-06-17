use crate::errors::Result;
use config::{Config, Environment, File};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Opts {
    pub application: ApplicationOpts,
    pub database: DatabaseOpts,
    pub auth0: Auth0Opts,
}

impl Opts {
    pub fn get_from_args() -> PathBuf {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            eprintln!("Usage: cargo run --release <config_file>");
            std::process::exit(1);
        }

        let config_path = &args[1];
        println!("Using config file: {}", config_path);

        PathBuf::from(config_path)
    }
}

#[derive(Debug, Deserialize)]
pub struct ApplicationOpts {
    pub bind: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseOpts {
    pub database_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Auth0Opts {
    pub client_id: Box<str>,
    pub client: String,
    pub client_secret: String,
    pub connection: String,
    pub dev_key_file: String,
    pub audience: String,
}

pub fn load_configurations() -> Result<Opts> {
    let config_path = Opts::get_from_args();
    let config_data = Config::new()
        .with_merged(File::from(config_path))?
        .with_merged(Environment::new().separator("_"))?;

    let data: Opts = config_data.try_into()?;
    Ok(data)
}
