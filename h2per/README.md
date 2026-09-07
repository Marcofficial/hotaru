# H2PER prototype

This workspace crate proves that Hyper can serve Hotaru endpoint definitions
through the current `Protocol`, `Channel`, `RequestContext`, and `UrlRoot`
contracts without modifying `hotaru_core`.

Public protocol names:

- `H1` / `HyperHttp1Protocol`: HTTP/1 server
- `H2` / `HyperHttp2Protocol`: cleartext HTTP/2 prior-knowledge server
- `H3` / `HyperHttp3Protocol`: reserved API shell; not a working protocol

H1 and H2 use Tokio TCP. Client/outpoint support, TLS/ALPN, upgrades, streaming
request bodies, and a working HTTP/3 implementation are intentionally outside
this first prototype.

## Suggested source reading order

The source tree mirrors the Hotaru contracts implemented by H2PER:

1. `src/traits/protocol_error.rs` — `ProtocolError` and error conversion.
2. `src/traits/channel.rs` — `Channel` and ownership of the detected TCP I/O.
3. `src/message.rs` — request and response values carried by the context.
4. `src/traits/request_context.rs` — `RequestContext` state seen by handlers.
5. `src/traits/endpoint_outcome.rs` — accepted endpoint return types.
6. `src/traits/protocol/dispatch.rs` — shared Hyper-to-Hotaru request dispatch.
7. `src/traits/protocol/h1.rs` — the HTTP/1 `Protocol` implementation.
8. `src/traits/protocol/h2.rs` — the HTTP/2 `Protocol` implementation.
9. `src/traits/protocol/h3.rs` — reserved H3 shell and TODO only.

`src/io.rs` is the Tokio adapter that recombines Hotaru's buffered read/write
halves into the duplex I/O shape expected by Hyper.

Run the HTTP/1 example:

```console
cargo run --manifest-path h2per/example/Cargo.toml
curl http://127.0.0.1:38081/
```

Run all focused checks (the H2 test binds a loopback port):

```console
cargo check -p h2per
cargo check -p h2per-example
cargo test -p h2per
cargo clippy -p h2per --all-targets --no-deps -- -D warnings
```

## Verified scope

- H1 serves Hotaru `endpoint!` routes through a real Hyper HTTP/1 connection.
- H1 exposes the request version to the endpoint and returns a framework-level 404.
- H2 completes a real prior-knowledge handshake, executes a Hotaru endpoint,
  and returns an HTTP/2 response.
- Hotaru's protocol-detection buffer is preserved when I/O ownership moves to Hyper.

## Prototype limits

- Server-side only; `acquire_channel` and `send` return an explicit
  `H2perError::Unsupported`.
- Tokio TCP only; TLS/ALPN has not been connected.
- Request bodies are collected in memory before endpoint execution.
- Native `htmstd` middleware and native `HttpContext` helpers are not reused.
- Pattern routes can match, but the prototype does not yet expose captured
  route parameters on `HyperContext`.
- H1 and native Hotaru `HTTP` recognize the same wire prefix and must not be
  registered as competing handlers on one listener.
- H3 only reserves the public Rust and registry names. It deliberately does not
  implement Hotaru's `Protocol` trait until a QUIC transport and real HTTP/3
  connection handling exist.
