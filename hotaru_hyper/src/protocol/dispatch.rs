use core::convert::Infallible;
use std::sync::Arc;

use hotaru_core::app::common::RuntimeConfig;
use hotaru_core::protocol::RequestContext;
use hotaru_core::url::UrlRoot;
use hotaru_io_tokio::TcpTransport;
use http_body_util::{BodyExt, Limited};
use hyper::body::Incoming;
use hyper::{Request, Response};

use crate::channel::HyperChannel;
use crate::context::HyperContext;
use crate::error::HyperHttpError;
use crate::message::HyperRequest;
use crate::response::response_templates;

pub(super) async fn dispatch<const VERSION: u8>(
    request: Request<Incoming>,
    channel: HyperChannel<VERSION>,
    runtime: Arc<RuntimeConfig>,
    root: Arc<UrlRoot<HyperContext<VERSION>, TcpTransport>>,
) -> Result<Response<http_body_util::Full<bytes::Bytes>>, Infallible> {
    let version = request.version();
    let path = request.uri().path().to_owned();
    let endpoint = root.walk_str(&path).await;

    let mut safety = channel.safety().clone();
    if let Some(route_safety) = endpoint
        .as_ref()
        .and_then(|endpoint| endpoint.get_params::<crate::safety::HyperSafety>())
    {
        safety.update(&route_safety);
    }

    let (parts, incoming) = request.into_parts();
    let body = match Limited::new(incoming, safety.max_body_size())
        .collect()
        .await
    {
        Ok(collected) => collected.to_bytes(),
        Err(error) if error.is::<http_body_util::LengthLimitError>() => {
            return Ok(
                response_templates::error_response(&HyperHttpError::BodyTooLarge, version)
                    .into_inner(),
            );
        }
        Err(_) => {
            return Ok(
                response_templates::error_response(&HyperHttpError::BadRequest, version)
                    .into_inner(),
            );
        }
    };

    let Some(endpoint) = endpoint else {
        return Ok(
            response_templates::error_response(&HyperHttpError::NoRoute(path), version)
                .into_inner(),
        );
    };

    let request = HyperRequest::new(Request::from_parts(parts, body));
    let context = HyperContext::new_server(runtime, endpoint.clone(), request, channel, safety);
    match endpoint.run(context).await {
        Ok(context) => Ok(context.into_response().into_inner()),
        Err(error) => Ok(response_templates::error_response(&error, version).into_inner()),
    }
}

pub(super) fn parse_http_path(input: &str) -> Vec<&str> {
    if input.is_empty() {
        Vec::new()
    } else {
        input.split('/').collect()
    }
}
