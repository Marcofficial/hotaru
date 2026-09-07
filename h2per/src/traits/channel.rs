use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use hotaru_core::connection::ConnMeta;
use hotaru_core::protocol::Channel;
use hotaru_io_tokio::TcpMeta;

use crate::io::{DuplexIo, TcpReader, TcpWriter};
use crate::traits::protocol_error::H2perError;

/// Cheap, cloneable owner of one Hyper connection.
pub struct HyperChannel<const VERSION: u8> {
    io: Arc<Mutex<Option<DuplexIo>>>,
    open: Arc<AtomicBool>,
    local_addr: Option<std::net::SocketAddr>,
    remote_addr: Option<std::net::SocketAddr>,
}

impl<const VERSION: u8> HyperChannel<VERSION> {
    pub(crate) fn new(reader: TcpReader, writer: TcpWriter, meta: TcpMeta) -> Self {
        Self {
            io: Arc::new(Mutex::new(Some(DuplexIo::new(reader, writer)))),
            open: Arc::new(AtomicBool::new(true)),
            local_addr: meta.local_addr(),
            remote_addr: meta.remote_addr(),
        }
    }

    pub(crate) fn take_io(&self) -> Result<DuplexIo, H2perError> {
        self.io
            .lock()
            .map_err(|_| H2perError::ChannelAlreadyTaken)?
            .take()
            .ok_or(H2perError::ChannelAlreadyTaken)
    }

    pub fn local_addr(&self) -> Option<std::net::SocketAddr> {
        self.local_addr
    }

    pub fn remote_addr(&self) -> Option<std::net::SocketAddr> {
        self.remote_addr
    }
}

impl<const VERSION: u8> Clone for HyperChannel<VERSION> {
    fn clone(&self) -> Self {
        Self {
            io: self.io.clone(),
            open: self.open.clone(),
            local_addr: self.local_addr,
            remote_addr: self.remote_addr,
        }
    }
}

/// Hotaru `Channel` contract for the Hyper-owned connection.
impl<const VERSION: u8> Channel for HyperChannel<VERSION> {
    fn is_open(&self) -> bool {
        self.open.load(Ordering::Acquire)
    }

    fn close(&self) {
        self.open.store(false, Ordering::Release);
    }
}
