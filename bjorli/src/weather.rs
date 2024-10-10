use serde::{Deserialize, Serialize};
use reqwest::Client;

// Representerer responsen som kommer fra open-meteo api'et
#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub hourly: HourlyData,
}

// Representerer timedataen
#[derive(Debug, Deserialize)]
pub struct HourlyData {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
}

// Representerer dataen etter den har blitt serialisert
#[derive(Debug, Serialize)]
pub struct WeatherData {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub hourly_time: Vec<String>,
    pub hourly_temperature: Vec<f64>,
}

impl WeatherData {
    // funksjonen som konverterer fra WeatherResponse til WeatherData
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

// Funksjonen som henter data fra open-meteo og konverterer den til WeatherData
pub async fn fetch_weather_data(latitude: f64, longitude: f64) -> Result<WeatherData, String> {
    let client = Client::new();
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m",
        latitude, longitude
    );

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|err| format!("Request error: {}", err))?;

    if !resp.status().is_success() {
        return Err(format!("Failed to fetch data: HTTP {}", resp.status()));
    }

    let weather_response: WeatherResponse = resp
        .json()
        .await
        .map_err(|err| format!("JSON parse error: {}", err))?;

    Ok(WeatherData::from_response(weather_response))
}
