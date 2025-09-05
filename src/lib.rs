mod error;
mod forecast;
mod hourly;
mod parse;
mod units;

use std::io::Read;

use url::Url;

pub use error::Error;
pub use forecast::Forecast;
pub use parse::Coordinates;
pub use units::*;

use crate::parse::RawForecast;

const HOURLY_URL: &str = "https://data.hub.api.metoffice.gov.uk/sitespecific/v0/point/hourly";

pub fn hourly_predictions_from_reader(rdr: impl Read) -> Result<Forecast, Error> {
    serde_json::de::from_reader::<_, RawForecast>(rdr)
        .map_err(Error::Serde)
        .and_then(Forecast::try_from)
}

pub fn hourly_predictions_url_for_location(latitude: f64, longitude: f64) -> Url {
    Url::parse_with_params(
        HOURLY_URL,
        &[
            ("latitude", latitude.to_string().as_str()),
            ("longitude", longitude.to_string().as_str()),
            ("source", "BD1"),
            ("excludeParameterMetadata", "true"),
            ("includeLocationName", "true"),
        ],
    )
    .expect("Bug in hourly URL construction.")
}
