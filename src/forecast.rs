use alloc::string::String;
use alloc::vec::Vec;

use crate::hourly::Hourly;
use crate::parse::{RawForecast, RawHourlyForecast, RawTimePeriod};
use crate::{Coordinates, Error, Metres, TimePeriod};

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

impl core::str::FromStr for Forecast<Hourly> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rf: RawForecast<RawHourlyForecast> = serde_json::from_str(s)?;
        Forecast::try_from(rf)
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
