use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};

mod days;

use days::{
    day_0::hello_world,
    day_1::calculate_sled_id,
    day_11::count_magical_reds,
    day_12::{convert_ulids_to_uuids, let_santa_broil, load_packet_id, store_packet_id},
    day_13::{orders, popular, reset, task1, total},
    day_14::{safe_html, unsafe_html},
    day_15::validate_nice,
    day_18::{reset_day_18, orders_day_18, regions, regions_total},
    day_4::{calculate_strength, contest},
    day_6::count_elf,
    day_7::{bake_cookie_recipe, decode_cookie_recipe},
    day_8::{pokemon_momentum_by_id, pokemon_weight_by_id},
};
use shuttle_runtime::CustomError;
use sqlx::PgPool;
use tower_http::services::ServeDir;

#[derive(Debug)]
struct AppState {
    pub packets: Mutex<HashMap<String, SystemTime>>,
    pub pool: PgPool,
}

impl AppState {
    pub fn add_packet(&self, packet_id: String) {
        match self.packets.lock() {
            Ok(mut packets) => {
                packets.insert(packet_id, SystemTime::now());
            }
            Err(_) => println!("Locked failed"),
        }
    }

    pub fn get_packet(&self, packet_id: String) -> Option<SystemTime> {
        match self.packets.lock() {
            Ok(packets) => {
                return packets.get(&packet_id).copied();
            }
            Err(_) => None,
        }
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        // local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:17155/postgres"
    )]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    let app_state = Arc::new(AppState {
        packets: Mutex::new(HashMap::new()),
        pool,
    });

    sqlx::migrate!()
        .run(&app_state.pool)
        .await
        .map_err(CustomError::new)?;

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(StatusCode::INTERNAL_SERVER_ERROR))
        .route("/1/*packets_ids", get(calculate_sled_id))
        .route("/4/strength", post(calculate_strength))
        .route("/4/contest", post(contest))
        .route("/6", post(count_elf))
        .route("/7/decode", get(decode_cookie_recipe))
        .route("/7/bake", get(bake_cookie_recipe))
        .route("/8/weight/:pokedex_number", get(pokemon_weight_by_id))
        .route("/8/drop/:pokedex_number", get(pokemon_momentum_by_id))
        .nest_service("/11/assets", ServeDir::new("assets"))
        .route("/11/red_pixels", post(count_magical_reds))
        .route("/12/save/:packet_id", post(store_packet_id))
        .route("/12/load/:packet_id", get(load_packet_id))
        .route("/12/ulids", post(convert_ulids_to_uuids))
        .route("/12/ulids/:weekday", post(let_santa_broil))
        .route("/13/sql", get(task1))
        .route("/13/orders", post(orders))
        .route("/13/orders/total", get(total))
        .route("/13/orders/popular", get(popular))
        .route("/13/reset", post(reset))
        .route("/14/unsafe", post(unsafe_html))
        .route("/14/safe", post(safe_html))
        .route("/15/nice", post(validate_nice))
        // .route("/15/game", post(validate_nice))
        .route("/18/reset", post(reset_day_18))
        .route("/18/orders", post(orders_day_18))
        .route("/18/regions", post(regions))
        .route("/18/regions/total", get(regions_total))
        .with_state(app_state);

    Ok(router.into())
}
