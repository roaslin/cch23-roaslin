
use axum::{extract::Multipart, http::StatusCode};
use image::GenericImageView;

pub async fn count_magical_reds(mut multipart: Multipart) -> (StatusCode, String) {
    let image = multipart.next_field().await.unwrap().unwrap();

    let img = image::load_from_memory(image.bytes().await.unwrap().as_ref()).unwrap();
    let magical_reds = img
        .pixels()
        .filter(|x| {
            let [r, g, b, _] = x.2 .0;
            r as u16 > (g as u16 + b as u16)
        })
        .count();

    (StatusCode::OK, format!("{}", magical_reds))
}
