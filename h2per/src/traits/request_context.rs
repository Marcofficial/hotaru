use hotaru_core::protocol::{ProtocolRole, RequestContext};
use hyper::StatusCode;

use crate::message::{HyperRequest, HyperResponse};
use crate::traits::channel::HyperChannel;
use crate::traits::protocol_error::H2perError;

/// Request context used by both H1 and H2 handlers.
pub struct HyperContext<const VERSION: u8> {
    request: HyperRequest,
    response: HyperResponse,
    channel: Option<HyperChannel<VERSION>>,
    role: ProtocolRole,
}

impl<const VERSION: u8> HyperContext<VERSION> {
    pub(crate) fn server(request: HyperRequest, channel: HyperChannel<VERSION>) -> Self {
        Self {
            request,
            response: HyperResponse::default(),
            channel: Some(channel),
            role: ProtocolRole::Server,
        }
    }

    pub fn request(&self) -> &HyperRequest {
        &self.request
    }

    pub fn request_mut(&mut self) -> &mut HyperRequest {
        &mut self.request
    }

    pub fn response(&self) -> &HyperResponse {
        &self.response
    }

    pub fn response_mut(&mut self) -> &mut HyperResponse {
        &mut self.response
    }

    pub fn set_response(&mut self, response: HyperResponse) {
        self.response = response;
    }

    pub(crate) fn install_channel(&mut self, channel: HyperChannel<VERSION>) {
        self.channel = Some(channel);
    }
}

impl<const VERSION: u8> Default for HyperContext<VERSION> {
    fn default() -> Self {
        Self {
            request: HyperRequest::default(),
            response: HyperResponse::default(),
            channel: None,
            role: ProtocolRole::Server,
        }
    }
}

/// Hotaru `RequestContext` contract for a Hyper request/response exchange.
impl<const VERSION: u8> RequestContext for HyperContext<VERSION> {
    type Request = HyperRequest;
    type Response = HyperResponse;
    type Error = H2perError;
    type Channel = HyperChannel<VERSION>;

    fn handle_error(&mut self) {
        self.response =
            HyperResponse::text("route has no handler").status(StatusCode::INTERNAL_SERVER_ERROR);
    }

    fn role(&self) -> ProtocolRole {
        self.role
    }

    fn inject_request(&mut self, request: Self::Request) {
        self.request = request;
    }

    fn into_response(self) -> Self::Response {
        self.response
    }
}

pub type H1Context = HyperContext<1>;
pub type H2Context = HyperContext<2>;
