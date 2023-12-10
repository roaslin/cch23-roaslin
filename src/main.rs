use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use cch23_roaslin::{
    day_0::hello_world,
    day_1::calculate_sled_id,
    day_4::{calculate_strength, contest},
    day_6::count_elf,
    day_7::{bake_cookie_recipe, decode_cookie_recipie},
    day_8::{pokemon_momentum_by_id, pokemon_weight_by_id},
};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(StatusCode::INTERNAL_SERVER_ERROR))
        .route("/1/*packets_ids", get(calculate_sled_id))
        .route("/4/strength", post(calculate_strength))
        .route("/4/contest", post(contest))
        .route("/6", post(count_elf))
        .route("/7/decode", get(decode_cookie_recipie))
        .route("/7/bake", get(bake_cookie_recipe))
        .route("/8/weight/:pokedex_number", get(pokemon_weight_by_id))
        .route("/8/drop/:pokedex_number", get(pokemon_momentum_by_id));

    Ok(router.into())
}
