use axum::extract::Path;
use reqwest::StatusCode;
use serde_json::Value;

static POKEMON_API_URL: &str = "https://pokeapi.co/api/v2/pokemon";

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
