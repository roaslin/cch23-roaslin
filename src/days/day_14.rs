use std::collections::HashMap;

use axum::{extract, response::Html};

pub async fn unsafe_html(
    extract::Json(content): extract::Json<HashMap<String, String>>,
) -> Html<String> {
    let html = "<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>";
    let html = html.replace("{}", content.get("content").unwrap());
    let html = html.trim();

    Html(html.to_string())
}
