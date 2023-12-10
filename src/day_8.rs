use axum::extract::Path;
use reqwest::StatusCode;
use serde_json::Value;

static POKEMON_API_URL: &str = "https://pokeapi.co/api/v2/pokemon";
static GRAVITATIONAL_ACCELERATION: f64 = 9.825;

pub async fn pokemon_weight_by_id(Path(pokedex_number): Path<String>) -> (StatusCode, String) {
    let request = format!("{}/{}", POKEMON_API_URL, pokedex_number);
    let response = reqwest::get(request).await;

    let response = match response {
        Ok(response) => {
            if response.status() == StatusCode::NOT_FOUND {
                return (
                    StatusCode::OK,
                    format!("{}", response.text().await.unwrap()),
                );
            }
            let weight = response.text().await.unwrap();
            let weight: Value = serde_json::from_str(&weight).unwrap();

            (
                StatusCode::OK,
                format!("{}", weight["weight"].as_number().unwrap()),
            )
        }
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong {}", error.to_string()),
        ),
    };
    response
}

pub async fn pokemon_momentum_by_id(Path(pokedex_number): Path<String>) -> (StatusCode, String) {
    let request = format!("{}/{}", POKEMON_API_URL, pokedex_number);
    let response = reqwest::get(request).await;

    let response = match response {
        Ok(response) => {
            if response.status() == StatusCode::NOT_FOUND {
                return (
                    StatusCode::OK,
                    format!("{}", response.text().await.unwrap()),
                );
            }
            let weight = response.text().await.unwrap();
            let weight: Value = serde_json::from_str(&weight).unwrap();
            // Pokemon weight is in Hectograms, convert it to Kilograms
            let weight: f64 = weight["weight"].as_number().unwrap().as_f64().unwrap() / 10.0;
            let momentum = compute_momentum(weight, 10.0);
            (StatusCode::OK, format!("{}", momentum))
        }
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong {}", error.to_string()),
        ),
    };
    response
}

/// Velocity = SQRT(2 * g * h)
/// p(momentum) = m * v (velocity)
/// p= m * SQRT(2 * g * h)
/// Weight is Kilograms
/// Height in meters
fn compute_momentum(mass: f64, height: f64) -> f64 {
    let velocity: f64 = 2.0 * GRAVITATIONAL_ACCELERATION * height;
    let velocity = velocity.sqrt();
    mass * velocity
}
