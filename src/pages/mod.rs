extern crate futures;
extern crate hyper;
extern crate rust_tags;

use rust_tags::attributes::*;
use rust_tags::core::*;
use rust_tags::tags::*;
use rust_tags::tags::{title};

pub mod page_404;
pub mod page_blog;
pub mod page_contact;
pub mod page_css;
pub mod page_home;
pub mod page_projects;

pub const URL_KEYBASE: &str = "https://keybase.io/longshorej";

pub fn page(sub_title: &str, section: &str, c: Fragment) -> String {
    let frag = html(&[
        head(&[
            meta(&[http_equiv("Content-Type"), rust_tags::attributes::content("text/html; charset=UTF-8")]),
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
