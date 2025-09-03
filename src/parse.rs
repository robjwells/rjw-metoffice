use std::str::FromStr;

use serde::Deserialize;

use crate::units::Metres;
use crate::{
    Celsius, Conditions, Degrees, Error, MetresPerSecond, Millimetres, MillimetresPerHour, Pascals,
    Percentage, UvIndex,
};

#[derive(Debug)]
pub struct Forecast {
    /// Weather station location in the WGS 84 geographic coordinate reference system.
    pub coordinates: Coordinates,
    /// Weather station distance from the requested location.
    pub request_point_distance: Metres,
    /// Time at which the weather model was run.
    pub predictions_made_at: jiff::Zoned,
    /// Hourly forecast predictions.
    pub predictions: Vec<HourlyForecast>,
}

impl std::str::FromStr for Forecast {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rf: RawForecast = serde_json::from_str(s)?;
        Forecast::try_from(rf)
    }
}

impl TryFrom<RawForecast> for Forecast {
    type Error = Error;

    fn try_from(mut value: RawForecast) -> Result<Self, Self::Error> {
        let feature = value.features.remove(0);
        Ok(Forecast {
            coordinates: feature.geometry.coordinates,
            request_point_distance: Metres(feature.properties.request_point_distance),
            predictions_made_at: feature.properties.model_run_date,
            predictions: feature
                .properties
                .time_series
                .into_iter()
                .map(HourlyForecast::try_from)
                .collect::<Result<Vec<HourlyForecast>, Error>>()?,
        })
    }
}

#[derive(Debug)]
pub struct HourlyForecast {
    /// Time at which this forecast is valid.
    pub time: jiff::Zoned,
    /// The most significant weather conditions at this time, taking into account both
    /// instantaneous and preceding conditions.
    pub conditions: Conditions,
    /// Temperature at screen level.
    ///
    /// Stevenson screen height is approximately 1.5m above ground level.
    pub screen_temperature: Celsius,
    /// Maximum air temperature at screen level.
    ///
    /// Appears to be missing after 48 hours.
    pub screen_maximum_temperature: Option<Celsius>,
    /// Minimum air temperature at screen level.
    ///
    /// Appears to be missing after 48 hours.
    pub screen_mininium_temperature: Option<Celsius>,
    /// The temperature it feels like, taking into account humidity and wind chill but
    /// not radiation.
    pub feels_like_temperature: Celsius,
    /// Dew point temperature at screen level.
    ///
    /// Stevenson screen height is approximately 1.5m above ground level.
    pub screen_dew_point_temperature: Celsius,
    /// Probability of precipitation over the hour centered at the validity time.
    pub precipitation_probability: Percentage,
    /// Rate at which liquid water is being deposited on the surface, in mm per hour.
    pub precipitation_rate: MillimetresPerHour,
    /// Implied depth of the layer of liquid water which has been deposited on the
    /// surface since the previous hour.
    ///
    /// Appears to be missing after 48 hours.
    pub precipitation_total: Option<Millimetres>,
    /// Amount of snow that has fallen out of the sky in the last hour.
    ///
    /// This does not reflect snow lying on the ground. Falling snow may not settle at all and may
    /// be accompanied by rain (ie is sleet). Falling snow is stated as liquid water equivalent in
    /// mm, which can be considered approximately the same as cm of fresh snow or as a kilogram per
    /// square metre.
    ///
    /// Appears to be missing after 48 hours.
    pub snow_total: Option<Millimetres>,
    /// Surface wind speed in metres per second.
    ///
    /// Mean wind speed is equivalent to the mean speed observed over the 10 minutes preceding the
    /// validity time. Measured at 10 metres above ground, this is considered surface wind speed.
    pub wind_speed: MetresPerSecond,
    /// Direction from which the wind is blowing in degrees.
    ///
    /// Mean wind direction is equivalent to the mean direction observed over the 10 minutes
    /// preceding the validity time. Measured at 10 metres above ground, this is considered surface
    /// wind direction.
    pub wind_direction: Degrees,
    /// Maximum 3-second mean wind speed observed over the 10 minutes preciding the validity time.
    pub gust_speed: MetresPerSecond,
    /// Maximum 3-second mean wind speed observed over the hour preciding the validity time.
    ///
    /// Appears to be missing after 48 hours.
    pub gust_hourly_maximum_speed: Option<MetresPerSecond>,
    /// Distance in metres at which a known object can be seen horizontally from screen level (1.5m.)
    pub visibility: Metres,
    /// Percent relative humidity at screen level (1.5m).
    pub relative_humidity: Percentage,
    /// Air pressure at mean sea level in Pascals.
    pub pressure: Pascals,
    /// Maxmium UV value over the hour preceding the validity time. Usually a value 0 to 13 but
    /// higher values are possible in extreme situations.
    pub uv_index: UvIndex,
}

impl TryFrom<RawHourlyForecast> for HourlyForecast {
    type Error = Error;

