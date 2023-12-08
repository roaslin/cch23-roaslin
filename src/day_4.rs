use axum::{extract, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReindeerStrength {
    pub name: String,
    pub strength: i32,
}

#[derive(Deserialize)]
pub struct ReindeerContest {
    pub name: String,
    pub strength: i32,
    pub speed: f32,
    pub height: i32,
    pub antler_width: i32,
    pub snow_magic_power: i32,
    pub favorite_food: String,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    pub candies_eaten_yesterday: i32,
}

#[derive(Serialize)]
pub struct ContestSummary {
    pub fastest: String,
    pub tallest: String,
    pub magician: String,
    pub consumer: String,
}

pub async fn contest(
    extract::Json(payload): extract::Json<Vec<ReindeerContest>>,
) -> (StatusCode, Json<ContestSummary>) {
    let reindeers = payload.iter();
    let fastest = reindeers
        .clone()
        .max_by(|r1, r2| r1.speed.total_cmp(&r2.speed))
        .unwrap();
    let tallest = reindeers
        .clone()
        .max_by(|r1, r2| r1.height.cmp(&r2.height))
        .unwrap();
    let magician = reindeers
        .clone()
        .max_by(|r1, r2| r1.snow_magic_power.cmp(&r2.snow_magic_power))
        .unwrap();
    let consumer = reindeers
        .clone()
        .max_by(|r1, r2| r1.candies_eaten_yesterday.cmp(&r2.candies_eaten_yesterday))
        .unwrap();

    let fastest = format!(
        "Speeding past the finish line with a strength of {} is {}",
        fastest.strength, fastest.name
    );
    let tallest = format!(
        "{} is standing tall with his {} cm wide antlers",
        tallest.name, tallest.antler_width
    );
    let magician = format!(
        "{} could blast you away with a snow magic power of {}",
        magician.name, magician.snow_magic_power
    );
    let consumer = format!("{} ate lots of candies, but also some grass", consumer.name);

    (
        StatusCode::OK,
        Json(ContestSummary {
            fastest,
            tallest,
            magician,
            consumer,
        }),
    )
}

pub async fn calculate_strength(
    extract::Json(payload): extract::Json<Vec<ReindeerStrength>>,
) -> (StatusCode, String) {
    let total_strength: i32 = payload.into_iter().map(|reindeer| reindeer.strength).sum();

    (StatusCode::OK, format!("{}", total_strength))
}
