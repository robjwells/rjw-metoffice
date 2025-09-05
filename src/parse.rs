use std::str::FromStr;

use serde::Deserialize;

use crate::Error;

#[derive(Debug, Deserialize)]
pub(crate) struct RawForecast {
    pub features: Vec<RawFeature>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RawFeature {
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Geometry {
    pub coordinates: Coordinates,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Properties {
    pub location: Location,
    pub request_point_distance: f32,
    #[serde(deserialize_with = "utc_minutes")]
    pub model_run_date: jiff::Zoned,
    pub time_series: Vec<RawHourlyForecast>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Location {
    pub name: String
}

fn utc_minutes<'de, D>(d: D) -> Result<jiff::Zoned, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    jiff::Timestamp::from_str(&s)
        .map(|ts| ts.to_zoned(jiff::tz::TimeZone::UTC))
        .map_err(|_| serde::de::Error::custom("Failed to parse datetime."))
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(try_from = "[f64; 3]")]
/// Coordinates in the WGS 84 coordinate reference system.
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
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
pub(crate) struct RawHourlyForecast {
    /// Time at which this forecast is valid.
    #[serde(deserialize_with = "utc_minutes")]
    pub time: jiff::Zoned,
    /// Temperature at screen level.
    ///
    /// Stevenson screen height is approximately 1.5m above ground level.
    pub screen_temperature: f32,
    /// Maximum air temperature at screen level.
    ///
    /// Appears to be missing after 48 hours.
    pub max_screen_air_temp: Option<f32>,
    /// Minimum air temperature at screen level.
    ///
    /// Appears to be missing after 48 hours.
    pub min_screen_air_temp: Option<f32>,
    /// Dew point temperature at screen level.
    ///
    /// Stevenson screen height is approximately 1.5m above ground level.
    pub screen_dew_point_temperature: f32,
    /// The temperature it feels like, taking into account humidity and wind chill but
    /// not radiation.
    pub feels_like_temperature: f32,
    /// Surface wind speed in metres per second.
    ///
    /// Mean wind speed is equivalent to the mean speed observed over the 10 minutes preceding the
    /// validity time. Measured at 10 metres above ground, this is considered surface wind speed.
    pub wind_speed_10m: f32,
    /// Direction from which the wind is blowing in degrees.
    ///
    /// Mean wind direction is equivalent to the mean direction observed over the 10 minutes
    /// preceding the validity time. Measured at 10 metres above ground, this is considered surface
    /// wind direction.
    pub wind_direction_from_10m: f32,
    /// Maximum 3-second mean wind speed observed over the 10 minutes preciding the validity time.
    pub wind_gust_speed_10m: f32,
    /// Maximum 3-second mean wind speed observed over the hour preciding the validity time.
    ///
    /// Appears to be missing after 48 hours.
    pub max_10m_wind_gust: Option<f32>,
    /// Distance in metres at which a known object can be seen horizontally from screen level (1.5m.)
    pub visibility: f32,
    /// Percent relative humidity at screen level (1.5m).
    pub screen_relative_humidity: f32,
    /// Air pressure at mean sea level in Pascals.
    pub mslp: u32,
    /// Maxmium UV value over the hour preceding the validity time. Usually a value 0 to 13 but
    /// higher values are possible in extreme situations.
    pub uv_index: u8,
    /// The most significant weather conditions at this time, taking into account both
    /// instantaneous and preceding conditions.
    ///
    /// This number is really an enum discriminant.
    pub significant_weather_code: i8,
    /// Rate at which liquid water is being deposited on the surface, in mm per hour.
    pub precipitation_rate: f32,
    /// Implied depth of the layer of liquid water which has been deposited on the
    /// surface since the previous hour.
    ///
    /// Appears to be missing after 48 hours.
    pub total_precip_amount: Option<f32>,
    /// Amount of snow that has fallen out of the sky in the last hour.
    ///
    /// This does not reflect snow lying on the ground. Falling snow may not settle at all and may
    /// be accompanied by rain (ie is sleet). Falling snow is stated as liquid water equivalent in
    /// mm, which can be considered approximately the same as cm of fresh snow or as a kilogram per
    /// square metre.
    ///
    /// Appears to be missing after 48 hours.
    pub total_snow_amount: Option<f32>,
    /// Probability of precipitation over the hour centered at the validity time.
    pub prob_of_precipitation: f32,
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
