use solana_sdk::signature::Signature;
use std::time::Duration;

pub struct PendingTransaction {
    pub signature: Signature,
    pub source: String,
    pub destination: String,
    pub send_time: Duration,
    pub amount_sol: f64,
}

pub struct TransferProcessor {
    client: RpcClient,
}

impl TransferProcessor {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            client: RpcClient::new(rpc_url.to_string()),
        }
    }

    pub async fn process_transfers(&self, transfers: &[Transfer]) -> Vec<TransactionResult> {
        let pending_transfers = future::join_all(
            transfers
                .iter()
                .map(|transfer| self.process_single_transfer(transfer)),
        )
        .await
        .into_iter()
        .filter_map(|r| r)
        .collect::<Vec<_>>();

        future::join_all(
            pending_transfers
                .into_iter()
                .map(|pending| self.confirm_transaction(pending)),
        )
        .await
        .into_iter()
        .filter_map(|r| r)
        .collect()
    }

    async fn process_single_transfer(&self, transfer: &Transfer) -> Option<PendingTransaction> {
        // Перенести логику из текущей process_single_transfer
    }

    async fn confirm_transaction(&self, pending: PendingTransaction) -> Option<TransactionResult> {
        // Перенести логику из текущей confirm_transaction
    }
} 