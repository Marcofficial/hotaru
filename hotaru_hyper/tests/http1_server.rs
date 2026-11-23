use core::sync::atomic::{AtomicUsize, Ordering};
use core::time::Duration;

use hotaru_core::{
    app::server::Server,
    executable::{ProtocolEntryBuilder, def::Endpoint},
    extensions::ParamsClone,
    prelude::Arc,
};
use hotaru_hyper::{
    HyperH1, HyperH1Context, HyperHttpConfig, HyperResponse, HyperSafety, text_response,
};
use hotaru_io_tokio::{TcpStream as HotaruTcpStream, TcpTransport};
use hotaru_rt_tokio::TokioRuntime;
use hyper::StatusCode;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;

#[tokio::test]
async fn serves_a_hotaru_endpoint_over_hyper_http1() {
    let app = Server::<TcpTransport, TokioRuntime>::new()
        .binding("127.0.0.1:0")
        .single_protocol(ProtocolEntryBuilder::new(HyperH1::server(
            HyperHttpConfig::default(),
        )))
        .build();

    app.insert(Endpoint::<HyperH1>::endpoint(
        "/",
        "index",
        |context: &mut HyperH1Context| {
            let version = context.request().version();
            Box::pin(async move { text_response(format!("Hello over {version:?}")) })
        },
    ))
    .expect("register HTTP/1 endpoint");

    let address = serve_once(app).await;
    let response = exchange(
        address,
        b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
    assert!(response.ends_with("Hello over HTTP/1.1"));
}

#[tokio::test]
async fn rejects_an_oversized_body_before_endpoint_execution() {
    let route_hits = Arc::new(AtomicUsize::new(0));
    let handler_hits = route_hits.clone();
    let app = Server::<TcpTransport, TokioRuntime>::new()
        .binding("127.0.0.1:0")
        .single_protocol(ProtocolEntryBuilder::new(HyperH1::server(
            HyperHttpConfig::default().with_max_body_size(8),
        )))
        .build();

    let mut route_config = ParamsClone::default();
    route_config.set(HyperSafety::new().with_max_body_size(4));
    app.insert(
        Endpoint::<HyperH1>::endpoint("/upload", "upload", move |_context: &mut HyperH1Context| {
            let handler_hits = handler_hits.clone();
            Box::pin(async move {
                handler_hits.fetch_add(1, Ordering::SeqCst);
                HyperResponse::text("accepted").status(StatusCode::OK)
            })
        })
        .with_config(route_config),
    )
    .expect("register upload endpoint");

    let address = serve_once(app).await;
    let response = exchange(
        address,
        b"POST /upload HTTP/1.1\r\nHost: localhost\r\nContent-Length: 5\r\nConnection: close\r\n\r\n12345",
    )
    .await;

    assert!(response.starts_with("HTTP/1.1 413 Payload Too Large\r\n"));
    assert_eq!(route_hits.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn exposes_runtime_connection_and_route_parameters() {
    let app = Server::<TcpTransport, TokioRuntime>::new()
        .binding("127.0.0.1:0")
        .single_protocol(ProtocolEntryBuilder::new(HyperH1::default()))
        .build();

    app.insert(Endpoint::<HyperH1>::endpoint(
        "/users/<id>",
        "user",
        |context: &mut HyperH1Context| {
            let id = context.param("id").expect("matched id parameter");
            assert!(context.runtime().is_some());
            assert!(context.remote_addr().is_some());
            assert!(context.local_addr().is_some());
            Box::pin(async move { id })
        },
    ))
    .expect("register parameterized endpoint");

    let address = serve_once(app).await;
    let response = exchange(
        address,
        b"GET /users/42 HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
    assert!(response.ends_with("42"));
}

async fn serve_once(app: Arc<Server<TcpTransport, TokioRuntime>>) -> core::net::SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test listener");
    let address = listener.local_addr().expect("read listener address");
    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.expect("accept test connection");
        app.handle_wire(HotaruTcpStream::new(stream));
    });
    address
}

async fn exchange(address: core::net::SocketAddr, request: &[u8]) -> String {
    let mut stream = TcpStream::connect(address)
        .await
        .expect("connect test client");
    stream.write_all(request).await.expect("write request");

    let mut response = Vec::new();
    timeout(Duration::from_secs(2), stream.read_to_end(&mut response))
        .await
        .expect("server should close connection")
        .expect("read response");
    String::from_utf8(response).expect("HTTP/1 response should be UTF-8")
}
