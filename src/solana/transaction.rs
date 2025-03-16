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
use solana_transaction_status::TransactionConfirmationStatus;
use std::time::{Duration, Instant};

pub async fn send_transaction(
    client: &RpcClient,
    source_keypair: &Keypair,
    destination: &Pubkey,
    amount_lamports: u64,
) -> Result<(Signature, Duration)> {
    // Существующая реализация
}

pub async fn check_transaction_status(
    client: &RpcClient,
    signature: &Signature,
) -> Result<String> {
    // Существующая реализация
} 