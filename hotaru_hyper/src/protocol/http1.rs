use core::time::Duration;
use std::sync::Arc;

use hotaru_core::app::common::RuntimeConfig;
use hotaru_core::connection::{ConnStream, HotaruRead, HotaruWrite, TransportSpec};
use hotaru_core::protocol::{Channel, CtxError, Protocol, ProtocolFlow, ProtocolRole};
use hotaru_core::url::UrlRoot;
use hotaru_io_tokio::{TcpStream as HotaruTcpStream, TcpTransport};
use hyper::service::service_fn;
use hyper_util::rt::{TokioIo as HyperTokioIo, TokioTimer};

use crate::channel::HyperChannel;
use crate::context::HyperContext;
use crate::error::HyperHttpError;
use crate::protocol::dispatch::{dispatch, parse_http_path};
use crate::safety::{HyperHttpConfig, HyperSafety};

const HTTP1_METHOD_PREFIXES: &[&[u8]] = &[
    b"GET ",
    b"POST ",
    b"PUT ",
    b"DELETE ",
    b"HEAD ",
    b"OPTIONS ",
    b"PATCH ",
    b"CONNECT ",
    b"TRACE ",
];

/// Hyper-backed HTTP/1 server protocol for Tokio TCP.
#[derive(Debug, Clone)]
pub struct HyperHttp1Protocol {
    role: ProtocolRole,
    safety: Arc<HyperSafety>,
}

impl HyperHttp1Protocol {
    pub fn server(config: HyperHttpConfig) -> Self {
        Self {
            role: ProtocolRole::Server,
            safety: Arc::new(config),
        }
    }

    pub fn config(&self) -> &HyperHttpConfig {
        &self.safety
    }

    pub fn safety(&self) -> &HyperSafety {
        &self.safety
    }
}

impl Default for HyperHttp1Protocol {
    fn default() -> Self {
        Self::server(HyperHttpConfig::default())
    }
}

pub type HyperH1 = HyperHttp1Protocol;

impl Protocol for HyperHttp1Protocol {
    type Wire = HotaruTcpStream;
    type TS = TcpTransport;
    type Channel = HyperChannel<1>;
    type Stream = ();
    type Message = ();
    type Context = HyperContext<1>;

    fn name(&self) -> &'static str {
        "hotaru-hyper/http1"
    }

    fn role(&self) -> ProtocolRole {
        self.role
    }

    fn default_connection_timeout(&self) -> Option<Duration> {
        None
    }

    fn lit_parser(input: &str) -> Vec<&str> {
        parse_http_path(input)
    }

    fn detect(initial_bytes: &[u8]) -> bool {
        !initial_bytes.is_empty()
            && HTTP1_METHOD_PREFIXES.iter().any(|prefix| {
                prefix.starts_with(initial_bytes) || initial_bytes.starts_with(prefix)
            })
    }

    fn open_channel(
        self,
        reader: <<<Self::TS as TransportSpec>::Wire as ConnStream>::ReadHalf as HotaruRead>::Buffered,
        writer: <<<Self::TS as TransportSpec>::Wire as ConnStream>::WriteHalf as HotaruWrite>::Buffered,
        meta: <<Self::TS as TransportSpec>::Wire as ConnStream>::Meta,
    ) -> Self::Channel {
        HyperChannel::new(reader.into_inner(), writer.into_inner(), meta, self.safety)
    }

    async fn handle(
        channel: &Self::Channel,
        runtime: Arc<RuntimeConfig>,
        root: Arc<UrlRoot<Self::Context, Self::TS>>,
    ) -> Result<ProtocolFlow, CtxError<Self>> {
        let io = HyperTokioIo::new(channel.take_io()?);
        let service_channel = channel.clone();
        let service = service_fn(move |request| {
            dispatch(
                request,
                service_channel.clone(),
                runtime.clone(),
                root.clone(),
            )
        });

        let mut builder = hyper::server::conn::http1::Builder::new();
        builder
            .keep_alive(channel.safety().keep_alive())
            .max_headers(channel.safety().max_headers())
            .max_header_size(channel.safety().max_header_size())
            .header_read_timeout(channel.safety().header_read_timeout())
            .timer(TokioTimer::new());
        let result = builder.serve_connection(io, service).await;
        channel.close();
        result?;
        Ok(ProtocolFlow::Close)
    }

    async fn acquire_channel(
        &self,
        _runtime: &Arc<RuntimeConfig>,
        _outbound: Arc<<Self::TS as TransportSpec>::Outbound>,
    ) -> Result<Self::Channel, CtxError<Self>> {
        Err(HyperHttpError::Unsupported("HTTP/1 client channels"))
    }

    async fn send(ctx: Self::Context) -> Result<Self::Context, CtxError<Self>> {
        let _ = ctx;
        Err(HyperHttpError::Unsupported("HTTP/1 client send"))
    }

    fn install_channel(ctx: &mut Self::Context, channel: Self::Channel) {
        ctx.install_channel(channel);
    }
}
