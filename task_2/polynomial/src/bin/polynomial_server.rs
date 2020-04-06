extern crate futures;

extern crate grpc;

extern crate polynomial;

use std::thread;

use polynomial::polynomial::*;
use polynomial::polynomial_function::f;
use polynomial::polynomial_grpc::*;

struct PolynomialImpl;

impl Polynomial for PolynomialImpl {
    fn solve(
        &self,
        _m: grpc::RequestOptions,
        req: PolynomialInput,
    ) -> grpc::SingleResponse<PolynomialOutput> {
        let mut input = PolynomialOutput::new();
        let (ordo_5, ordo_4) = f(req.get_x());
        input.set_res_x(ordo_5);
        input.set_res_y(ordo_4);
        grpc::SingleResponse::completed(input)
    }
}

fn main() {
    let port = 50051;

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(port);
    server.add_service(PolynomialServer::new_service_def(PolynomialImpl));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("server");

    println!("greeter server started on port {}", port);

    loop {
        thread::park();
    }
}
