//! Hyper-powered HTTP protocols for Hotaru applications running on std and Tokio.
//!
//! This crate is independent of `hotaru_http`. Hyper owns HTTP parsing and
//! connection state while Hotaru continues to provide protocol registration,
//! routing, middleware, and endpoint execution.

pub mod channel;
pub mod context;
pub mod error;
mod io;
pub mod message;
pub mod protocol;
pub mod response;
pub mod safety;

pub use bytes::Bytes;
pub use channel::HyperChannel;
pub use context::{HyperContext, HyperH1Context, HyperH2Context};
pub use error::HyperHttpError;
pub use hyper::{HeaderMap, Method, StatusCode, Uri, Version};
pub use message::{HyperRequest, HyperResponse};
pub use protocol::{HyperH1, HyperH2, HyperHttp1Protocol, HyperHttp2Protocol};
pub use safety::{HyperHttpConfig, HyperSafety};

/// Creates a UTF-8 text response with a `text/plain` content type.
pub fn text_response(body: impl Into<String>) -> HyperResponse {
    HyperResponse::text(body)
}
