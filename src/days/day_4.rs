use axum::{extract, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReindeerStrength {
    pub name: String,
    pub strength: i32,
}

#[derive(Deserialize)]
pub struct ReindeerContest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub strength: i64,
    #[serde(default)]
    pub speed: f64,
    #[serde(default)]
    pub height: i64,
    #[serde(default)]
    pub antler_width: i64,
    #[serde(default)]
    pub snow_magic_power: i64,
    #[serde(default)]
    pub favorite_food: String,
    #[serde(default, rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    pub candies_eaten_yesterday: i64,
}

#[derive(Default, Serialize)]
pub struct ContestSummary {
    pub fastest: String,
    pub tallest: String,
    pub magician: String,
    pub consumer: String,
}

pub async fn contest(
    extract::Json(payload): extract::Json<Vec<ReindeerContest>>,
) -> (StatusCode, Json<ContestSummary>) {
    if payload.len() == 0 {
        return (StatusCode::OK, Json(ContestSummary::default()));
    }
    let first = payload.first().unwrap();
    let (mut fastest, mut tallest, mut magician, mut consumer) = (first, first, first, first);
    for x in &payload {
        if fastest.speed < x.speed {
            fastest = x;
        }
        if tallest.height < x.height {
            tallest = x;
        }
        if magician.snow_magic_power < x.snow_magic_power {
            magician = x;
        }
        if consumer.candies_eaten_yesterday < x.candies_eaten_yesterday {
            consumer = x;
        }
    }

    let fastest = format!(
        "Speeding past the finish line with a strength of {} is {}",
        fastest.strength, fastest.name,
    );
    let tallest = format!(
        "{} is standing tall with his {} cm wide antlers",
        tallest.name, tallest.antler_width,
    );
    let magician = format!(
        "{} could blast you away with a snow magic power of {}",
        magician.name, magician.snow_magic_power,
    );
    let consumer = format!(
        "{} ate lots of candies, but also some {}",
        consumer.name, consumer.favorite_food,
    );

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
