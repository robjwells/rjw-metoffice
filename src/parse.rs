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
    pub predictions_made_at: jiff::Zoned,
    pub predictions: Vec<HourlyForecast>,
}

impl std::str::FromStr for Forecast {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
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

/// Forecast temperature values.
#[derive(Debug)]
#[allow(unused)]
pub struct TemperatureForecast {
    /// Temperature at screen level.
    ///
    /// Stevenson screen height is approximately 1.5m above ground level.
    screen: Celsius,
    /// The temperature it feels like, taking into account humidity and wind chill but
    /// not radiation.
    feels_like: Celsius,
    /// Dew point temperature at screen level.
    ///
    /// Stevenson screen height is approximately 1.5m above ground level.
    screen_dew_point: Celsius,
    /// Maximum air temperature at screen level.
    ///
    /// Appears to be missing after 48 hours.
    screen_max: Option<Celsius>,
    /// Minimum air temperature at screen level.
    ///
    /// Appears to be missing after 48 hours.
    screen_min: Option<Celsius>,
}

/// Forecast wind values.
#[derive(Debug)]
#[allow(unused)]
pub struct WindForecast {
    /// Surface wind speed in metres per second.
    ///
    /// Mean wind speed is equivalent to the mean speed observed over the 10 minutes preceding the
    /// validity time. Measured at 10 metres above ground, this is considered surface wind speed.
    speed: MetresPerSecond,
    /// Direction from which the wind is blowing in degrees.
    ///
    /// Mean wind direction is equivalent to the mean direction observed over the 10 minutes
    /// preceding the validity time. Measured at 10 metres above ground, this is considered surface
    /// wind direction.
    direction: Degrees,
    /// Maximum 3-second mean wind speed observed over the 10 minutes preciding the validity time.
    gust_speed: MetresPerSecond,
    /// Maximum 3-second mean wind speed observed over the hour preciding the validity time.
    ///
    /// Appears to be missing after 48 hours.
    max_hourly_gust_speed: Option<MetresPerSecond>,
}

/// Forecast precipitation values.
#[derive(Debug)]
#[allow(unused)]
pub struct PrecipitationForecast {
    /// Probability of precipitation over the hour centered at the validity time.
    probability: Percentage,
    /// Rate at which liquid water is being deposited on the surface, in mm per hour.
    rate: MillimetresPerHour,
    /// Implied depth of the layer of liquid water which has been deposited on the
    /// surface since the previous hour.
    ///
    /// Appears to be missing after 48 hours.
    total_precip_amount: Option<Millimetres>,
    /// Amount of snow that has fallen out of the sky in the last hour.
    ///
    /// This does not reflect snow lying on the ground. Falling snow may not settle at all and may
    /// be accompanied by rain (ie is sleet). Falling snow is stated as liquid water equivalent in
    /// mm, which can be considered approximately the same as cm of fresh snow or as a kilogram per
    /// square metre.
    ///
    /// Appears to be missing after 48 hours.
    total_snow_amount: Option<Millimetres>,
}

#[derive(Debug)]
#[allow(unused)]
pub struct HourlyForecast {
    /// Time at which this forecast is valid.
    time: jiff::Zoned,
    /// The most significant weather conditions at this time, taking into account both
    /// instantaneous and preceding conditions.
    conditions: Conditions,
    /// Temperature predictions.
    temperature: TemperatureForecast,
    /// Precipitation predictions (rain, snow, etc).
    precipitation: PrecipitationForecast,
    /// Wind speed and direction predictions.
    wind: WindForecast,
    /// Distance in metres at which a known object can be seen horizontally from screen level (1.5m.)
    visibility: Metres,
    /// Percent relative humidity at screen level (1.5m).
    relative_humidity: Percentage,
    /// Air pressure at mean sea level in Pascals.
    pressure: Pascals,
    /// Maxmium UV value over the hour preceding the validity time. Usually a value 0 to 13 but
    /// higher values are possible in extreme situations.
    uv_index: UvIndex,
}

impl TryFrom<RawHourlyForecast> for HourlyForecast {
    type Error = Error;

    fn try_from(rf: RawHourlyForecast) -> Result<Self, Self::Error> {
        Ok(Self {
            time: rf.time,
            conditions: rf.significant_weather_code.try_into()?,
            temperature: TemperatureForecast {
                screen: Celsius(rf.screen_temperature),
                feels_like: Celsius(rf.feels_like_temperature),
                screen_dew_point: Celsius(rf.screen_dew_point_temperature),
                screen_max: rf.max_screen_air_temp.map(Celsius),
                screen_min: rf.min_screen_air_temp.map(Celsius),
            },
            precipitation: PrecipitationForecast {
                probability: Percentage(rf.prob_of_precipitation),
                rate: MillimetresPerHour(rf.precipitation_rate),
                total_precip_amount: rf.total_precip_amount.map(Millimetres),
                total_snow_amount: rf.total_snow_amount.map(Millimetres),
            },
            wind: WindForecast {
                speed: MetresPerSecond(rf.wind_speed_10m),
                direction: Degrees(rf.wind_direction_from_10m),
                gust_speed: MetresPerSecond(rf.wind_gust_speed_10m),
                max_hourly_gust_speed: rf.max_10m_wind_gust.map(MetresPerSecond),
            },
            visibility: Metres(rf.visibility),
            relative_humidity: Percentage(rf.screen_relative_humidity),
            pressure: Pascals(rf.mslp),
            uv_index: UvIndex(rf.uv_index),
        })
    }
}

fn parse(s: &str) -> Result<Forecast, Error> {
    let rf: RawForecast = serde_json::from_str(s)?;
    Forecast::try_from(rf)
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
