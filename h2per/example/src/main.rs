use h2per::{H1, text_response};
use hotaru::prelude::*;

LServer!(
    APP = Server::new()
        .binding("127.0.0.1:38081")
        .single_protocol(ProtocolBuilder::new(H1::server()))
        .build()
);

endpoint! {
    APP.url("/"),
    pub index<H1> {
        text_response("Hello from Hotaru + Hyper")
    }
}

endpoint! {
    APP.url("/version"),
    pub version<H1> {
        format!("{:?}", req.request().version())
    }
}

fn main() {
    APP.bind(index).expect("bind index");
    APP.bind(version).expect("bind version");
    run_server!(APP);
}
