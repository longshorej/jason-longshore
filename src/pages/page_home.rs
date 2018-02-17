use pages::*;

pub fn content() -> String {
    page("Home", "home", nl2br("I'm Jason Longshore, a software engineer from Chicago, IL, U.S.

        You've reached my personal website. Be sure to access the various links to see what I'm currently working on.
    ".into()))
}
