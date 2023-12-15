use std::collections::HashMap;

use axum::{extract, response::Html};
use minijinja::{context, Environment};

pub async fn unsafe_html(
    extract::Json(content): extract::Json<HashMap<String, String>>,
) -> Html<String> {
    let html = "<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {{content | safe}}
  </body>
</html>";
    // let html = html.replace("{}", content.get("content").unwrap());
    // let html = html.trim();
    let mut env = Environment::new();
    env.add_template("unsafe", html).unwrap();
    let tmpl = env.get_template("unsafe").unwrap();

    Html(
        tmpl.render(context!(content => content.get("content")))
            .unwrap()
            .trim()
            .to_string(),
    )
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Content {
    content: String,
}

pub async fn safe_html(extract::Json(content): extract::Json<Content>) -> Html<String> {
    let html = "<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {{content | escape}}
  </body>
</html>";

    // let content = html_escape::encode_text(&content);
    // let html = html.replace("{}", &content).replace("\"", "&quot;");
    // let html = html.trim();
    println!("Content is {:?}", content);
    let mut env = Environment::new();
    env.add_template("safe", html).unwrap();
    let tmpl = env.get_template("safe").unwrap();
    let render = tmpl.render(context!(content => content.content)).unwrap();
    Html(render.replace("&#x2f;", "/").trim().to_string())
}
