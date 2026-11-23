use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use hotaru_core::connection::ConnMeta;
use hotaru_core::protocol::Channel;
use hotaru_io_tokio::TcpMeta;

use crate::error::HyperHttpError;
use crate::io::{HyperDuplexIo, TcpReader, TcpWriter};
use crate::safety::HyperSafety;

/// Cheap, cloneable handle to one Hyper-owned TCP connection.
pub struct HyperChannel<const VERSION: u8> {
    io: Arc<Mutex<Option<HyperDuplexIo>>>,
    open: Arc<AtomicBool>,
    safety: Arc<HyperSafety>,
    local_addr: Option<core::net::SocketAddr>,
    remote_addr: Option<core::net::SocketAddr>,
}

impl<const VERSION: u8> HyperChannel<VERSION> {
    pub(crate) fn new(
        reader: TcpReader,
        writer: TcpWriter,
        meta: TcpMeta,
        safety: Arc<HyperSafety>,
    ) -> Self {
        Self {
            io: Arc::new(Mutex::new(Some(HyperDuplexIo::new(reader, writer)))),
            open: Arc::new(AtomicBool::new(true)),
            safety,
            local_addr: meta.local_addr(),
            remote_addr: meta.remote_addr(),
        }
    }

    pub(crate) fn take_io(&self) -> Result<HyperDuplexIo, HyperHttpError> {
        self.io
            .lock()
            .map_err(|_| HyperHttpError::ChannelPoisoned)?
            .take()
            .ok_or(HyperHttpError::ChannelAlreadyTaken)
    }

    pub fn local_addr(&self) -> Option<core::net::SocketAddr> {
        self.local_addr
    }

    pub fn remote_addr(&self) -> Option<core::net::SocketAddr> {
        self.remote_addr
    }

    pub fn safety(&self) -> &HyperSafety {
        &self.safety
    }

    /// Backward-compatible name for the connection safety configuration.
    pub fn config(&self) -> &HyperSafety {
        self.safety()
    }
}

impl<const VERSION: u8> Clone for HyperChannel<VERSION> {
    fn clone(&self) -> Self {
        Self {
            io: self.io.clone(),
            open: self.open.clone(),
            safety: self.safety.clone(),
            local_addr: self.local_addr,
            remote_addr: self.remote_addr,
        }
    }
}

impl<const VERSION: u8> Channel for HyperChannel<VERSION> {
    fn is_open(&self) -> bool {
        self.open.load(Ordering::Acquire)
    }

    fn close(&self) {
        self.open.store(false, Ordering::Release);
    }
}
