

use std::{error::Error, fmt::Debug};
use super::{BlockHeaderT, BlockT};

pub trait AppState {
        /// Returns the current state root (32-byte hash)
        fn state_root(&self) -> [u8; 32];
        
        /// Returns the previous state root (for rollback/verification)
        fn previous_state_root(&self) -> Option<[u8; 32]>;
    }


pub trait StateTransitionFunction {   

        type State: AppState;
        type Error: Error + Debug + Send + Sync;
        type BlockHeader: BlockHeaderT;
        type Block: BlockT;

        // Validate block
        fn validate_block(state: &Self::State, block: &Self::Block) -> Result<(), Self::Error>;

        // Apply block
        fn apply_block(state: &mut Self::State, block: &Self::Block) -> Result<(), Self::Error>;
    
}    