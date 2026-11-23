# hotaru_hyper prototype

`hotaru_hyper` is an independent HTTP protocol package for Hotaru applications
running on std and Tokio. Hyper owns HTTP parsing and connection state; Hotaru
continues to own protocol registration, URL routing, middleware, and endpoint
execution.

The crate does not depend on `hotaru_http` and does not replace it.

## Implemented scope

- Hyper HTTP/1 server over Tokio TCP.
- Cleartext HTTP/2 prior-knowledge server over Tokio TCP.
- Hotaru `Protocol`, `Channel`, `RequestContext`, and `EndpointOutcome` adapters.
- Buffered request bodies with a configurable hard size limit.
- Protocol-level safety baselines with per-endpoint body-limit overrides.
- Hotaru `UrlRoot` endpoint dispatch and framework-level 404/500 responses.
- Runtime, matched endpoint, route parameters, locals, and connection addresses
  exposed through `HyperContext`.
- Preservation of bytes buffered during Hotaru protocol detection.

The package follows the same processing layers as `hotaru_http` (`channel`,
`context`, `message`, `protocol`, `response`, `safety`, and `error`), but it
does not copy the native parser. Hyper remains responsible for HTTP syntax,
framing, and connection state.

## Prototype limits

- Server-side only; client channel acquisition and sending are unsupported.
- Tokio TCP only; TLS and ALPN are not connected.
- Request and response streaming are not implemented.
- Native `hotaru_http` messages, contexts, and `htmstd` middleware are not reused.
- Native Hotaru HTTP/1 and Hyper HTTP/1 must not compete on the same listener.
- Hyper HTTP/1 and HTTP/2 should use separate listeners until Hotaru protocol
  detection can represent an incomplete, ambiguous wire prefix.
- HTTP/3 is not represented until Hotaru has a QUIC transport integration.

Run the HTTP/1 example:

```console
cargo run -p hotaru_hyper --example http1_server
curl http://127.0.0.1:38081/
```

Run the focused checks:

```console
cargo check -p hotaru_hyper
cargo test -p hotaru_hyper
cargo clippy -p hotaru_hyper --all-targets --no-deps -- -D warnings
```
