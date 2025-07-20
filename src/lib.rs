mod parse;

use std::error::Error;

// use parse::site_forecast_from_reader;
pub use parse::{Coordinates, HourlyForecast, SiteForecast};

pub struct ApiKey {
    /// DataHub API key ID.
    pub id: String,
    /// DataHub API key secret.
    pub secret: String,
}

pub fn fetch_hourly_forecasts(
    _key: &ApiKey,
    _latitude: f64,
    _longitude: f64,
) -> Result<SiteForecast, Box<dyn Error>> {
    Err("The service this crate relied on is no longer provided.".into())
}
