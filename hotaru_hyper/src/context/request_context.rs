use std::sync::Arc;

use hotaru_core::app::common::RuntimeConfig;
use hotaru_core::extensions::{Locals, Params};
use hotaru_core::protocol::{EndpointOutcome, ProtocolRole, RequestContext};
use hotaru_core::url::UrlNode;
use hotaru_io_tokio::TcpTransport;

use crate::channel::HyperChannel;
use crate::error::HyperHttpError;
use crate::message::{HyperRequest, HyperResponse};
use crate::response::response_templates;
use crate::safety::HyperSafety;

/// Describes whether a context represents an inbound request or a client response.
pub enum Executable<const VERSION: u8> {
    Request {
        runtime: Arc<RuntimeConfig>,
        endpoint: Arc<UrlNode<HyperContext<VERSION>, TcpTransport>>,
    },
    Response,
}

/// Request context shared by the Hyper HTTP/1 and HTTP/2 adapters.
pub struct HyperContext<const VERSION: u8> {
    request: HyperRequest,
    response: HyperResponse,
    executable: Executable<VERSION>,
    safety: HyperSafety,
    channel: Option<HyperChannel<VERSION>>,
    role: ProtocolRole,
    pub params: Params,
    pub locals: Locals,
}

impl<const VERSION: u8> HyperContext<VERSION> {
    pub(crate) fn new_server(
        runtime: Arc<RuntimeConfig>,
        endpoint: Arc<UrlNode<Self, TcpTransport>>,
        request: HyperRequest,
        channel: HyperChannel<VERSION>,
        safety: HyperSafety,
    ) -> Self {
        Self {
            request,
            response: HyperResponse::default(),
            executable: Executable::Request { runtime, endpoint },
            safety,
            channel: Some(channel),
            role: ProtocolRole::Server,
            params: Params::default(),
            locals: Locals::default(),
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

    pub fn runtime(&self) -> Option<Arc<RuntimeConfig>> {
        match &self.executable {
            Executable::Request { runtime, .. } => Some(runtime.clone()),
            Executable::Response => None,
        }
    }

    pub fn endpoint(&self) -> Option<Arc<UrlNode<Self, TcpTransport>>> {
        match &self.executable {
            Executable::Request { endpoint, .. } => Some(endpoint.clone()),
            Executable::Response => None,
        }
    }

    pub fn safety(&self) -> &HyperSafety {
        &self.safety
    }

    pub fn remote_addr(&self) -> Option<core::net::SocketAddr> {
        self.channel.as_ref().and_then(HyperChannel::remote_addr)
    }

    pub fn local_addr(&self) -> Option<core::net::SocketAddr> {
        self.channel.as_ref().and_then(HyperChannel::local_addr)
    }

    pub fn path(&self) -> &str {
        self.request.path()
    }

    pub fn segment(&self, index: usize) -> Option<&str> {
        self.request.path_segments().nth(index)
    }

    pub fn param(&self, name: impl AsRef<str>) -> Option<String> {
        let index = self.endpoint()?.match_seg_name_with_index(name)?;
        self.request
            .path()
            .split('/')
            .nth(index)
            .map(ToOwned::to_owned)
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
            executable: Executable::Response,
            safety: HyperSafety::default(),
            channel: None,
            role: ProtocolRole::Server,
            params: Params::default(),
            locals: Locals::default(),
        }
    }
}

impl<const VERSION: u8> RequestContext for HyperContext<VERSION> {
    type Request = HyperRequest;
    type Response = HyperResponse;
    type Error = HyperHttpError;
    type Channel = HyperChannel<VERSION>;

    fn handle_error(&mut self) {
        self.response = response_templates::internal_server_error();
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

impl<const VERSION: u8> EndpointOutcome<HyperContext<VERSION>> for HyperResponse {
    fn apply_to(self, context: &mut HyperContext<VERSION>) -> Result<(), HyperHttpError> {
        context.set_response(self);
        Ok(())
    }
}

impl<const VERSION: u8> EndpointOutcome<HyperContext<VERSION>> for String {
    fn apply_to(self, context: &mut HyperContext<VERSION>) -> Result<(), HyperHttpError> {
        context.set_response(HyperResponse::text(self));
        Ok(())
    }
}

impl<const VERSION: u8> EndpointOutcome<HyperContext<VERSION>> for &'static str {
    fn apply_to(self, context: &mut HyperContext<VERSION>) -> Result<(), HyperHttpError> {
        context.set_response(HyperResponse::text(self));
        Ok(())
    }
}

pub type HyperH1Context = HyperContext<1>;
pub type HyperH2Context = HyperContext<2>;
