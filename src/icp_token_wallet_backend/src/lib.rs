use candid::CandidType;
// Use the correct imports for ICP development
use ic_cdk::api::caller;
use ic_cdk::storage;
use ic_cdk_macros::{init, query, update};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define the Token structure
#[derive(CandidType,Serialize, Deserialize, Clone, Debug)]
pub struct Token {
    symbol: String,
    name: String,
    decimals: u8,
    total_supply: u64,
}

// Define the Wallet structure
#[derive(CandidType,Serialize, Deserialize, Clone, Debug)]
pub struct Wallet {
    balances: HashMap<String, u64>, // Maps account IDs to balances
}

// Implement the Wallet structure
impl Wallet {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    // Send tokens from one account to another
    pub fn send_token(&mut self, from: String, to: String, amount: u64) -> Result<(), String> {
        let sender_balance = self.balances.entry(from.clone()).or_insert(0);
        if *sender_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        *sender_balance -= amount;
        let recipient_balance = self.balances.entry(to).or_insert(0);
        *recipient_balance += amount;
        Ok(())
    }

    // Receive tokens into an account
    pub fn receive_token(&mut self, to: String, amount: u64) {
        let recipient_balance = self.balances.entry(to).or_insert(0);
        *recipient_balance += amount;
    }

    // Get the balance of an account
    pub fn get_balance(&self, owner: String) -> u64 {
        *self.balances.get(&owner).unwrap_or(&0)
    }
}

// Initialize the token and wallet
#[init]
fn init(symbol: String, name: String, decimals: u8, total_supply: u64) {
    let token = Token {
        symbol,
        name,
        decimals,
        total_supply,
    };

    let mut wallet = Wallet::new();
    wallet.receive_token("admin".to_string(), total_supply); // Assign total supply to admin

    // Store the token and wallet in stable storage
    storage::stable_save((token, wallet)).expect("Failed to save token and wallet");
}

// Send tokens from the caller's account to another account
#[update]
fn send_token(to: String, amount: u64) -> Result<(), String> {
    let caller = caller().to_string(); // Get the caller's identity
    let (token, mut wallet) = storage::stable_restore::<(Token, Wallet)>().expect("Failed to restore token and wallet");

    wallet.send_token(caller, to, amount)?;
    storage::stable_save((token, wallet)).expect("Failed to save wallet");
    Ok(())
}

// Get the balance of the caller's account
#[query]
fn get_balance() -> u64 {
    let caller = caller().to_string(); // Get the caller's identity
    let (_, wallet) = storage::stable_restore::<(Token, Wallet)>().expect("Failed to restore wallet");
    wallet.get_balance(caller)
}