    fn try_from(rf: RawHourlyForecast) -> Result<Self, Self::Error> {
        Ok(Self {
            time: rf.time,
            conditions: rf.significant_weather_code.try_into()?,
            screen_temperature: Celsius(rf.screen_temperature),
            feels_like_temperature: Celsius(rf.feels_like_temperature),
            screen_dew_point_temperature: Celsius(rf.screen_dew_point_temperature),
            screen_maximum_temperature: rf.max_screen_air_temp.map(Celsius),
            screen_mininium_temperature: rf.min_screen_air_temp.map(Celsius),
            precipitation_probability: Percentage(rf.prob_of_precipitation),
            precipitation_rate: MillimetresPerHour(rf.precipitation_rate),
            precipitation_total: rf.total_precip_amount.map(Millimetres),
            snow_total: rf.total_snow_amount.map(Millimetres),
            wind_speed: MetresPerSecond(rf.wind_speed_10m),
            wind_direction: Degrees(rf.wind_direction_from_10m),
            gust_speed: MetresPerSecond(rf.wind_gust_speed_10m),
            gust_hourly_maximum_speed: rf.max_10m_wind_gust.map(MetresPerSecond),
            visibility: Metres(rf.visibility),
            relative_humidity: Percentage(rf.screen_relative_humidity),
            pressure: Pascals(rf.mslp),
            uv_index: UvIndex(rf.uv_index),
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawForecast {
    features: Vec<RawFeature>,
}

#[derive(Debug, Deserialize)]
struct RawFeature {
    geometry: Geometry,
    properties: Properties,
}

#[derive(Debug, Deserialize)]
struct Geometry {
    coordinates: Coordinates,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Properties {
    request_point_distance: f32,
    #[serde(deserialize_with = "utc_minutes")]
    model_run_date: jiff::Zoned,
    time_series: Vec<RawHourlyForecast>,
}

fn utc_minutes<'de, D>(d: D) -> Result<jiff::Zoned, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = <&str as Deserialize>::deserialize(d)?;
    jiff::Timestamp::from_str(s)
        .map(|ts| ts.to_zoned(jiff::tz::TimeZone::UTC))
        .map_err(|_| serde::de::Error::custom("Failed to parse datetime."))
}

#[allow(unused)]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(try_from = "[f64; 3]")]
/// Coordinates in the WGS 84 coordinate reference system.
pub struct Coordinates {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

impl TryFrom<[f64; 3]> for Coordinates {
    type Error = Error;

    fn try_from(value: [f64; 3]) -> Result<Self, Self::Error> {
        if let [
            longitude @ -180.0..=180.0,
            latitude @ -90.0..=90.0,
            altitude,
        ] = value
        {
            Ok(Self {
                latitude,
                longitude,
                altitude,
            })
        } else {
            Err(Error::CoordinatesOutOfBounds)
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct RawHourlyForecast {
    /// Time at which this forecast is valid.
    #[serde(deserialize_with = "utc_minutes")]
    time: jiff::Zoned,
    /// Temperature at screen level.
    ///
    /// Stevenson screen height is approximately 1.5m above ground level.
    screen_temperature: f32,
    /// Maximum air temperature at screen level.
    ///
    /// Appears to be missing after 48 hours.
    max_screen_air_temp: Option<f32>,
    /// Minimum air temperature at screen level.
    ///
    /// Appears to be missing after 48 hours.
    min_screen_air_temp: Option<f32>,
    /// Dew point temperature at screen level.
    ///
    /// Stevenson screen height is approximately 1.5m above ground level.
    screen_dew_point_temperature: f32,
    /// The temperature it feels like, taking into account humidity and wind chill but
    /// not radiation.
    feels_like_temperature: f32,
    /// Surface wind speed in metres per second.
    ///
    /// Mean wind speed is equivalent to the mean speed observed over the 10 minutes preceding the
    /// validity time. Measured at 10 metres above ground, this is considered surface wind speed.
    wind_speed_10m: f32,
    /// Direction from which the wind is blowing in degrees.
    ///
    /// Mean wind direction is equivalent to the mean direction observed over the 10 minutes
    /// preceding the validity time. Measured at 10 metres above ground, this is considered surface
    /// wind direction.
    wind_direction_from_10m: f32,
    /// Maximum 3-second mean wind speed observed over the 10 minutes preciding the validity time.
    wind_gust_speed_10m: f32,
    /// Maximum 3-second mean wind speed observed over the hour preciding the validity time.
    ///
    /// Appears to be missing after 48 hours.
    max_10m_wind_gust: Option<f32>,
    /// Distance in metres at which a known object can be seen horizontally from screen level (1.5m.)
    visibility: f32,
    /// Percent relative humidity at screen level (1.5m).
    screen_relative_humidity: f32,
    /// Air pressure at mean sea level in Pascals.
    mslp: u32,
    /// Maxmium UV value over the hour preceding the validity time. Usually a value 0 to 13 but
    /// higher values are possible in extreme situations.
    uv_index: u8,
    /// The most significant weather conditions at this time, taking into account both
    /// instantaneous and preceding conditions.
    ///
    /// This number is really an enum discriminant.
    significant_weather_code: i8,
    /// Rate at which liquid water is being deposited on the surface, in mm per hour.
    precipitation_rate: f32,
    /// Implied depth of the layer of liquid water which has been deposited on the
    /// surface since the previous hour.
    ///
    /// Appears to be missing after 48 hours.
    total_precip_amount: Option<f32>,
    /// Amount of snow that has fallen out of the sky in the last hour.
    ///
    /// This does not reflect snow lying on the ground. Falling snow may not settle at all and may
    /// be accompanied by rain (ie is sleet). Falling snow is stated as liquid water equivalent in
    /// mm, which can be considered approximately the same as cm of fresh snow or as a kilogram per
    /// square metre.
    ///
    /// Appears to be missing after 48 hours.
    total_snow_amount: Option<f32>,
    /// Probability of precipitation over the hour centered at the validity time.
    prob_of_precipitation: f32,
}

#[cfg(test)]
mod test {
    use super::Coordinates;

    #[test]
    fn coordinates_only_in_bounds() {
        let oob = [
            [-180.1, 0.0, 0.0],
            [180.1, 0.0, 0.0],
            [0.0, 90.1, 0.0],
            [0.0, -90.1, 0.0],
        ];
        for coords in oob {
            assert!(Coordinates::try_from(coords).is_err())
        }
    }
}
