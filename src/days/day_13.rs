use std::sync::Arc;

use axum::{
    extract::{self, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::FromRow;

use crate::AppState;

pub async fn task1(State(state): State<Arc<AppState>>) -> (StatusCode, String) {
    let result: (String,) = sqlx::query_as("SELECT $1")
        .bind(20231213.to_string())
        .fetch_one(&state.pool)
        .await
        .unwrap();

    (StatusCode::OK, result.0)
}

#[derive(Debug, Deserialize, FromRow)]
pub struct Order {
    pub id: i32,
    #[serde(default)]
    pub region_id: i32,
    pub gift_name: String,
    #[serde(default)]
    pub quantity: i32,
}

pub async fn orders(
    State(state): State<Arc<AppState>>,
    extract::Json(orders): extract::Json<Vec<Order>>,
) -> StatusCode {
    // println!("Orders {:?}", orders);
    for order in orders {
        match sqlx::query_as::<_, Order>(
            "INSERT INTO orders (id, region_id, gift_name, quantity) VALUES ($1, $2,$3,$4) RETURNING id, region_id, gift_name, quantity",
        )
        .bind(&order.id)
        .bind(&order.region_id)
        .bind(&order.gift_name)
        .bind(&order.quantity)
        .fetch_one(&state.pool)
        .await
        {
            Ok(order) => println!("inserted order with id {:?}", order),
            Err(e) => {
                println!("Error {:?}", e);
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
    }

    StatusCode::OK
}

#[derive(Default, Serialize, FromRow)]
pub struct Total {
    #[serde(default)]
    pub total: i64,
}

pub async fn total(State(state): State<Arc<AppState>>) -> (StatusCode, Json<Total>) {
    match sqlx::query_as::<_, Total>("SELECT SUM(quantity) as total FROM orders")
        .fetch_one(&state.pool)
        .await
    {
        Ok(total) => (StatusCode::OK, Json(total)),
        Err(_) => (StatusCode::OK, Json(Total::default())),
    }
}

#[derive(Default, Serialize, FromRow)]
pub struct PopularRow {
    pub popular: String,
}

#[derive(Default, Serialize)]
pub struct Popular {
    #[serde(default)]
    pub popular: Value,
}

pub async fn popular(State(state): State<Arc<AppState>>) -> Json<Popular> {
    match sqlx::query_as::<_, PopularRow>(
        "SELECT gift_name as popular FROM orders GROUP BY gift_name ORDER BY SUM(id) DESC LIMIT 1",
    )
    .fetch_one(&state.pool)
    .await
    {
        Ok(popular) => {
            if popular.popular.len() ==0{
                return Json(Popular{
                    popular: json!(null)
                });
            }
            
            Json(Popular { popular: Value::String(popular.popular) })
        },
        Err(e) => {
            println!("Error is {}", e);
            Json(Popular::default())},
    }
}

pub async fn reset(State(state): State<Arc<AppState>>) -> StatusCode {
     sqlx::query("DELETE FROM orders")
        .execute(&state.pool)
        .await
        .unwrap();

    StatusCode::OK
}
