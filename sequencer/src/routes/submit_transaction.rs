use crate::error::ApiError;
use crate::sequencer::SequencerContext;
use crate::sequencer::SequencerRpcMethods;
use crate::transaction::SignedTransaction;
use axum::Json;
use axum::extract::State;
use serde::Serialize;
use std::sync::Arc;

// todo return transaction commit hash
#[derive(Debug, Serialize)]
pub struct Response {
    tx_commit: String,
}

pub async fn handle_submit_transaction(
    State(ctx): State<Arc<SequencerContext>>,
    Json(payload): Json<SignedTransaction>,
) -> Result<Json<Response>, ApiError> {
    ctx.accept_tx(payload.encode())?;

    Ok(Json(Response {
        tx_commit: "trasnacrion_hash".to_string(),
    }))
}
