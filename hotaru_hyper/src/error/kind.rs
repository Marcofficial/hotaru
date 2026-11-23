use core::fmt;

use hotaru_core::protocol::ProtocolError;
use hyper::StatusCode;

/// Errors produced by the Hyper-backed protocols.
#[derive(Debug)]
#[non_exhaustive]
pub enum HyperHttpError {
    Io(std::io::Error),
    Hyper(hyper::Error),
    BadRequest,
    BodyTooLarge,
    NoRoute(String),
    Status(StatusCode),
    ChannelAlreadyTaken,
    ChannelPoisoned,
    Unsupported(&'static str),
}

impl HyperHttpError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::BodyTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Self::NoRoute(_) => StatusCode::NOT_FOUND,
            Self::Status(status) => *status,
            Self::Io(_)
            | Self::Hyper(_)
            | Self::ChannelAlreadyTaken
            | Self::ChannelPoisoned
            | Self::Unsupported(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl fmt::Display for HyperHttpError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "transport I/O error: {error}"),
            Self::Hyper(error) => write!(formatter, "Hyper connection error: {error}"),
            Self::BadRequest => formatter.write_str("failed to read request body"),
            Self::BodyTooLarge => formatter.write_str("request body too large"),
            Self::NoRoute(path) => write!(formatter, "no route matched path: {path}"),
            Self::Status(status) => write!(formatter, "HTTP status error: {status}"),
            Self::ChannelAlreadyTaken => formatter.write_str("channel I/O was already taken"),
            Self::ChannelPoisoned => formatter.write_str("channel I/O lock was poisoned"),
            Self::Unsupported(operation) => write!(formatter, "unsupported operation: {operation}"),
        }
    }
}

impl std::error::Error for HyperHttpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Hyper(error) => Some(error),
            _ => None,
        }
    }
}

impl ProtocolError for HyperHttpError {
    fn can_continue(&self) -> bool {
        matches!(self, Self::NoRoute(_) | Self::Status(_))
    }
}

impl From<std::io::Error> for HyperHttpError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<hyper::Error> for HyperHttpError {
    fn from(error: hyper::Error) -> Self {
        Self::Hyper(error)
    }
}

impl From<StatusCode> for HyperHttpError {
    fn from(status: StatusCode) -> Self {
        Self::Status(status)
    }
}
