extern crate env_logger;
extern crate futures;
extern crate grpc;
extern crate httpbis;

extern crate example;

use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use grpc::ClientStub;
use grpc::ClientStubExt;

use example::helloworld::*;
use example::helloworld_grpc::*;

fn main() {
    env_logger::init();

    let name = env::args()
        .nth(1)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "world".to_string());

    let port = 50051;

    let client_conf = Default::default();

    let client = GreeterClient::new_plain("::1", port, client_conf).unwrap();

    let mut req = HelloRequest::new();
    req.set_name(name);

    let resp = client.say_hello(grpc::RequestOptions::new(), req);

    println!("{:?}", resp.wait());
}
