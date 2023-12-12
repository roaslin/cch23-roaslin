use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};
use base64::{engine::general_purpose, Engine};
use serde::Serialize;
use serde_json::{Map, Number, Value};

pub async fn decode_cookie_recipie(header: HeaderMap) -> (StatusCode, String) {
    let recipe: &Vec<&str> = &header["Cookie"]
        .to_str()
        .unwrap()
        .split("recipe=")
        .collect();
    let recipe = general_purpose::STANDARD.decode(recipe[1]).unwrap();
    let recipe = String::from_utf8(recipe).unwrap();

    (StatusCode::OK, format!("{}", recipe.trim()))
}

#[derive(Default, Serialize)]
pub struct BakedCookiesResponse {
    pub cookies: u32,
    pub pantry: Map<String, Value>,
}

pub async fn bake_cookie_recipe(header: HeaderMap) -> (StatusCode, Json<BakedCookiesResponse>) {
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
