extern crate futures;
extern crate hyper;
extern crate rust_tags;

mod pages;

use futures::future::Future;
use hyper::{Method, StatusCode};
use hyper::header::ContentType;
use hyper::mime;
use hyper::server::{Http, Request, Response, Service};
use pages::*;

struct JasonLongshore;

impl Service for JasonLongshore {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let html = ContentType(mime::TEXT_HTML_UTF_8);
        let css = ContentType(mime::TEXT_CSS);
        let mut response = Response::new();

        let (ct, body) = match (req.method(), req.path()) {
            (&Method::Get, "/") => (html, page_home::content()),
            (&Method::Get, "/css") => (css, page_css::content()),
            (&Method::Get, "/projects") => (html, page_projects::content()),
            (&Method::Get, "/blog") => (html, page_blog::content()),
            (&Method::Get, "/contact") => (html, page_contact::content()),
            _ => {
                response.set_status(StatusCode::NotFound);
                (html, page_404::content())
            },
        };

        let mut response = response.with_header(ct);


        response.set_body(body);

        Box::new(futures::future::ok(response))
    }
}

fn main() {
    let addr_str = "0.0.0.0:3000";
    let addr = addr_str.parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(JasonLongshore)).unwrap();
    println!("listening on {}", addr_str);
    server.run().unwrap();
}
