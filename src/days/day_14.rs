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
// html_escape::encode_script_single_quoted_text_to_string("<script>'s end tag is </script>", &mut html));

pub async fn safe_html(
    extract::Json(content): extract::Json<HashMap<String, String>>,
) -> Html<String> {
    println!("Content is {:?}", content);
    let html = "<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>";

    let content = content.get("content").unwrap().to_string();
    let content = html_escape::encode_text(&content);
    let html = html.replace("{}", &content).replace("\"", "&quot;");
    let html = html.trim();

    Html(html.to_string())
}
