pub fn content() -> String {
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

        #content {
            padding-bottom: 30px;
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
