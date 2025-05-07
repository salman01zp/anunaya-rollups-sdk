mod info;

use std::net::SocketAddr;
use std::sync::Arc;

use crate::error::Result;
use crate::sequencer::SequencerContext;
use axum::Router;
use axum::routing::{get, post};
use info::handle_sequencer_info;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub async fn build_api_services(ctx: SequencerContext) -> Result<()> {
    // Todo get api port from sequencer config.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3033").await.unwrap();

    let api = Router::new()
        .route("/info", get(handle_sequencer_info))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let app = Router::new().nest("/api/v1", api).with_state(Arc::new(ctx));

    tracing::info!("Starting the server on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
