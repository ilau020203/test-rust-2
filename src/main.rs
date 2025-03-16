extern crate prettytable;
use anyhow::Result;
use clap::Parser;
use futures::future;
use solana_client::nonblocking::rpc_client::RpcClient;

mod config;
mod models;
mod solana;
mod utils;

use crate::{
    config::load_config,
    models::Args,
    solana::{process_single_transfer, confirm_transaction},
    utils::{create_results_table, print_statistics},
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = load_config(&args.config)?;

    let client = RpcClient::new(config.rpc_url.clone());

    println!("Starting SOL transfer between wallets");
    println!("Number of transfers: {}", config.transfers.len());

    let pending_transfers = future::join_all(
        config
            .transfers
            .iter()
            .map(|transfer| process_single_transfer(&client, transfer)),
    )
    .await
    .into_iter()
    .filter_map(|r| r)
    .collect::<Vec<_>>();

    let transaction_results = future::join_all(
        pending_transfers
            .into_iter()
            .map(|pending| confirm_transaction(&client, pending)),
    )
    .await
    .into_iter()
    .filter_map(|r| r)
    .collect::<Vec<_>>();

    let (table, total_time, successful) = create_results_table(&transaction_results);

    println!("\nTransaction Results:");
    table.printstd();

    print_statistics(&transaction_results, total_time, successful);

    Ok(())
}
