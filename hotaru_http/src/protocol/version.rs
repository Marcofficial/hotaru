//! Version capability policy for the HTTP/1 protocol implementation.

use crate::message::http_value::HttpVersion;

use super::error::HttpError;

/// Require a version whose wire format is implemented by `Http1Protocol`.
///
/// `HttpVersion` also represents versions used by other protocol
/// implementations. Recognising one of those versions while parsing must not
/// allow it to enter the HTTP/1 router.
pub(super) fn ensure_http1_version(version: &HttpVersion) -> Result<(), HttpError> {
    match version {
        HttpVersion::Http10 | HttpVersion::Http11 => Ok(()),
        other => Err(HttpError::VersionNotSupported(other.clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_10_and_11_are_supported() {
        assert!(ensure_http1_version(&HttpVersion::Http10).is_ok());
        assert!(ensure_http1_version(&HttpVersion::Http11).is_ok());
    }

    #[test]
    fn non_http1_versions_are_rejected() {
        for version in [
            HttpVersion::Http09,
            HttpVersion::Http20,
            HttpVersion::Http30,
            HttpVersion::Unknown,
        ] {
            assert!(matches!(
                ensure_http1_version(&version),
                Err(HttpError::VersionNotSupported(_))
            ));
        }
    }
}
