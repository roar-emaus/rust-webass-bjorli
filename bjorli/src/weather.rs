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

// Representerer daglige temperaturer
#[derive(Debug, Serialize, Deserialize)]
pub struct DailyTemperature {
    pub date: String,
    pub max_temperature: f64,
    pub min_temperature: f64,
}

// Wrapper for daglige temperaturer
#[derive(Debug, Serialize, Deserialize)]
pub struct DailyWeatherData {
    pub daily_temperatures: Vec<DailyTemperature>,
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
        // Beregner daglige maks og min temperaturer
    pub fn compute_daily_temperatures(&self) -> Vec<DailyTemperature> {
        let mut daily_map: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();

        for (time, temp) in self.hourly_time.iter().zip(self.hourly_temperature.iter()) {
            let date = match time.split('T').next() {
                Some(d) => d.to_string(),
                None => continue,
            };

            daily_map.entry(date).or_insert_with(Vec::new).push(*temp);
        }

        let mut daily_temperatures = Vec::new();

        for (date, temps) in daily_map {
            if let Some(&max_temp) = temps.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
                if let Some(&min_temp) = temps.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
                    daily_temperatures.push(DailyTemperature {
                        date,
                        max_temperature: max_temp,
                        min_temperature: min_temp,
                    });
                }
            }
        }

        daily_temperatures.sort_by(|a, b| a.date.cmp(&b.date));

        daily_temperatures
    }
}

// Funksjonen som henter data fra open-meteo og konverterer den til WeatherData
pub async fn fetch_weather_data(latitude: f64, longitude: f64) -> Result<WeatherData, String> {
    let client = Client::new();
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m&timezone=Europe%2FBerlin",
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
