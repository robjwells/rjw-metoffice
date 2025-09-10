use alloc::string::{String, ToString};
use alloc::vec::Vec;
use url::Url;

use crate::hourly::Hourly;
use crate::parse::{
    RawDailyForecast, RawForecast, RawHourlyForecast, RawThreeHourlyForecast, RawTimePeriod,
};
use crate::{Coordinates, Daily, Error, Latitude, Longitude, Metres, ThreeHourly, TimePeriod};

#[derive(Debug)]
pub struct Forecast<T>
where
    T: TimePeriod,
{
    /// Forecast location name.
    pub location_name: String,
    /// Weather station location in the WGS 84 geographic coordinate reference system.
    pub coordinates: Coordinates,
    /// Weather station distance from the requested location.
    pub requested_point_distance: Metres,
    /// Time at which the weather model was run.
    pub predictions_made_at: jiff::Zoned,
    /// Forecast predictions.
    pub predictions: Vec<T>,
}

const HOURLY_URL: &str = "https://data.hub.api.metoffice.gov.uk/sitespecific/v0/point/hourly";
const THREE_HOURLY_URL: &str =
    "https://data.hub.api.metoffice.gov.uk/sitespecific/v0/point/three-hourly";
const DAILY_URL: &str = "https://data.hub.api.metoffice.gov.uk/sitespecific/v0/point/daily";
const SOURCE_PARAM: (&str, &str) = ("source", "BD1");
const METADATA_PARAM: (&str, &str) = ("excludeParameterMetadata", "true");
const LOCATION_NAME_PARAM: (&str, &str) = ("includeLocationName", "true");

impl<T: TimePeriod> Forecast<T> {
    fn url_with_params(url: &str, latitude: Latitude, longitude: Longitude) -> Url {
        Url::parse_with_params(
            url,
            &[
                ("latitude", latitude.as_float().to_string().as_str()),
                ("longitude", longitude.as_float().to_string().as_str()),
                SOURCE_PARAM,
                METADATA_PARAM,
                LOCATION_NAME_PARAM,
            ],
        )
        .expect("Bug in URL construction.")
    }
}

impl Forecast<Hourly> {
    pub fn url_for_location(latitude: Latitude, longitude: Longitude) -> Url {
        Self::url_with_params(HOURLY_URL, latitude, longitude)
    }
}

impl Forecast<ThreeHourly> {
    pub fn url_for_location(latitude: Latitude, longitude: Longitude) -> Url {
        Self::url_with_params(THREE_HOURLY_URL, latitude, longitude)
    }
}

impl Forecast<Daily> {
    pub fn url_for_location(latitude: Latitude, longitude: Longitude) -> Url {
        Self::url_with_params(DAILY_URL, latitude, longitude)
    }
}

impl<R> TryFrom<RawForecast<R>> for Forecast<R::Output>
where
    R: RawTimePeriod,
{
    type Error = Error;

    fn try_from(mut value: RawForecast<R>) -> Result<Self, Self::Error> {
        let feature = value.features.remove(0);
        Ok(Forecast {
            location_name: feature.properties.location.name,
            coordinates: feature.geometry.coordinates,
            requested_point_distance: Metres(feature.properties.request_point_distance),
            predictions_made_at: feature.properties.model_run_date,
            predictions: feature
                .properties
                .time_series
                .into_iter()
                .map(R::Output::try_from)
                .collect::<Result<Vec<R::Output>, Error>>()?,
        })
    }
}

impl<T: TimePeriod> Forecast<T> {
    fn try_from_str<'a, R>(s: &'a str) -> Result<Self, Error>
    where
        R: RawTimePeriod<Output = T> + serde::Deserialize<'a>,
    {
        serde_json::from_str::<RawForecast<R>>(s)
            .map_err(Error::Serde)
            .and_then(Forecast::try_from)
    }

    fn try_from_bytes<'a, R>(bytes: &'a [u8]) -> Result<Self, Error>
    where
        R: RawTimePeriod<Output = T> + serde::Deserialize<'a>,
    {
        serde_json::from_slice::<RawForecast<R>>(bytes)
            .map_err(Error::Serde)
            .and_then(Forecast::try_from)
    }
}

impl TryFrom<&[u8]> for Forecast<Hourly> {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Forecast::try_from_bytes::<RawHourlyForecast>(bytes)
    }
}

impl core::str::FromStr for Forecast<Hourly> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Forecast::try_from_str::<RawHourlyForecast>(s)
    }
}

impl TryFrom<&[u8]> for Forecast<ThreeHourly> {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Forecast::try_from_bytes::<RawThreeHourlyForecast>(bytes)
    }
}

impl core::str::FromStr for Forecast<ThreeHourly> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Forecast::try_from_str::<RawThreeHourlyForecast>(s)
    }
}

impl TryFrom<&[u8]> for Forecast<Daily> {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from_bytes::<RawDailyForecast>(bytes)
    }
}

impl core::str::FromStr for Forecast<Daily> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str::<RawDailyForecast>(s)
    }
}
