// Copyright 2022-2025 Anunaya Systems.
// This file is part of Anunaya Systems.

// Anunaya Systems is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Anunaya Systems is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Anunaya Systems. If not, see <http://www.gnu.org/licenses/>.

use crate::errors::TokenDappRollupError;
use crate::types::*;
use alloy::primitives::Address;
use anunaya_rollup_core::block::*;
use anunaya_rollup_core::hasher::KeccakHasher;
use anunaya_rollup_core::traits::*;
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TokenDappState {
    accounts: BTreeMap<Address, Account>,
    state_root: Hash,
    prev_state_root: Option<Hash>, // Previous state root
    block_hash: Option<Hash>,      // Hash of most recent block
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
    type BlockHeader = BlockHeader<BlockNumber, KeccakHasher>;
    type Block = Block<Self::BlockHeader, Transaction>;

    fn validate_block(state: &Self::State, block: &Self::Block) -> Result<(), Self::Error> {
        todo!()
    }

    fn apply_block(state: &mut Self::State, block: &Self::Block) -> Result<(), Self::Error> {
        todo!()
    }
}
