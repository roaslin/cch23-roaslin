use axum::{http::StatusCode, extract::Path};

static MAX_PACKETS_IN_SLED: usize = 20;

pub async fn calculate_sled_id(Path(packets_ids): Path<String>) -> (StatusCode, String) {
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
