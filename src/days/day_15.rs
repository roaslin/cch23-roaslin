use axum::{extract, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestNice {
    input: String,
}

#[derive(Serialize)]
pub enum PasswordKind {
    #[serde(rename(serialize = "nice"))]
    Nice,
    #[serde(rename(serialize = "naughty"))]
    Naughty,
}

#[derive(Serialize)]
pub struct ResponseNice {
    result: PasswordKind,
}

pub async fn validate_nice(
    extract::Json(payload): Json<RequestNice>,
) -> (StatusCode, Json<ResponseNice>) {
    const INVALID_SUBSTRINGS: &[&str] = &["ab", "cd", "pq", "xy"];
    const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u', 'y'];
    let input = payload.input;

    for sub in INVALID_SUBSTRINGS {
        if input.contains(sub) {
            return (
                StatusCode::BAD_REQUEST,
                Json(ResponseNice {
                    result: PasswordKind::Naughty,
                }),
            );
        }
    }
    let mut count_vowels = 0;
    let mut consecutive = false;
    for i in 0..input.len() - 2 {
        println!("VALUE input is {}", input);
        let c1 = &input[i..i + 1].chars().next().unwrap();
        let c2 = &input[i + 1..i + 2].chars().next().unwrap();
        if VOWELS.contains(c1) {
            count_vowels += 1;
        }
        if c1.is_alphabetic() && c1 == c2 {
            consecutive = true;
        }
    }
    if count_vowels > 2 && consecutive {
        return (
            StatusCode::OK,
            Json(ResponseNice {
                result: PasswordKind::Nice,
            }),
        );
    }
    (
        StatusCode::BAD_REQUEST,
        Json(ResponseNice {
            result: PasswordKind::Naughty,
        }),
    )
}
