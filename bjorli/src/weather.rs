use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use reqwest::Client;
use serde_wasm_bindgen::to_value; 

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub hourly: HourlyData,
}

#[derive(Debug, Deserialize)]
pub struct HourlyData {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
}

#[derive(Debug, Serialize)]
pub struct WeatherData {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub hourly_time: Vec<String>,
    pub hourly_temperature: Vec<f64>,
}

impl WeatherData {
    pub fn from_response(resp: WeatherResponse) -> Self {
        WeatherData {
            latitude: resp.latitude,
            longitude: resp.longitude,
            timezone: resp.timezone,
            hourly_time: resp.hourly.time,
            hourly_temperature: resp.hourly.temperature_2m,
        }
    }
}

// Public function exposed to JavaScript to fetch weather data using reqwest
#[wasm_bindgen]
pub async fn get_weather_data(latitude: f64, longitude: f64) -> Result<JsValue, JsValue> {
    // Initialize reqwest client
    let client = Client::new();

    // Construct the API URL
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m",
        latitude, longitude
    );

    // Perform the GET request
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|err| JsValue::from_str(&format!("Request error: {}", err)))?;

    // Check if the response status is success
    if !resp.status().is_success() {
        return Err(JsValue::from_str(&format!(
            "Failed to fetch data: HTTP {}",
            resp.status()
        )));
    }

    // Parse the JSON response
    let weather_response: WeatherResponse = resp
        .json()
        .await
        .map_err(|err| JsValue::from_str(&format!("JSON parse error: {}", err)))?;

    // Convert to WeatherData
    let weather_data = WeatherData::from_response(weather_response);
    // Serialize WeatherData to JsValue using serde-wasm-bindgen
    to_value(&weather_data)
        .map_err(|err| JsValue::from_str(&format!("Serialization error: {}", err)))
}

