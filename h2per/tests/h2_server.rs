use h2per::{H2, H2Context, HyperResponse};
use hotaru::prelude::*;
use http_body_util::{BodyExt, Empty};
use hyper::Request;
use hyper_util::rt::{TokioExecutor, TokioIo};

async fn index_body(_context: &mut H2Context) -> HyperResponse {
    HyperResponse::text("Hello over H2")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn hotaru_serves_a_real_h2_request() {
    let probe = std::net::TcpListener::bind("127.0.0.1:0").expect("reserve test port");
    let address = probe.local_addr().expect("read test address");
    drop(probe);

    let app = Server::<TcpTransport, TokioRuntime>::new()
        .binding(address.to_string())
        .single_protocol(ProtocolBuilder::new(H2::server()))
        .build();

    let endpoint = Endpoint::<H2>::endpoint("/", "index", |context: &mut H2Context| {
        Box::pin(index_body(context))
    });
    app.insert(endpoint).expect("register H2 endpoint");

    // Bind before spawning so the client never races server startup.
    app.ensure_inbound().await.expect("bind H2 server");
    let (stop_tx, stop_rx) = tokio::sync::oneshot::channel::<()>();
    let server = tokio::spawn(app.clone().run_until(async move {
        let _ = stop_rx.await;
    }));

    let stream = tokio::net::TcpStream::connect(address)
        .await
        .expect("connect H2 client");
    let io = TokioIo::new(stream);
    let (mut sender, connection) = hyper::client::conn::http2::Builder::new(TokioExecutor::new())
        .handshake(io)
        .await
        .expect("complete H2 handshake");
    let connection_task = tokio::spawn(connection);

    let request = Request::builder()
        .uri(format!("http://{address}/"))
        .body(Empty::<bytes::Bytes>::new())
        .expect("build request");
    let response = sender.send_request(request).await.expect("send H2 request");

    assert_eq!(response.status(), hyper::StatusCode::OK);
    assert_eq!(response.version(), hyper::Version::HTTP_2);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("collect response")
        .to_bytes();
    assert_eq!(body, "Hello over H2");

    drop(sender);
    let _ = stop_tx.send(());
    server.await.expect("join server");
    connection_task.abort();
}
