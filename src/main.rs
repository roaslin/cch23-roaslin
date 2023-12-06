use axum::{
    extract::{self, Path},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::Deserialize;

static MAX_PACKETS_IN_SLED: usize = 20;

#[derive(Deserialize)]
pub struct Reindeer {
    pub name: String,
    pub strength: i32,
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
    extract::Json(payload): extract::Json<Vec<Reindeer>>,
) -> (StatusCode, String) {
    let total_strength: i32 = payload.into_iter().map(|reindeer| reindeer.strength).sum();

    (StatusCode::OK, format!("{}", total_strength))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(StatusCode::INTERNAL_SERVER_ERROR))
        .route("/1/*packets_ids", get(calculate_sled_id))
        .route("/4/strength", post(calculate_strength));

    Ok(router.into())
}
