#![allow(dead_code)]
use std::{fmt::Display, io::Read, error::Error};

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};

pub fn site_forecast_from_reader(rdr: impl Read) -> Result<SiteForecast, Box<dyn Error>> {
    let mut parsed: Response = serde_json::from_reader(rdr)?;
    let feature = parsed.features.remove(0);
    Ok(feature.into())
}

#[derive(Debug)]
pub struct SiteForecast {
    pub name: String,
    pub coordinates: Coordinates,
    pub forecast_made_at: DateTime<Utc>,
    pub time_series: Vec<HourlyForecast>,
}

impl From<Feature> for SiteForecast {
    fn from(value: Feature) -> Self {
        let Feature { geometry, properties } = value;

        SiteForecast {
            name: properties.location.name,
            coordinates: geometry.coordinates,
            forecast_made_at: properties.model_run_date,
            time_series: properties.time_series,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    features: Vec<Feature>,
}

#[derive(Debug, Deserialize)]
struct Feature {
    geometry: Geometry,
    properties: Properties,
}

#[derive(Debug, Deserialize)]
struct Geometry {
    coordinates: Coordinates,
}

#[derive(Debug, Deserialize)]
pub struct DecimalDegrees(f64);

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    pub longitude: DecimalDegrees,
    pub latitude: DecimalDegrees,
    pub height_above_sea_level: Metres,
}

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Properties {
    location: Location,
    request_point_distance: Metres,
    #[serde(deserialize_with = "deserialize_datetime_without_seconds")]
    model_run_date: DateTime<Utc>, // ISO date
    time_series: Vec<HourlyForecast>,
}

#[derive(Debug, Deserialize)]
pub struct Millimetres(f32);

impl Display for Millimetres {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mm", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct MillimetresPerHour(f32);

impl Display for MillimetresPerHour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mm/h", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct Metres(f32);

impl Display for Metres {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct MetresPerSecond(f32);

impl Display for MetresPerSecond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m/s", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct Celsius(f32);

impl Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}°C", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct Percentage(f32);

impl Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct Pascals(f32);

impl Display for Pascals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Pa", self.0)
    }
}

#[derive(Debug)]
pub enum UvIndex {
    None,
    Low(u8),
    Moderate(u8),
    High(u8),
    VeryHigh(u8),
    Extreme(u8),
}

impl From<u8> for UvIndex {
    fn from(value: u8) -> Self {
        use self::UvIndex::*;
        match value {
            0 => None,
            1..=2 => Low(value),
            3..=5 => Moderate(value),
            6..=7 => High(value),
            8..=10 => VeryHigh(value),
            11.. => Extreme(value),
        }
    }
}

impl<'de> Deserialize<'de> for UvIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Ok(value.into())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyForecast {
    #[serde(deserialize_with = "deserialize_datetime_without_seconds")]
    pub time: DateTime<Utc>, // ISO datetime hour
    /// Screen air temperature (°C)
    pub screen_temperature: Celsius,
    /// Maximum screen air temperature over previous hour (°C)
    pub max_screen_air_temp: Option<Celsius>,
    /// Minimum screen air temperature over previous hour (°C)
    pub min_screen_air_temp: Option<Celsius>,
    /// Screen dew point temperature (°C)
    pub screen_dew_point_temperature: Celsius,
    /// Feels-like temperature (°C)
    pub feels_like_temperature: Celsius,
    /// 10m wind speed (in m/s)
    pub wind_speed_10m: MetresPerSecond,
    /// 10m wind from direction
    pub wind_direction_from_10m: MetresPerSecond,
    /// 10m wind gust speed (m/s)
    pub wind_gust_speed_10m: MetresPerSecond,
    /// Maximum 10m wind gust speed over previous hour (m/s)
    pub max_10m_wind_gust: Option<MetresPerSecond>,
    /// Visibility (in metres)
    pub visibility: Metres,
    /// Screen relative humidity (%)
    pub screen_relative_humidity: Percentage,
    /// Mean sea level pressure (Pa)
    pub mslp: Pascals,
    /// UV index
    pub uv_index: UvIndex,
    /// Significant weather code
    pub significant_weather_code: WeatherCode,
    /// Precipitation rate (in mm per hour)
    pub precipitation_rate: MillimetresPerHour,
    /// Total precipitation amount over prevous hour (in mm)
    pub total_precip_amount: Option<Millimetres>,
    /// Total snow amount over previous hour (in mm)
    pub total_snow_amount: Option<Millimetres>,
    /// Probability of precipitation (%)
    pub prob_of_precipitation: Percentage,
}

#[derive(Debug)]
pub enum DayOrNight {
    Night,
    Day,
}

#[derive(Debug)]
pub enum WeatherCode {
    ClearNight,
    SunnyDay,
    PartlyCloudy(DayOrNight),
    NotUsed(u8),
    Mist,
    Fog,
    Cloudy,
    Overcast,
    LightRainShower(DayOrNight),
    Drizzle,
    LightRain,
    HeavyRainShower(DayOrNight),
    HeavyRain,
    SleetShower(DayOrNight),
    Sleet,
    HailShower(DayOrNight),
    Hail,
    LightSnowShower(DayOrNight),
    LightSnow,
    HeavySnowShower(DayOrNight),
    HeavySnow,
    ThunderShower(DayOrNight),
    Thunder,
}

impl From<u8> for WeatherCode {
    fn from(value: u8) -> Self {
        use self::DayOrNight::*;
        use self::WeatherCode::*;
        match value {
            0 => ClearNight,
            1 => SunnyDay,
            2 => PartlyCloudy(Night),
            3 => PartlyCloudy(Day),
            4 => NotUsed(4),
            5 => Mist,
            6 => Fog,
            7 => Cloudy,
            8 => Overcast,
            9 => LightRainShower(Night),
            10 => LightRainShower(Day),
            11 => Drizzle,
            12 => LightRain,
            13 => HeavyRainShower(Night),
            14 => HeavyRainShower(Day),
            15 => HeavyRain,
            16 => SleetShower(Night),
            17 => SleetShower(Day),
            18 => Sleet,
            19 => HailShower(Night),
            20 => HailShower(Day),
            21 => Hail,
            22 => LightSnowShower(Night),
            23 => LightSnowShower(Day),
            24 => LightSnow,
            25 => HeavySnowShower(Night),
            26 => HeavySnowShower(Day),
            27 => HeavySnow,
            28 => ThunderShower(Night),
            29 => ThunderShower(Day),
            30 => Thunder,
            code => NotUsed(code),
        }
    }
}

impl<'de> Deserialize<'de> for WeatherCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Ok(value.into())
    }
}

fn deserialize_datetime_without_seconds<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    const FORMAT: &str = "%Y-%m-%dT%H:%MZ";
    let value = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&value, FORMAT)
        .map_err(serde::de::Error::custom)
}
