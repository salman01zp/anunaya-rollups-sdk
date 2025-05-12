use alloy::primitives::Address;
use anunaya_rollup_core::traits::SignedTransactionT;
use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];
pub type Amount = u64;
pub type Nonce = u64;
pub type BlockNumber = u32;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Account {
    balance: Amount,
    nonce: Nonce,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub amount: Amount,
    pub destination: Address,
    pub nonce: Nonce,
}

impl SignedTransactionT for Transaction {}
