use pages::*;

pub fn content() -> String {
    page("Home", "home", div(&[
        p(&[
          "I'm Jason Longshore, a software engineer from Chicago, IL, U.S. Here's ".into(),
          a(&[href("https://longshorej.github.io/jason-longshore/longshore-jason-resume.pdf"), "my résumé".into()]),
          ".".into()]),
        p(&["You've reached my personal website. Be sure to access the various links to see what I'm currently working on.".into()])
    ]))
}
