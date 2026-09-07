mod dispatch;
mod h1;
mod h2;
mod h3;

pub use h1::{H1, HyperHttp1Protocol};
pub use h2::{H2, HyperHttp2Protocol};
pub use h3::{H3, HyperHttp3Protocol};

#[cfg(test)]
mod tests {
    use hotaru_core::protocol::Protocol;

    use super::{H1, H2};

    #[test]
    fn detects_expected_wire_versions() {
        assert!(H1::detect(b"GET / HTTP/1.1\r\n"));
        assert!(!H1::detect(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n"));
        assert!(H2::detect(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n"));
        assert!(!H2::detect(b"GET / HTTP/1.1\r\n"));
    }

    #[test]
    fn names_are_distinct_from_native_http() {
        assert_eq!(H1::server().name(), "h2per/h1");
        assert_eq!(H2::server().name(), "h2per/h2");
    }
}
