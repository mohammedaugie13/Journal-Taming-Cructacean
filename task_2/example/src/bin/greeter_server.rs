extern crate futures;

extern crate grpc;

extern crate example;

use std::env;
use std::thread;

use example::helloworld::*;
use example::helloworld_grpc::*;

struct GreeterImpl;

impl Greeter for GreeterImpl {
    fn say_hello(
        &self,
        _m: grpc::RequestOptions,
        req: HelloRequest,
    ) -> grpc::SingleResponse<HelloReply> {
        let mut r = HelloReply::new();
        let name = if req.get_name().is_empty() {
            "world"
        } else {
            req.get_name()
        };
        println!("greeting request from {}", name);
        r.set_message(format!("Hello {}", name));
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let port = 50051;

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(port);
    server.add_service(GreeterServer::new_service_def(GreeterImpl));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("server");

    println!("greeter server started on port {}", port);

    loop {
        thread::park();
    }
}
