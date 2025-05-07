use axum::Json;
use axum::extract::State;
use serde::Serialize;
use std::sync::Arc;

use crate::sequencer::SequencerContext;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SequencerInfo {
    /// Sequencer version
    pub version: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(flatten)]
    sequencer_info: SequencerInfo,
}

pub async fn handle_sequencer_info(State(ctx): State<Arc<SequencerContext>>) -> Json<Response> {
    let sequencer_info = SequencerInfo {
        version: "v0.0.1-rc1".to_string(),
    };
    Json(Response { sequencer_info })
}
