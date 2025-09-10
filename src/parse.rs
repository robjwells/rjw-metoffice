use alloc::str::FromStr;
use alloc::string::String;
use alloc::vec::Vec;
use serde::Deserialize;

use crate::units::Coordinates;
use crate::{Daily, Error, Hourly, ThreeHourly, TimePeriod};

pub(crate) trait RawTimePeriod: Sized {
    type Output: TimePeriod + TryFrom<Self, Error = Error>;
}

impl RawTimePeriod for RawHourlyForecast {
    type Output = Hourly;
}

impl RawTimePeriod for RawThreeHourlyForecast {
    type Output = ThreeHourly;
}

impl RawTimePeriod for RawDailyForecast {
    type Output = Daily;
}

#[derive(Debug, Deserialize)]
pub(crate) struct RawForecast<T>
where
    T: RawTimePeriod,
{
    pub features: Vec<RawFeature<T>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RawFeature<T> {
    pub geometry: Geometry,
    pub properties: Properties<T>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Geometry {
    pub coordinates: Coordinates,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Properties<T> {
    pub location: Location,
    pub request_point_distance: f32,
    #[serde(deserialize_with = "utc_minutes")]
    pub model_run_date: jiff::Zoned,
    pub time_series: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Location {
    pub name: String,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawThreeHourlyForecast {
    /// Time at which this forecast is valid.
    #[serde(deserialize_with = "utc_minutes")]
    pub time: jiff::Zoned,
    /// Maximum air temperature at screen level.
    pub max_screen_air_temp: f32,
    /// Minimum air temperature at screen level.
    pub min_screen_air_temp: f32,
    /// The temperature it feels like, taking into account humidity and wind chill but
    /// not radiation.
    pub feels_like_temp: f32,
    /// Surface wind speed in metres per second.
    pub wind_speed_10m: f32,
    /// Direction from which the wind is blowing in degrees.
    pub wind_direction_from_10m: f32,
    /// Maximum 3-second mean wind speed.
    pub wind_gust_speed_10m: f32,
    /// Maximum 3-second mean wind speed.
    pub max_10m_wind_gust: f32,
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
    /// Implied depth of the layer of liquid water which has been deposited on the
    /// surface since the previous hour.
    pub total_precip_amount: f32,
    /// Amount of snow that has fallen out of the sky in the last hour.
    ///
    /// This does not reflect snow lying on the ground. Falling snow may not settle at all and may
    /// be accompanied by rain (ie is sleet). Falling snow is stated as liquid water equivalent in
    /// mm, which can be considered approximately the same as cm of fresh snow or as a kilogram per
    /// square metre.
    pub total_snow_amount: f32,
    /// Probability of precipitation over the hour centered at the validity time.
    pub prob_of_precipitation: f32,
    pub prob_of_snow: f32,
    pub prob_of_heavy_snow: f32,
    pub prob_of_rain: f32,
    pub prob_of_heavy_rain: f32,
    pub prob_of_hail: f32,
    pub prob_of_sferics: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawDailyForecast {
    /// Time at which this forecast is valid.
    #[serde(deserialize_with = "utc_minutes")]
    pub time: jiff::Zoned,
    pub day_significant_weather_code: Option<i8>,
    pub day_max_screen_temperature: f32,
    pub day_upper_bound_max_temp: f32,
    pub day_lower_bound_max_temp: f32,
    pub day_max_feels_like_temp: Option<f32>,
    pub day_upper_bound_max_feels_like_temp: f32,
    pub day_lower_bound_max_feels_like_temp: f32,
    pub day_probability_of_precipitation: Option<f32>,
    pub day_probability_of_rain: Option<f32>,
    pub day_probability_of_heavy_rain: Option<f32>,
    pub day_probability_of_snow: Option<f32>,
    pub day_probability_of_heavy_snow: Option<f32>,
    pub day_probability_of_hail: Option<f32>,
    pub day_probability_of_sferics: Option<f32>,
    pub max_uv_index: Option<u8>,
    #[serde(rename = "midday10MWindSpeed")]
    pub midday_10m_wind_speed: f32,
    #[serde(rename = "midday10MWindDirection")]
    pub midday_10m_wind_direction: f32,
    #[serde(rename = "midday10MWindGust")]
    pub midday_10m_wind_gust: f32,
    pub midday_mslp: u32,
    pub midday_relative_humidity: f32,
    pub midday_visibility: f32,
    pub night_significant_weather_code: i8,
    pub night_min_screen_temperature: f32,
    pub night_upper_bound_min_temp: f32,
    pub night_lower_bound_min_temp: f32,
    pub night_min_feels_like_temp: f32,
    pub night_upper_bound_min_feels_like_temp: f32,
    pub night_lower_bound_min_feels_like_temp: f32,
    pub night_probability_of_precipitation: f32,
    pub night_probability_of_rain: f32,
    pub night_probability_of_heavy_rain: f32,
    pub night_probability_of_snow: f32,
    pub night_probability_of_heavy_snow: f32,
    pub night_probability_of_hail: f32,
    pub night_probability_of_sferics: f32,
    #[serde(rename = "midnight10MWindSpeed")]
    pub midnight_10m_wind_speed: f32,
    #[serde(rename = "midnight10MWindDirection")]
    pub midnight_10m_wind_direction: f32,
    #[serde(rename = "midnight10MWindGust")]
    pub midnight_10m_wind_gust: f32,
    pub midnight_mslp: u32,
    pub midnight_relative_humidity: f32,
    pub midnight_visibility: f32,
}
