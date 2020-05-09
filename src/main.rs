extern crate hyper;

extern crate futures;

#[macro_use]

extern crate serde_json;

use hyper::{Body, Response, Server, Method, StatusCode};

use hyper::service::service_fn_ok;

// This is here because we use map_error.

use futures::Future;

fn main() {

    let router = || {

        service_fn_ok(|req| {

            match(req.method(), req.uri().path()) {

                (&Method::GET, "/ping") => {

                    Response::new(

                        Body::from(

                            json!({"message": "pong"}).to_string()

                        )

                    )

                },

                (&Method::GET, "/king") => {

                    Response::new(

                        Body::from(

                            json!({"message": "style"}).to_string()

                        )

                    )

                },

                (_, _) => {

                    let mut res = Response::new(Body::from("not found"));

                    *res.status_mut() = StatusCode::NOT_FOUND;

                    res

                }

            }

        })

    };

    // Setup and run the server

    let addr  = "127.0.0.1:8080".parse().unwrap();

    let server = Server::bind(&addr).serve(router);

    hyper::rt::run(server.map_err(|e| {

        eprintln!("server error: {}", e);

    }));

}