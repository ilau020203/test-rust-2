use anyhow::Result;
use dirs::home_dir;
use serde_json;
use solana_sdk::signature::Keypair;
use std::{fs, path::PathBuf};

pub fn read_keypair(path: &str) -> Result<Keypair> {
    // Существующая реализация
} 