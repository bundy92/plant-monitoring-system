// Your SQLite database functions for saving and retrieving sensor data
use sqlx::sqlite::SqlitePool;

pub async fn get_db_pool() -> SqlitePool {
    SqlitePool::connect("sqlite://plant_data.db")
        .await
        .unwrap()
}

pub async fn save_sensor_data(db: &SqlitePool, data: &SensorData) {
    sqlx::query!(
        "INSERT INTO sensor_data (temperature, moisture, light, timestamp) VALUES (?, ?, ?, ?)",
        data.temperature, data.moisture, data.light, data.timestamp
    )
    .execute(db)
    .await
    .unwrap();
}
