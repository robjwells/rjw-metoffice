#![no_std]

// TODO: Document expected memory requirements for (separately) JSON text and forecast structs.
extern crate alloc;

mod error;
mod forecast;
mod hourly;
mod parse;
mod sealed;
mod three_hourly;
mod units;

use alloc::string::ToString;

use url::Url;

pub use error::Error;
pub use forecast::Forecast;
pub use hourly::Hourly;
pub use parse::Coordinates;
pub use sealed::TimePeriod;
pub use three_hourly::ThreeHourly;
pub use units::*;

const HOURLY_URL: &str = "https://data.hub.api.metoffice.gov.uk/sitespecific/v0/point/hourly";
const SOURCE_PARAM: (&str, &str) = ("source", "BD1");
const METADATA_PARAM: (&str, &str) = ("excludeParameterMetadata", "true");
const LOCATION_NAME_PARAM: (&str, &str) = ("includeLocationName", "true");

pub fn hourly_predictions_url_for_location(latitude: f64, longitude: f64) -> Url {
    Url::parse_with_params(
        HOURLY_URL,
        &[
            ("latitude", latitude.to_string().as_str()),
            ("longitude", longitude.to_string().as_str()),
            SOURCE_PARAM,
            METADATA_PARAM,
            LOCATION_NAME_PARAM,
        ],
    )
    .expect("Bug in hourly URL construction.")
}
