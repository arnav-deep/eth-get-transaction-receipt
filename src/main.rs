use std::error::Error;
use std::io;
use std::str::FromStr;

use ethers::prelude::*;
use ethers::providers::Provider;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let http_provider: Provider<Http>;
    loop {
        let current_rpc = "https://sepolia-rollup.arbitrum.io/rpc";
        println!("Current RPC URL is: {}", current_rpc);
        println!("Enter '1' to use the current RPC URL or enter a new RPC URL:");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let current_rpc = if input.trim() == "1" {
            current_rpc
        } else {
            input.trim()
        };

        let provider = Provider::<Http>::try_from(current_rpc);
        if provider.is_err() {
            println!("Error creating provider: {:?}", provider.err().unwrap());
            continue;
        }
        http_provider = provider.unwrap();
        println!("RPC Provider created successfully");
        break;
    }

    loop {
        let mut tx_hash_str = String::new();
        println!("Enter the transaction hash:");
        io::stdin().read_line(&mut tx_hash_str)?;

        // Trim the newline character from the input and parse the transaction hash
        let tx_hash_str = tx_hash_str.trim();
        let tx_hash = H256::from_str(tx_hash_str);
        if tx_hash.is_err() {
            println!(
                "Error parsing transaction hash: {:?}",
                tx_hash.err().unwrap()
            );
            // enter 1 to retry
            let mut input = String::new();
            println!("Enter '1' to retry or any other key to exit:");
            io::stdin().read_line(&mut input)?;
            if input.trim() == "1" {
                continue;
            } else {
                break;
            }
        }
        let tx_hash = tx_hash.unwrap();

        // Get the transaction receipt
        let res = http_provider.get_transaction_receipt(tx_hash).await;
        if res.is_err() {
            println!(
                "Error fetching transaction receipt: {:?}",
                res.err().unwrap()
            );
            // enter 1 to retry
            let mut input = String::new();
            println!("Enter '1' to retry or any other key to exit:");
            io::stdin().read_line(&mut input)?;
            if input.trim() == "1" {
                continue;
            } else {
                break;
            }
        } else {
            let receipt = res.unwrap();
            if receipt.is_none() {
                println!("Transaction Receipt not found");
            } else {
                println!("Transaction Receipt: {:?}", receipt.unwrap());
            }
        }
        // enter 1 to try with another transaction hash or any other key to exit
        let mut input = String::new();
        println!("Enter '1' to try with another transaction hash or any other key to exit:");
        io::stdin().read_line(&mut input)?;
        if input.trim() != "1" {
            break;
        }
    }

    Ok(())
}
