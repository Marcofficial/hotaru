use core::convert::Infallible;
use std::sync::Arc;

use hotaru_core::protocol::RequestContext;
use hotaru_core::url::UrlRoot;
use hotaru_io_tokio::TcpTransport;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::{Request, Response, StatusCode};

use crate::message::{HyperRequest, HyperResponse};
use crate::traits::channel::HyperChannel;
use crate::traits::request_context::HyperContext;

pub(super) async fn dispatch<const VERSION: u8>(
    request: Request<Incoming>,
    channel: HyperChannel<VERSION>,
    root: Arc<UrlRoot<HyperContext<VERSION>, TcpTransport>>,
) -> Result<Response<Full<bytes::Bytes>>, Infallible> {
    let path = request.uri().path().to_owned();
    let (parts, incoming) = request.into_parts();
    let body = match incoming.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(error) => {
            let response = HyperResponse::text(format!("failed to read request body: {error}"))
                .status(StatusCode::BAD_REQUEST);
            return Ok(response.into_inner());
        }
    };

    let Some(endpoint) = root.walk_str(&path).await else {
        return Ok(HyperResponse::text("not found")
            .status(StatusCode::NOT_FOUND)
            .into_inner());
    };

    let request = HyperRequest::new(Request::from_parts(parts, body));
    let context = HyperContext::server(request, channel);
    match endpoint.run(context).await {
        Ok(context) => Ok(context.into_response().into_inner()),
        Err(error) => Ok(HyperResponse::text(format!("endpoint error: {error}"))
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .into_inner()),
    }
}

pub(super) fn parse_http_path(input: &str) -> Vec<&str> {
    if input.is_empty() {
        Vec::new()
    } else {
        input.split('/').collect()
    }
}
