//! Experimental Hyper integration for Hotaru master.
//!
//! The prototype intentionally targets Tokio TCP servers. It proves that
//! Hyper can own a Hotaru `Protocol` connection without changing `hotaru_core`.

mod io;
mod message;
mod traits;

pub use bytes::Bytes;
pub use hyper::{HeaderMap, Method, StatusCode, Uri, Version};
pub use message::{HyperRequest, HyperResponse};
pub use traits::{
    H1, H1Context, H2, H2Context, H2perError, H3, HyperChannel, HyperContext, HyperHttp1Protocol,
    HyperHttp2Protocol, HyperHttp3Protocol,
};

/// Convenience response constructor for Hotaru endpoint bodies.
pub fn text_response(body: impl Into<String>) -> HyperResponse {
    HyperResponse::text(body)
}
