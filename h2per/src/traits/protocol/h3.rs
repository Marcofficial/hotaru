/// Reserved shell for a future HTTP/3 protocol in H2PER.
///
/// This type deliberately does not implement Hotaru's `Protocol` trait yet.
/// HTTP/3 requires a QUIC transport and cannot reuse the TCP channel used by
/// [`crate::H1`] and [`crate::H2`].
///
/// ```compile_fail
/// use h2per::H3;
/// use hotaru_core::protocol::Protocol;
///
/// fn require_protocol<P: Protocol>() {}
/// require_protocol::<H3>();
/// ```
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
// TODO(h2per/h3): implement QUIC transport and HTTP/3 before implementing Protocol.
pub struct HyperHttp3Protocol {
    _reserved: (),
}

impl HyperHttp3Protocol {
    /// Reserved protocol registry name.
    pub const NAME: &'static str = "h2per/h3";

    /// Work required before this shell may implement `Protocol`.
    pub const TODO: &'static str = "implement QUIC transport and HTTP/3 connection handling";
}

/// Reserved concise alias for the future HTTP/3 protocol.
pub type H3 = HyperHttp3Protocol;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserves_h3_names() {
        assert_eq!(H3::NAME, "h2per/h3");
        assert!(H3::TODO.contains("QUIC"));
    }
}
