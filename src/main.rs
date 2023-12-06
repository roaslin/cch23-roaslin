use axum::{
    extract::{self, Path},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

static MAX_PACKETS_IN_SLED: usize = 20;

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

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn calculate_sled_id(Path(packets_ids): Path<String>) -> (StatusCode, String) {
    let sled_id: Vec<&str> = packets_ids.split("/").collect();

    if sled_id.len() > MAX_PACKETS_IN_SLED {
        return (
            StatusCode::BAD_REQUEST,
            String::from("Ho Ho ho!! Only 20 📦 packets fit in a 🛷 sled!"),
        );
    }

    let sled_id = sled_id
        .into_iter()
        .map(|packet_id| packet_id.parse::<i32>().unwrap())
        .reduce(|sled_id, packet_id| sled_id ^ packet_id)
        .unwrap();

    (StatusCode::OK, format!("{}", sled_id.pow(3)))
}

async fn calculate_strength(
    extract::Json(payload): extract::Json<Vec<ReindeerStrength>>,
) -> (StatusCode, String) {
    let total_strength: i32 = payload.into_iter().map(|reindeer| reindeer.strength).sum();

    (StatusCode::OK, format!("{}", total_strength))
}

async fn contest(
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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(StatusCode::INTERNAL_SERVER_ERROR))
        .route("/1/*packets_ids", get(calculate_sled_id))
        .route("/4/strength", post(calculate_strength))
        .route("/4/contest", post(contest));

    Ok(router.into())
}
