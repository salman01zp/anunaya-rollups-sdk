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


use alloy::primitives::SignatureError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = std::result::Result<T, SequencerError>;

#[derive(Debug, thiserror::Error)]
pub enum SequencerError {
    /// An Io error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Generic error.
    #[error("{}", _0)]
    Generic(&'static str),
    /// Transaction verification
    #[error(transparent)]
    SignatureError(#[from] SignatureError),
    /// Transaction store error
    #[error(transparent)]
    TxStoreError(#[from] TxStoreError),
}

#[derive(Debug, thiserror::Error)]
pub enum TxStoreError {
    /// Mempool is full
    #[error("Mempool is full")]
    MempoolFull,
    /// Index out of bounds
    #[error("Index out of bounds")]
    IndexOutOfBounds,
    /// Failed to acquire lock
    #[error("Failed to acquire lock")]
    LockError,
}

impl From<SequencerError> for ApiError {
    fn from(value: SequencerError) -> Self {
        ApiError(StatusCode::INTERNAL_SERVER_ERROR, value.to_string())
    }
}

/// Error type for HTTP handlers
pub struct ApiError(
    /// HTTP status code for response
    pub StatusCode,
    /// Response message
    pub String,
);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}
