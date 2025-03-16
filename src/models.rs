use clap::Parser;
use serde::{Deserialize, Serialize};
use solana_sdk::signature::Signature;
use std::{path::PathBuf, time::Duration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transfer {
    pub sender_keypair_path: String,
    pub receiver_address: String,
    pub amount_sol: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub transfers: Vec<Transfer>,
    pub rpc_url: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to config file
    #[arg(short, long, default_value = "config.yaml")]
    pub config: PathBuf,
}

pub struct TransactionResult {
    pub signature: Signature,
    pub source: String,
    pub destination: String,
    pub execution_time: Duration,
    pub status: String,
    pub amount_sol: f64,
}
