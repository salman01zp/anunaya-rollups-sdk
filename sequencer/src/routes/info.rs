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
