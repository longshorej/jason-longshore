use pages::*;
use rust_tags::attributes::style;

pub fn content() -> String {
    page("Contact", "contact", div(&[
        div(&[
            "Want to get in touch? I have an ".into(),
            a(&[
              href("mailto:hello@jasonlongshore.com"),
              "email address".into()]),
            " that you can send email to.".into()]),
        p(&[
            "If you need to send something confidential, I'm on ".into(), a(&[href(URL_KEYBASE), "Keybase".into()]), ". My PGP public key is also listed below.".into()]),

        hr(&[]),

        div(&[style("padding-left: 15px;font-size: smaller;"),
            pre(&[
            "
-----BEGIN PGP PUBLIC KEY BLOCK-----
mQENBFqJz6EBCADuNUzQ8FckMiybbu4dPIq7Yp0sShXtERtzFS/vKxGr41Ir2bfi
uigByC5jjuI2TDDADMyBIgFjJkunG5p9ApEpaHb4w8nIJgriXy99c9SQITodz5Oi
3yehMCOQQCl7Jb7REBpzJqo9DyvHxHuyYBb4OA7sGKvfKuoM2YZz2Xnnl55P/IEP
xEFuASlyPmSaNHlE7I9nsjLF/a0ccdl4jjQFE1+JmHtD83KKS0YHsISmc+gI1Koh
DdzAD9oxy+Tega7+rKnpVwBksrjkqSxS9nMouS745smSwRAtObXvxIct8h8FzU7O
esEnYHqWveoOcZZZZ/5wCKyYCShfdhW8+PYFABEBAAG0Kkphc29uIExvbmdzaG9y
ZSA8aGVsbG9AamFzb25sb25nc2hvcmUuY29tPokBTgQTAQgAOBYhBDY3DFKa34xA
MZfsGPM7obXp1JTpBQJaic+hAhsDBQsJCAcCBhUKCQgLAgQWAgMBAh4BAheAAAoJ
EPM7obXp1JTpqzAIAOz+1V91YvUtBaGla4TdyCqXfT7R/uIgk+9I8Yl429/rsIFD
02rfqi2maVcKx9AMbMz/x287DgZjfJ5PGo40kd2BiF25wUldQJuZIjKHqYuNMZLy
HbKNzl2O7IhOWJlJrvLkMftDnaDBJx7Nt4EmLO8LvWod/srKEB2ZaGSaO6Yh0FLC
GVb40o3lJPlNgp+kE1Zw3vPl+TqNHvkKoHzQpA7dhxEvdky0BhEeiVUSAzcO4kO8
wDFRJWUfq5xxFXHOXAL7zloBQCUtN4BGwiCQFr5AK2J8F18rW9TzMdwhBobyqrOi
QE7fj6lhFMO5zpFkwwhcobsO36EzAN/3BTIrgOy5AQ0EWonPoQEIALxEBhlyAGj2
p8oeqrUBOSQaTQxTwkA/KUlsbnPciAlQA6MBflJAcq4mkbxxpInJ9rUMbl/Dz/4A
tE0QYEsu6SdY/jayxMvsGU9mf1HsG5wqwj3eMHILx8ZuFAJmD/HaquYsfLEoutmO
a6T2N5aDACzbkiJyVw0pyOS4qFTmxoAuiBNNa1VTslb+PILUdRvmWu+GqgU3dhUJ
xNaoyPtFubPxglPc6c2t8L0Eprthgazwm9UtmdCuGKgvNLtqoaYS+TRQ1NROy32M
85gnd4QgAXRDI3N6SW9a8BC1X0Y9oeNkeH3IHWxmwdvbqLpn+XAFxRdTkO8XaJvs
Bx9btxcRBYMAEQEAAYkBNgQYAQgAIBYhBDY3DFKa34xAMZfsGPM7obXp1JTpBQJa
ic+hAhsMAAoJEPM7obXp1JTp6c0IANV85nEstzNjbEMRtSKGu/F//2yOkbkesG/+
hbectQd/s9KF5qsdfH73g74Tr6HPiNnNa2rd8M74vzZB2dfR5jbCDF3wGSzAMKyr
EVaPNypategkHl2DV9r3F3pljuS0yIWi7PSmDwyUy0Sd3WbKh5xumPLedDLnx2B6
+r2r4KGhY2NJNa35wmhE+0j8ROvTrXeS2OEPnnXXs9B7EqgFIbi6ZaEnkL+cJx9c
o841rVNUWJnt0SPHk1e8JzamNZ61TgS8vyFDhKFUK8obXT+iT2ptsUFWUl+a2CFf
1rw81LXLKp9qhR+QrdrbQonLBBafzlBEK1UHQ8M0Xngfevp+gp8=
=BzhM
-----END PGP PUBLIC KEY BLOCK-----".into()])])

    ]))
}
