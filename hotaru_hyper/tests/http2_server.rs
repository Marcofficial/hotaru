use hotaru_core::{
    app::server::Server,
    executable::{ProtocolEntryBuilder, def::Endpoint},
};
use hotaru_hyper::{HyperH2, HyperH2Context, HyperHttpConfig, HyperResponse};
use hotaru_io_tokio::{TcpStream as HotaruTcpStream, TcpTransport};
use hotaru_rt_tokio::TokioRuntime;
use http_body_util::{BodyExt, Empty};
use hyper::Request;
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::net::TcpListener;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn serves_a_hotaru_endpoint_over_real_http2() {
    let app = Server::<TcpTransport, TokioRuntime>::new()
        .binding("127.0.0.1:0")
        .single_protocol(ProtocolEntryBuilder::new(HyperH2::server(
            HyperHttpConfig::default(),
        )))
        .build();

    app.insert(Endpoint::<HyperH2>::endpoint(
        "/",
        "index",
        |_context: &mut HyperH2Context| {
            Box::pin(async { HyperResponse::text("Hello over HTTP/2") })
        },
    ))
    .expect("register HTTP/2 endpoint");

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test listener");
    let address = listener.local_addr().expect("read listener address");
    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.expect("accept test connection");
        app.handle_wire(HotaruTcpStream::new(stream));
    });

    let stream = tokio::net::TcpStream::connect(address)
        .await
        .expect("connect HTTP/2 client");
    let io = TokioIo::new(stream);
    let (mut sender, connection) = hyper::client::conn::http2::Builder::new(TokioExecutor::new())
        .handshake(io)
        .await
        .expect("complete HTTP/2 handshake");
    let connection_task = tokio::spawn(connection);

    let request = Request::builder()
        .uri(format!("http://{address}/"))
        .body(Empty::<bytes::Bytes>::new())
        .expect("build HTTP/2 request");
    let response = sender
        .send_request(request)
        .await
        .expect("send HTTP/2 request");

    assert_eq!(response.status(), hyper::StatusCode::OK);
    assert_eq!(response.version(), hyper::Version::HTTP_2);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("collect HTTP/2 response")
        .to_bytes();
    assert_eq!(body, "Hello over HTTP/2");

    drop(sender);
    connection_task.abort();
}
