extern crate futures;
extern crate grpc;

extern crate polynomial;

use grpc::ClientStubExt;

use polynomial::polynomial::*;
use polynomial::polynomial_grpc::*;
use polynomial::polynomial_plot::plot;

fn main() {
    let values = vec![
        -1.0,
        -0.77777778,
        -0.55555556,
        -0.33333333,
        -0.11111111,
        0.11111111,
        0.33333,
        0.55555556,
        0.77777778,
        1.0,
    ];

    let mut result_x = Vec::new();

    let mut result_y = Vec::new();

    let port = 50051;

    let client_conf = Default::default();

    let client = PolynomialClient::new_plain("::1", port, client_conf).unwrap();

    for value in values.iter() {
        let mut req = PolynomialInput::new();
        req.set_x(*value);
        let resp_x = client.solve(grpc::RequestOptions::new(), req.clone());
        let resp_y = client.solve(grpc::RequestOptions::new(), req.clone());
        result_x.push(resp_x.wait_drop_metadata().unwrap().res_x);
        result_y.push(resp_y.wait_drop_metadata().unwrap().res_y)
    }

    plot(values, result_x, result_y, "Ordo 5", "Ordo 4");
}
