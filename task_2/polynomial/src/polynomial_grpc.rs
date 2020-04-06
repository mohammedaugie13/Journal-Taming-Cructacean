// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Polynomial {
    fn solve(&self, o: ::grpc::RequestOptions, p: super::polynomial::PolynomialInput) -> ::grpc::SingleResponse<super::polynomial::PolynomialOutput>;
}

// client

pub struct PolynomialClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Solve: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::polynomial::PolynomialInput, super::polynomial::PolynomialOutput>>,
}

impl ::grpc::ClientStub for PolynomialClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        PolynomialClient {
            grpc_client: grpc_client,
            method_Solve: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Polynomial/Solve".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Polynomial for PolynomialClient {
    fn solve(&self, o: ::grpc::RequestOptions, p: super::polynomial::PolynomialInput) -> ::grpc::SingleResponse<super::polynomial::PolynomialOutput> {
        self.grpc_client.call_unary(o, p, self.method_Solve.clone())
    }
}

// server

pub struct PolynomialServer;


impl PolynomialServer {
    pub fn new_service_def<H : Polynomial + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/Polynomial",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Polynomial/Solve".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.solve(o, p))
                    },
                ),
            ],
        )
    }
}
