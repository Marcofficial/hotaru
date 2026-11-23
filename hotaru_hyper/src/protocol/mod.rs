mod dispatch;
mod http1;
mod http2;

pub use http1::{HyperH1, HyperHttp1Protocol};
pub use http2::{HyperH2, HyperHttp2Protocol};

#[cfg(test)]
mod tests {
    use hotaru_core::protocol::Protocol;

    use super::{HyperH1, HyperH2};

    #[test]
    fn detects_fragmented_http1_method_prefixes() {
        let request = b"GET / HTTP/1.1\r\n";
        for length in 1..=request.len() {
            assert!(HyperH1::detect(&request[..length]));
        }
    }

    #[test]
    fn detects_fragmented_http2_preface() {
        let preface = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
        for length in 1..=preface.len() {
            assert!(HyperH2::detect(&preface[..length]));
        }
    }

    #[test]
    fn protocol_names_are_distinct() {
        assert_eq!(
            HyperH1::server(Default::default()).name(),
            "hotaru-hyper/http1"
        );
        assert_eq!(
            HyperH2::server(Default::default()).name(),
            "hotaru-hyper/http2"
        );
    }
}
