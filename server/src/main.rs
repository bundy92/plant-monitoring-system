// Your Axum server setup, endpoints, and database interaction code go here
use axum::{Router, Json, routing::post};
use serde::{Serialize, Deserialize};
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct SensorData {
    temperature: f32,
    moisture: f32,
    light: f32,
    timestamp: String,
}

#[tokio::main]
async fn main() {
    let db = SqlitePool::connect("sqlite://plant_data.db").await.unwrap();

    let app = Router::new()
        .route("/api/data", post(move |Json(payload): Json<SensorData>| {
            let db = db.clone();
            async move {
                sqlx::query!(
                    "INSERT INTO sensor_data (temperature, moisture, light, timestamp) VALUES (?, ?, ?, ?)",
                    payload.temperature, payload.moisture, payload.light, payload.timestamp
                )
                .execute(&db)
                .await
                .unwrap();
                Json("Data saved")
            }
        }));

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
