use core::fmt;

use hotaru_core::protocol::ProtocolError;

/// Errors produced by the H2PER prototype.
#[derive(Debug)]
pub enum H2perError {
    /// The underlying Hotaru transport failed.
    Io(std::io::Error),
    /// Hyper rejected or terminated the HTTP connection.
    Hyper(hyper::Error),
    /// The connection I/O was already handed to Hyper.
    ChannelAlreadyTaken,
    /// The prototype currently implements inbound server traffic only.
    Unsupported(&'static str),
}

impl fmt::Display for H2perError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "transport I/O error: {error}"),
            Self::Hyper(error) => write!(formatter, "hyper connection error: {error}"),
            Self::ChannelAlreadyTaken => formatter.write_str("channel I/O was already taken"),
            Self::Unsupported(operation) => {
                write!(formatter, "unsupported prototype operation: {operation}")
            }
        }
    }
}

impl std::error::Error for H2perError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Hyper(error) => Some(error),
            Self::ChannelAlreadyTaken | Self::Unsupported(_) => None,
        }
    }
}

/// Hotaru `ProtocolError` contract for H2PER failures.
impl ProtocolError for H2perError {}

impl From<std::io::Error> for H2perError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<hyper::Error> for H2perError {
    fn from(error: hyper::Error) -> Self {
        Self::Hyper(error)
    }
}
