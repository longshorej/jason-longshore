extern crate futures;
extern crate hyper;
extern crate rust_tags;

use futures::future::Future;
use hyper::{Method, StatusCode};
use hyper::header::{ContentLength, ContentType};
use hyper::mime;
use hyper::server::{Http, Request, Response, Service};
use rust_tags::attributes::*;
use rust_tags::attributes::*;
use rust_tags::core::*;
use rust_tags::tags::*;
use rust_tags::tags::{style,title};
use rust_tags::extras::*;

const URL_KEYBASE: &str = "https://keybase.io/longshorej";

fn page(sub_title: &str, section: &str, c: Fragment) -> String {
    let frag = html(&[
        head(&[
            title(&[format!("{} - Jason Longshore", sub_title).into()]),
            link(&[rel("stylesheet"), href("/css")])
        ]),
        body(&[
             div(&[
                 id("page"),
                 header(),
                 content(c, section),
                 footer()
            ])
        ])
    ]);

    format!("<!doctype html>\n{}", frag.data)
}

fn header() -> Fragment {
    div(&[
        id("header"),
        class("container"),
        h5(&[id("name"), "Jason Longshore".into()])])
}

fn footer() -> Fragment {
    div(&[id("footer")])
}

fn content(c: Fragment, section: &str) -> Fragment {
    let class_name = |s: &str| {
        if section == s {
            "active"
        } else {
            "inactive"
        }
    };

    div(&[
        id("content-container"),
        class("container"),
        div(&[
            id("nav"),
            class("col-3"),
            div(&[class("link"), a(&[href("/"), class(class_name("home")), "Home".into()])]),
            div(&[class("link"), a(&[href("/resume"), class(class_name("resume")), "Résumé".into()])]),
            div(&[class("link"), a(&[href("/projects"), class(class_name("projects")), "Projects".into()])]),
            div(&[class("link"), a(&[href("/contact"), class(class_name("contact")), "Contact".into()])]),
            br(),
            br(),
            //div(&[class("link"), a(&[href("/blog"), "Blog".into()])]),
            div(&[class("link"), a(&[href("https://github.com/longshorej"), "GitHub".into()])]),
            div(&[class("link"), a(&[href(URL_KEYBASE), "Keybase".into()])]),
            div(&[class("link"), a(&[href("https://twitter.com/longshorej9"), "Twitter".into()])])
        ]),
        div(&[
            id("content"),
            class("col-9"),
            br(),
            c
        ])
    ])
}

fn decorate_with_nav(c: Fragment) -> Fragment {
    div(&[
      h3(&["Jason Longshore".into()])
    ])
}

fn page_home() -> String {
    page("Home", "home", nl2br("I'm Jason Longshore, a software engineer from Chicago, IL, U.S.

        You've reached my personal website. Be sure to access the various links to see what I'm currently working on.
    ".into()))
}

fn page_resume() -> String {
    page("Resume", "resume", "".into())
}

fn page_projects() -> String {
    page("Projects", "projects", "".into())
}

fn page_blog() -> String {
    page("Blog", "blog", "Blog!".into())
}

fn page_contact() -> String {
    page("Contact", "contact", div(&[
        div(&[
            "Want to get in touch? I have an ".into(),
            a(&[
              href("mailto:longshorej@gmail.com"),
              "email address".into()]),
            " that you can send email to.".into()]),
        p(&[
            "I'm also on ".into(), a(&[href(URL_KEYBASE), "Keybase".into()]), " if you need to send something confidential.".into()])
    ]))
}

fn page_404() -> String {
    page("404", "404", "404".into())
}

fn page_css() -> String {
    "
        *, *:after, *:before {
            -webkit-box-sizing: border-box;
            -moz-box-sizing: border-box;
            box-sizing: border-box;
        }

        html, body {
            margin: 0;
            padding: 0;
        }

        html {
            font: 112.5%/1.444444444 \"Runda\", \"Helvetica Neue\", Helvetica, Arial, sans-serif;
            -webkit-font-smoothing: antialiased;
            color: #ffffff;
            background-color: #47525d;
            background-color: #323a42;
            margin-top: 30px;
        }


        a, a:link, a:visited {
            color: #ffffff;
            text-decoration: underline;
        }

        a:hover, a:active, a.active {
            color: #38A2FF;
            text-decoration: underline;
        }

        #nav a, #nav a:link, #nav a:visited {
            text-decoration: none;
        }

        h1, h2, h3, h4, h5, h6 {
            font-family: \"Georgia\", \"Times New Roman\", serif;
            font-weight: normal;
        }

        #name {
            margin-top: 0;
        }

        #nav {
            font-weight: bold;
        }

        .container {
            max-width: 850px;
            padding: 0 30px 0 30px;
            margin: 0 auto;
        }

        [class*='col-'] {
          float: left;
          padding-right: @grid-padding;
        }

        [class*='col-']:last-of-type {
          padding-right: 0;
        }

        .col-1 {
          width: 8.33%;
        }

        .col-2 {
          width: 16.66%;
        }

        .col-3 {
          width: 25%;
        }

        .col-4 {
          width: 33.33%;
        }

        .col-5 {
          width: 41.66%;
        }

        .col-6 {
          width: 50%;
        }

        .col-7 {
          width: 58.33%;
        }

        .col-8 {
          width: 66.66%;
        }

        .col-9 {
          width: 75%;
        }

        .col-10 {
          width: 83.33%;
        }

        .col-11 {
          width: 91.66%;
        }

        .col-12 {
          width: 100%;
        }

        h1 {
          font-size: 40px;
        }

        h2 {
          font-size: 36px;
        }

        h3 {
          font-size: 32px;
        }

        h4 {
          font-size: 28px;
        }

        h5 {
          font-size: 24px;
        }

        h6 {
          font-size: 20px;
        }

        .row::after {
          content: \"\";
          clear: both;
          display: block;
        }
    ".to_string()
}

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
            (&Method::Get, "/") => (html, page_home()),
            (&Method::Get, "/css") => (css, page_css()),
            (&Method::Get, "/resume") => (html, page_resume()),
            (&Method::Get, "/projects") => (html, page_projects()),
            (&Method::Get, "/blog") => (html, page_blog()),
            (&Method::Get, "/contact") => (html, page_contact()),
            _ => {
                response.set_status(StatusCode::NotFound);
                (html, page_404())
            },
        };

        let mut response = response.with_header(ct);

        response.set_body(body);

        Box::new(futures::future::ok(response))
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(JasonLongshore)).unwrap();
    server.run().unwrap();


}
