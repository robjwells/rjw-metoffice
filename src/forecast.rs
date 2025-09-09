use alloc::string::String;
use alloc::vec::Vec;

use crate::hourly::Hourly;
use crate::parse::{
    RawDailyForecast, RawForecast, RawHourlyForecast, RawThreeHourlyForecast, RawTimePeriod,
};
use crate::{Coordinates, Daily, Error, Metres, ThreeHourly, TimePeriod};

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
