extern crate hyper;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server};
use std::fs::File;
use std::io::prelude::*;

fn hello_world(_req: Request<Body>) -> Response<Body> {
    let mut f = File::open("./static/index.html").expect("File not found");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("something went wrong reading the file");
    Response::new(Body::from(buffer))
}

fn main() {
    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn_ok(hello_world)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    hyper::rt::run(server);
}
