mod transaction;
mod keypair;
mod types;

pub use transaction::{send_transaction, check_transaction_status};
pub use keypair::read_keypair;
pub use types::{PendingTransaction, TransferProcessor};

use crate::models::{TransactionResult, Transfer};
use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    system_instruction,
    transaction::Transaction,
};
use std::{
    str::FromStr,
    time::{Duration, Instant},
    path::PathBuf,
};
use solana_transaction_status::TransactionConfirmationStatus;
use dirs::home_dir;
use std::fs;
use serde_json;


pub async fn send_transaction(
    client: &RpcClient,
    source_keypair: &Keypair,
    destination: &Pubkey,
    amount_lamports: u64,
) -> Result<(Signature, Duration)> {
    println!("Sending transaction...");
    let start_time = Instant::now();

    let instruction =
        system_instruction::transfer(&source_keypair.pubkey(), destination, amount_lamports);

    let message = Message::new(&[instruction], Some(&source_keypair.pubkey()));
    let transaction = Transaction::new(&[source_keypair], message, client.get_latest_blockhash().await?);

    let signature = client.send_transaction(&transaction).await?;
    let duration = start_time.elapsed();
    println!("Transaction sent: {}", signature);
    Ok((signature, duration))
}

pub async fn check_transaction_status(client: &RpcClient, signature: &Signature) -> Result<String> {
    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 30;

    loop {
        let statuses = client.get_signature_statuses(&[*signature]).await?;
        
        if let Some(status) = statuses.value[0].as_ref() {
            match status.confirmation_status {
                Some(TransactionConfirmationStatus::Finalized) => {
                    if let Some(err) = status.err.as_ref() {
                        return Ok(format!("Error: {:?}", err));
                    }
                    return Ok("Finalized".to_string());
                }
                Some(TransactionConfirmationStatus::Confirmed) => {
                    println!("Transaction confirmed, waiting for finalization...");
                    return Ok("Confirmed".to_string());
                }
                Some(TransactionConfirmationStatus::Processed) => {
                    println!("Transaction processed, waiting for confirmation...");
                    return Ok("Processed".to_string());
                }
                None => {
                    println!("Transaction status unknown, waiting...");
                }
            }

            if let Some(err) = &status.err {
                return Ok(format!("Error: {:?}", err));
            }
        } else {
            println!("Waiting for transaction processing...");
        }

        attempts += 1;
        if attempts >= MAX_ATTEMPTS {
            return Ok("Timeout waiting for finalization".to_string());
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

pub struct PendingTransaction {
    pub signature: Signature,
    pub source: String,
    pub destination: String,
    pub send_time: Duration,
    pub amount_sol: f64,
}

pub fn read_keypair(path: &str) -> Result<Keypair> {
    let expanded_path = if path.starts_with("~/") {
        let mut path_buf = PathBuf::from(home_dir().expect("Could not find home directory"));
        path_buf.push(&path[2..]);
        path_buf
    } else {
        PathBuf::from(path)
    };

    let keypair_bytes = fs::read_to_string(expanded_path)?;
    let keypair: Vec<u8> = serde_json::from_str(&keypair_bytes)?;
    Ok(Keypair::from_bytes(&keypair)?)
}

pub async fn process_single_transfer(
    client: &RpcClient,
    transfer: &Transfer,
) -> Option<PendingTransaction> {
    let now = chrono::Local::now();
    println!("Processing single transfer at {}...", now.format("%Y-%m-%d %H:%M:%S.%3f"));
    let amount_lamports = (transfer.amount_sol * 1_000_000_000.0) as u64;

    let source_keypair = match read_keypair(&transfer.sender_keypair_path) {
        Ok(keypair) => keypair,
        Err(e) => {
            println!("Error reading keypair file: {}", e);
            return None;
        }
    };

    let destination_pubkey = match Pubkey::from_str(&transfer.receiver_address) {
        Ok(pubkey) => pubkey,
        Err(e) => {
            println!(
                "Error with receiver address {}: {}",
                transfer.receiver_address, e
            );
            return None;
        }
    };

    match send_transaction(
        client,
        &source_keypair,
        &destination_pubkey,
        amount_lamports,
    )
    .await
    {
        Ok((signature, time)) => {
            let now = chrono::Local::now();
            println!("Transaction sent at {}...", now.format("%Y-%m-%d %H:%M:%S.%3f"));
            println!(
                "Transaction sent: {} -> {} ({}) amount: {} SOL",
                source_keypair.pubkey(),
                transfer.receiver_address,
                signature,
                transfer.amount_sol
            );

            Some(PendingTransaction {
                signature,
                source: source_keypair.pubkey().to_string(),
                destination: transfer.receiver_address.clone(),
                send_time: time,
                amount_sol: transfer.amount_sol,
            })
        }
        Err(e) => {
            println!(
                "Error sending from {} to {}: {}",
                source_keypair.pubkey(),
                transfer.receiver_address,
                e
            );
            None
        }
    }
}

pub async fn confirm_transaction(
    client: &RpcClient,
    pending: PendingTransaction,
) -> Option<TransactionResult> {
    let status = match check_transaction_status(client, &pending.signature).await {
        Ok(status) => status,
        Err(_) => "Unknown status".to_string(),
    };

    println!("Final status for {}: {}", pending.signature, status);

    Some(TransactionResult {
        signature: pending.signature,
        source: pending.source,
        destination: pending.destination,
        execution_time: pending.send_time,
        status,
        amount_sol: pending.amount_sol,
    })
}
