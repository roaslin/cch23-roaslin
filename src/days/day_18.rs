use std::sync::Arc;

use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::AppState;

#[derive(Debug, Deserialize, FromRow)]
pub struct Order {
    pub id: i32,
    #[serde(default)]
    pub region_id: i32,
    pub gift_name: String,
    #[serde(default)]
    pub quantity: i32,
}

pub async fn orders_day_18(
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

#[derive(Debug, Deserialize, FromRow)]
pub struct Region {
    pub id: i32,
    pub name: String,
}

pub async fn regions(
    State(state): State<Arc<AppState>>,
    extract::Json(regions): extract::Json<Vec<Region>>,
) -> StatusCode {
    println!("regions {:?}", regions);
    for region in regions {
        match sqlx::query_as::<_, Region>(
            "INSERT INTO regions (id, name) VALUES ($1, $2) RETURNING id, name",
        )
        .bind(&region.id)
        .bind(&region.name)
        .fetch_one(&state.pool)
        .await
        {
            Ok(region) => println!("inserted region with id {:?}", region),
            Err(e) => {
                println!("Error {:?}", e);
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
    }

    StatusCode::OK
}

#[derive(Debug, Default, Serialize, FromRow)]
pub struct TotalByRegion {
    pub region: String,
    #[serde(default)]
    pub total: i64,
}

pub async fn regions_total(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Vec<TotalByRegion>>) {
    match sqlx::query_as::<_, TotalByRegion>(
        "SELECT r.name as region, SUM(o.quantity) as total 
                                            FROM regions r
                                            JOIN orders o ON o.region_id = r.id
                                            GROUP BY r.name
                                            ",
    )
    .fetch_all(&state.pool)
    .await
    {
        Ok(total) => {
            println!("regions {:?}", total);
            (StatusCode::OK, Json(total))
        }
        Err(error) => {
            println!("error is {}", error);
            (StatusCode::OK, Json(Vec::default()))
        }
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct TopListGiftsByREgion {
    pub region: String,
    pub top_gifts: Vec<String>,
}

pub async fn regions_top_list(
    Path(top_no_gifts): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Vec<TopListGiftsByREgion>>) {
    if top_no_gifts < 1 {
        match sqlx::query_as::<_, TopListGiftsByREgion>(
            " SELECT name as region, ARRAY[]::text[] as top_gifts FROM regions",
        )
        .bind(&top_no_gifts)
        .fetch_all(&state.pool)
        .await
        {
            Ok(total) => {
                // println!("regions {:?}", total);
                return (StatusCode::OK, Json(total));
            }
            Err(error) => {
                println!("error is {}", error);
                return (StatusCode::OK, Json(Vec::default()));
            }
        }
    }

    match sqlx::query_as::<_, TopListGiftsByREgion>(
        " SELECT region, array_remove(array_agg(gift_name), NULL) as top_gifts FROM (
         SELECT regions.name as region, gift_name, row_number() OVER (PARTITION BY regions.name order by regions.name ASC, SUM(quantity) DESC, gift_name ASC) as row_num
         FROM regions LEFT OUTER JOIN orders ON regions.id = orders.region_id
         GROUP BY regions.name, gift_name
         ORDER BY regions.name ASC, SUM(quantity) DESC, gift_name ASC
       ) as deep
     where row_num <= $1
     group by region",
    )
    .bind(&top_no_gifts)
    .fetch_all(&state.pool)
    .await
    {
        Ok(total) => {
            // println!("regions {:?}", total);
            (StatusCode::OK, Json(total))
        }
        Err(error) => {
            println!("error is {}", error);
            (StatusCode::OK, Json(Vec::default()))
        }
    }
}

pub async fn reset_day_18(State(state): State<Arc<AppState>>) -> StatusCode {
    sqlx::query("DELETE FROM orders")
        .execute(&state.pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM regions")
        .execute(&state.pool)
        .await
        .unwrap();

    StatusCode::OK
}
