use pages::*;

pub fn content() -> String {
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
