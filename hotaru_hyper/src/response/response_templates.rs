use hotaru_core::protocol::ProtocolError;
use hyper::header::{CONNECTION, HeaderValue};
use hyper::{StatusCode, Version};

use crate::error::HyperHttpError;
use crate::message::HyperResponse;

pub fn error_response(error: &HyperHttpError, version: Version) -> HyperResponse {
    let status = error.status_code();
    let code = status.as_u16();
    let reason = status.canonical_reason().unwrap_or("Unknown");
    let body = format!(
        "<!DOCTYPE html>\n<html>\n<head><title>{code} {reason}</title></head>\n\
         <body><h1>{code} {reason}</h1></body>\n</html>\n"
    );
    let mut response = HyperResponse::html(body).status(status);

    if !error.can_continue() && matches!(version, Version::HTTP_10 | Version::HTTP_11) {
        response
            .headers_mut()
            .insert(CONNECTION, HeaderValue::from_static("close"));
    }
    response
}

pub fn internal_server_error() -> HyperResponse {
    HyperResponse::text("internal server error").status(StatusCode::INTERNAL_SERVER_ERROR)
}
