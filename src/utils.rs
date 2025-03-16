use crate::models::TransactionResult;
use prettytable::{format, row, Table};
use std::time::Duration;

pub fn create_results_table(transaction_results: &[TransactionResult]) -> (Table, Duration, i32) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);

    table.add_row(row![
        "Signature",
        "Sender",
        "Receiver",
        "Time (ms)",
        "Status",
        "Amount SOL"
    ]);

    let mut total_time = Duration::from_secs(0);
    let mut successful = 0;

    for result in transaction_results {
        table.add_row(row![
            result.signature.to_string(),
            result.source,
            result.destination,
            result.execution_time.as_millis(),
            result.status,
            format!("{:.4}", result.amount_sol)
        ]);

        total_time += result.execution_time;
        if result.status == "Success" {
            successful += 1;
        }
    }

    (table, total_time, successful)
}

pub fn print_statistics(
    transaction_results: &[TransactionResult],
    total_time: Duration,
    successful: i32,
) {
    let avg_time = if !transaction_results.is_empty() {
        total_time.as_millis() as f64 / transaction_results.len() as f64
    } else {
        0.0
    };

    println!("\nStatistics:");
    println!("Total transactions: {}", transaction_results.len());
    println!("Successful transactions: {}", successful);
    println!("Average execution time: {:.2} ms", avg_time);
}
