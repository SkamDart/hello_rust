extern crate actix;
extern crate actix_web;
extern crate futures;

use actix_web::{middleware, server, App, Error, HttpRequest, HttpResponse};
use actix_web::http::{Method};
use futures::future::{result, FutureResult};


// This is an asynchronous function that takes a reference to a http request.
// Here we are only `referring` to this object. As a result, we do not own it
// and we cannot modify it. If we were to attempt to modify this result, the
// Rust compiler would throw an error and not allow us to compile and/or run
// the program.
//
// This idea is the main idea behind Rust's References and Borrowing philsophy.
// For more information please see the documentation
//
// https://doc.rust-lang.org/nightly/book/references-and-borrowing.html
fn autocomplete_async(req: &HttpRequest) -> FutureResult<HttpResponse, Error> {
    println!("{:?}", req);

    result(Ok(HttpResponse::Ok().content_type("text/html").body(
        format!("Hello {}!", req.match_info().get("word").unwrap())
    )))
}

fn serve(hostname: &str) {
    println!("Starting server");
    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            // async handler
            .resource("/autocomplete/{word}", |r| r.method(Method::GET).a(autocomplete_async))
    }).bind(hostname)
        .unwrap()
        .start();
}

fn main() {
    let sys = actix::System::new("hello");

    let hostname = "127.0.0.1:8080".to_string();
    serve(&hostname);

    let _ = sys.run();
    println!("Shutting down server");
}
