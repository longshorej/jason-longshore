use pages::*;
use rust_tags::attributes::style;

pub fn content() -> String {
    page("Projects", "projects", div(&[
      h4(&[style("margin-top: 0; padding-top: 0;"),
         a(&[href("https://github.com/longshorej/dotfiles"), "dotfiles".into()])]),

      "
      Contains configuration files for numerous software packages,
      including XMonad and vim.
      ".into(),


      h4(&[a(&[href("https://github.com/longshorej/rust-tags"), "rust-tags".into()])]),

      "
      An HTML \"templating\" library for Rust. Templates are
      created using plain function calls.
      ".into(),

      h4(&[a(&[href("https://github.com/longshorej/jason-longshore"), "jason-longshore".into()])]),

      "
      The source code to this website. It's a simple HTTP server written in Rust
      that uses rust-tags for its content.
      ".into(),
    ]))
}
