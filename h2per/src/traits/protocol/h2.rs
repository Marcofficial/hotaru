use std::sync::Arc;

use hotaru_core::app::common::RuntimeConfig;
use hotaru_core::connection::{ConnStream, HotaruRead, HotaruWrite, TransportSpec};
use hotaru_core::protocol::{Channel, CtxError, Protocol, ProtocolFlow, ProtocolRole};
use hotaru_core::url::UrlRoot;
use hotaru_io_tokio::{TcpStream as HotaruTcpStream, TcpTransport};
use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo as HyperTokioIo};

use crate::traits::channel::HyperChannel;
use crate::traits::protocol::dispatch::{dispatch, parse_http_path};
use crate::traits::protocol_error::H2perError;
use crate::traits::request_context::HyperContext;

/// Hyper-backed HTTP/2 prior-knowledge protocol.
#[derive(Debug, Clone, Copy)]
pub struct HyperHttp2Protocol {
    role: ProtocolRole,
}

impl Default for HyperHttp2Protocol {
    fn default() -> Self {
        Self::server()
    }
}

impl HyperHttp2Protocol {
    pub fn server() -> Self {
        Self {
            role: ProtocolRole::Server,
        }
    }
}

/// Concise Hotaru DSL alias for HTTP/2.
pub type H2 = HyperHttp2Protocol;

/// Hotaru `Protocol` contract for Hyper HTTP/2.
impl Protocol for HyperHttp2Protocol {
    type Wire = HotaruTcpStream;
    type TS = TcpTransport;
    type Channel = HyperChannel<2>;
    type Stream = ();
    type Message = ();
    type Context = HyperContext<2>;

    fn name(&self) -> &'static str {
        "h2per/h2"
    }

    fn role(&self) -> ProtocolRole {
        self.role
    }

    fn lit_parser(input: &str) -> Vec<&str> {
        parse_http_path(input)
    }

    fn detect(initial_bytes: &[u8]) -> bool {
        initial_bytes.starts_with(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n")
    }

    fn open_channel(
        self,
        reader: <<<Self::TS as TransportSpec>::Wire as ConnStream>::ReadHalf as HotaruRead>::Buffered,
        writer: <<<Self::TS as TransportSpec>::Wire as ConnStream>::WriteHalf as HotaruWrite>::Buffered,
        meta: <<Self::TS as TransportSpec>::Wire as ConnStream>::Meta,
    ) -> Self::Channel {
        HyperChannel::new(reader.into_inner(), writer.into_inner(), meta)
    }

    async fn handle(
        channel: &Self::Channel,
        _runtime: Arc<RuntimeConfig>,
        root: Arc<UrlRoot<Self::Context, Self::TS>>,
    ) -> Result<ProtocolFlow, CtxError<Self>> {
        let io = HyperTokioIo::new(channel.take_io()?);
        let service_channel = channel.clone();
        let service =
            service_fn(move |request| dispatch(request, service_channel.clone(), root.clone()));

        hyper::server::conn::http2::Builder::new(TokioExecutor::new())
            .serve_connection(io, service)
            .await?;
        channel.close();
        Ok(ProtocolFlow::Close)
    }

    async fn acquire_channel(
        &self,
        _runtime: &Arc<RuntimeConfig>,
        _outbound: Arc<<Self::TS as TransportSpec>::Outbound>,
    ) -> Result<Self::Channel, CtxError<Self>> {
        Err(H2perError::Unsupported("H2 client channels"))
    }

    async fn send(ctx: Self::Context) -> Result<Self::Context, CtxError<Self>> {
        let _ = ctx;
        Err(H2perError::Unsupported("H2 client send"))
    }

    fn install_channel(ctx: &mut Self::Context, channel: Self::Channel) {
        ctx.install_channel(channel);
    }
}
