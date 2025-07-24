// Your data models for sensor data (like Temperature, Moisture, etc.)
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SensorData {
    pub temperature: f32,
    pub moisture: f32,
    pub light: f32,
    pub timestamp: String,
}
