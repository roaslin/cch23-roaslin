use std::sync::Arc;

use crate::AppState;
use axum::{
    extract::{self, Path, State},
    Json,
};
use chrono::{Datelike, TimeZone};
use reqwest::StatusCode;
use serde::Serialize;

pub async fn store_packet_id(
    Path(packet_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> StatusCode {
    state.add_packet(packet_id);
    StatusCode::OK
}

pub async fn load_packet_id(
    Path(packet_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, String) {
    let elapsed_time = match state.get_packet(packet_id.clone()) {
        Some(value) => value.elapsed().unwrap().as_secs(),
        None => {
            return (
                StatusCode::NOT_FOUND,
                format!("Not found packet with id {}", packet_id),
            );
        }
    };

    (StatusCode::OK, format!("{}", elapsed_time))
}

pub async fn convert_ulids_to_uuids(
    extract::Json(payload): extract::Json<Vec<String>>,
) -> (StatusCode, Json<Vec<String>>) {
    (
        StatusCode::OK,
        Json(
            payload
                .into_iter()
                .rev()
                .map(|x| uuid::Uuid::from_u128(ulid::Ulid::from_string(&x).unwrap().0).to_string())
                .collect(),
        ),
    )
}

#[derive(Serialize, Default)]
pub struct LsbResponse {
    #[serde(rename = "christmas eve")]
    xmas_eve: u64,
    weekday: u64,
    #[serde(rename = "in the future")]
    in_the_future: u64,
    lsb: u64,
}

pub async fn let_santa_broil(
    Path(weekday): Path<u8>,
    extract::Json(payload): extract::Json<Vec<String>>,
) -> (StatusCode, Json<LsbResponse>) {
    let mut xmas_eve: u64 = 0;
    let mut week_day: u64 = 0;
    let mut in_the_future: u64 = 0;
    let mut lsb: u64 = 0;
    let today = chrono::Utc::now();
    let weekday = chrono::Weekday::try_from(weekday).unwrap();

    for item in payload {
        let ulid = ulid::Ulid::from_string(&item).unwrap();
        let time_stamp = ulid.timestamp_ms() as i64;
        let date = chrono::Utc.timestamp_millis_opt(time_stamp).unwrap();

        if date.day() == 24 && date.month() == 12 {
            xmas_eve += 1;
        }

        if date.weekday() == weekday {
            week_day += 1;
        }

        if date.timestamp_millis() > today.timestamp_millis() {
            in_the_future += 1;
        }
        if ulid.0 & 1 == 1 {
            lsb += 1;
        }
    }

    (
        StatusCode::OK,
        Json(LsbResponse {
            xmas_eve,
            weekday: week_day,
            in_the_future,
            lsb,
        }),
    )
}
