use crate::models::Config;
use anyhow::{Context, Result};
use std::{fs::File, io::BufReader, path::PathBuf};

pub fn load_config(path: &PathBuf) -> Result<Config> {
    let file = File::open(path).context("Failed to open config file")?;
    let reader = BufReader::new(file);
    serde_yaml::from_reader(reader).context("Failed to read config file")
}
