mod parse;

use std::error::Error;

pub use parse::{site_forecast_from_reader, Coordinates, HourlyForecast, SiteForecast};

pub struct ApiKey {
    /// DataHub API key ID.
    pub id: String,
    /// DataHub API key secret.
    pub secret: String,
}

pub fn fetch_hourly_forecasts(
    key: &ApiKey,
    latitude: f64,
    longitude: f64,
) -> Result<SiteForecast, Box<dyn Error>> {
    let lat = format!("{latitude}");
    let lon = format!("{longitude}");
    let query_params = [
        ("includeLocationName", "true"),
        ("excludeParameterMetadata", "true"),
        ("latitude", &lat),
        ("longitude", &lon),
    ];
    let client = reqwest::blocking::Client::new()
        .get("https://api-metoffice.apiconnect.ibmcloud.com/v0/forecasts/point/hourly")
        .query(&query_params)
        .header("X-IBM-Client-Id", &key.id)
        .header("X-IBM-Client-Secret", &key.secret);

    let response = client.send()?;
    if cfg!(debug_assertions) {
        let ratelimit_header = response.headers().get("x-ratelimit-remaining");
        debug_assert!(ratelimit_header.is_some());
        let remaining = ratelimit_header
            .unwrap()
            .to_str()
            .unwrap()
            .split(',')
            .nth(1)
            .unwrap()
            .replace(';', "");
        eprintln!(
            "DEBUG :: API :: Remaining Met Office API calls :: {}",
            remaining
        );
    }

    let body = response.text()?;
    let parsed = site_forecast_from_reader(body.as_bytes()).unwrap();
    Ok(parsed)
}
