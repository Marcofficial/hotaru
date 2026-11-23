use hotaru_core::{
    app::server::Server,
    executable::{ProtocolEntryBuilder, def::Endpoint},
};
use hotaru_hyper::{HyperH1, HyperH1Context, HyperHttpConfig, text_response};
use hotaru_io_tokio::TcpTransport;
use hotaru_rt_tokio::TokioRuntime;

#[tokio::main]
async fn main() {
    let app = Server::<TcpTransport, TokioRuntime>::new()
        .binding("127.0.0.1:38081")
        .single_protocol(ProtocolEntryBuilder::new(HyperH1::server(
            HyperHttpConfig::default(),
        )))
        .build();

    app.insert(Endpoint::<HyperH1>::endpoint(
        "/",
        "index",
        |_context: &mut HyperH1Context| {
            Box::pin(async { text_response("Hello from Hotaru + Hyper") })
        },
    ))
    .expect("register index endpoint");

    app.run().await;
}
