use axum::{
    extract::{self, Path},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};

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

#[derive(Serialize)]
pub struct ElfCounter {
    pub elf: usize,
    #[serde(rename(serialize = "elf on a shelf"))]
    pub elf_on_a_shelf: usize,
    #[serde(rename(serialize = "shelf with no elf on it"))]
    pub shelf_with_no_elf_on_it: usize,
}

async fn count_elf(payload: String) -> (StatusCode, Json<ElfCounter>) {
    let elf: Vec<&str> = payload.matches("elf").collect();
    let elf_on_a_shelf: Vec<&str> = payload.matches("elf on a shelf").collect();
    let shelf_with_no_elf_on_it = payload.replace("elf on a shelf", "");
    let shelf_with_no_elf_on_it: Vec<&str> = shelf_with_no_elf_on_it.matches("shelf").collect();

    (
        StatusCode::OK,
        Json(ElfCounter {
            elf: elf.len(),
            elf_on_a_shelf: elf_on_a_shelf.len(),
            shelf_with_no_elf_on_it: shelf_with_no_elf_on_it.len(),
        }),
    )
}

async fn decode_cookie_recipie(header: HeaderMap) -> (StatusCode, String) {
    let recipe: &Vec<&str> = &header["Cookie"]
        .to_str()
        .unwrap()
        .split("recipe=")
        .collect();
    let recipe = general_purpose::STANDARD.decode(recipe[1]).unwrap();
    let recipe = String::from_utf8(recipe).unwrap();

    (StatusCode::OK, format!("{}", recipe.trim()))
}

#[derive(Serialize)]
struct BakedCookiesResponse {
    pub cookies: u32,
    pub pantry: Map<String, Value>,
}

async fn bake_cookie_recipe(header: HeaderMap) -> (StatusCode, Json<BakedCookiesResponse>) {
    // Get 🍪
    let baking_request: &Vec<&str> = &header["Cookie"]
        .to_str()
        .unwrap()
        .split("recipe=")
        .collect();
    // Decode Recipe and pantry
    let baking_request = general_purpose::STANDARD.decode(baking_request[1]).unwrap();
    let baking_request: Value = serde_json::from_slice(&baking_request).unwrap();

    let mut cookies: u32 = 0;
    let ingredients = baking_request["recipe"].as_object().unwrap();
    let mut pantry = baking_request["pantry"].as_object().unwrap().clone();

    while has_enough_ingredients(ingredients, &pantry) {
        // at this point we know at least we can do 1 🍪, let's start baking!

        for ingredient in ingredients {
            let ingredient_pantry_quantity = pantry.get(ingredient.0).unwrap().as_u64().unwrap();
            let ingredient_quantity = ingredient.1.as_u64().unwrap();

            pantry.insert(
                String::from(ingredient.0),
                Value::Number(Number::from(
                    ingredient_pantry_quantity - ingredient_quantity,
                )),
            );
        }
        cookies += 1;
    }

    (
        StatusCode::OK,
        Json(BakedCookiesResponse { cookies, pantry }),
    )
}

fn has_enough_ingredients(
    ingredients: &serde_json::Map<String, Value>,
    pantry: &serde_json::Map<String, Value>,
) -> bool {
    for ingredient in ingredients {
        let ingredient_in_pantry = pantry.get(ingredient.0);
        let pantry_quantity = match ingredient_in_pantry {
            Some(quantity) => quantity.as_u64().unwrap(),
            None => {
                return false;
            }
        };
        let ingredient_quantity = ingredient.1.as_u64().unwrap();
        if !(pantry_quantity >= ingredient_quantity) {
            return false;
        }
    }

    return true;
}

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
        .route("/7/bake", get(bake_cookie_recipe));

    Ok(router.into())
}
