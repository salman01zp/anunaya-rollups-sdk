use serde::{Serialize,Deserialize};
use alloy::primitives::Address;
use crate::errors::TokenDappRollupError;
use crate::types::*;
use anunaya_rollup_core::traits::*;
use anunaya_rollup_core::block::*;
use anunaya_rollup_core::hasher::KeccakHasher;


use std::collections::BTreeMap;
#[derive(Debug, Default, Clone, Serialize, Deserialize,)]
pub struct TokenDappState {
    accounts: BTreeMap<Address, Account>,
    state_root : Hash,
    prev_state_root: Option<Hash>, // Previous state root
    block_hash: Option<Hash>, // Hash of most recent block
}

impl AppState for TokenDappState {
    fn state_root(&self) -> [u8; 32] {
            self.state_root
    }

    fn previous_state_root(&self) -> Option<[u8; 32]> {
        self.prev_state_root
    }
}

pub struct TokenDappRollup;

    
impl StateTransitionFunction for TokenDappRollup {
    type State = TokenDappState;

    type Error = TokenDappRollupError;
    type BlockHeader = BlockHeader<BlockNumber,KeccakHasher>;
    type Block = Block<Self::BlockHeader,Transaction>;

    fn validate_block(state: &Self::State, block: &Self::Block) -> Result<(), Self::Error> {
        todo!()
    }

    fn apply_block(state: &mut Self::State, block: &Self::Block) -> Result<(), Self::Error> {
        todo!()
    }
}
