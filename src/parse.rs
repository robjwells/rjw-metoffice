#![allow(dead_code)]
use std::error::Error;
use std::fmt::{Alignment, Display};
use std::io::Read;

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};

pub(crate) fn site_forecast_from_reader(rdr: impl Read) -> Result<SiteForecast, Box<dyn Error>> {
    let mut parsed: ApiResponse = serde_json::from_reader(rdr)?;
    let feature = parsed.features.remove(0);
    Ok(feature.into())
}

/// Forecast data for a particular weather station site.
#[derive(Debug)]
pub struct SiteForecast {
    /// Name of the weather station.
    pub name: String,
    /// Position of the weather station.
    pub coordinates: Coordinates,
    /// When the forecast model was run.
    pub forecast_made_at: DateTime<Utc>,
    /// Available hourly forecasts for the site.
    pub time_series: Vec<HourlyForecast>,
}

impl From<ApiFeature> for SiteForecast {
    /// Extracts relevant site forecast data from an API Feature object.
    fn from(value: ApiFeature) -> Self {
        let ApiFeature {
            geometry,
            properties,
        } = value;

        SiteForecast {
            name: properties.location.name,
            coordinates: geometry.coordinates,
            forecast_made_at: properties.model_run_date,
            time_series: properties.time_series,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    features: Vec<ApiFeature>,
}

#[derive(Debug, Deserialize)]
struct ApiFeature {
    geometry: FeatureGeometry,
    properties: FeatureProperties,
}

#[derive(Debug, Deserialize)]
struct FeatureGeometry {
    coordinates: Coordinates,
}

#[derive(Debug, Deserialize)]
pub struct DecimalDegrees(f64);

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    pub longitude: DecimalDegrees,
    pub latitude: DecimalDegrees,
    pub altitude: Metres,
}

#[derive(Debug, Deserialize)]
struct FeatureLocation {
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FeatureProperties {
    location: FeatureLocation,
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
pub struct Celsius(pub f32);

impl Display for Celsius {
    /// Formats the value with the given formatter.
    ///
    /// The precision and any `+` sign specifier are used to format the
    /// floating-point value, while the width and alignment are applied
    /// to the formatted "x.y°C" string.
    ///
    /// Other format specifiers are currently ignored.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.0;
        let precision = f.precision().unwrap_or(0);

        let formatted_value = if f.sign_plus() {
            format!("{value:+.precision$}°C")
        } else {
            format!("{value:.precision$}°C")
        };

        let width = f.width().unwrap_or(0);
        match f.align().unwrap_or(Alignment::Left) {
            Alignment::Left => write!(f, "{:<width$}", formatted_value),
            Alignment::Right => write!(f, "{:>width$}", formatted_value),
            Alignment::Center => write!(f, "{:^width$}", formatted_value),
        }
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

/// Forecast data for a given site and time.
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

/// Discriminator for weather conditions that can occur during the day or night.
#[derive(Debug)]
pub enum DayOrNight {
    Night,
    Day,
}

/// The prevailing conditions for a forecast.
///
/// This is the "significant weather code" as the Met Office refers to it.
/// In this representation conditions that differ only by taking place at
/// night or during the day are combined and carry a `DayOrNight` enum
/// variant to differentiate, should that be necessary.
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

impl Display for WeatherCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::WeatherCode::*;
        let description = match self {
            ClearNight => "Clear",
            SunnyDay => "Sunny",
            PartlyCloudy(_) => "Partly cloudy",
            NotUsed(_) => "Unknown",
            Mist => "Misty",
            Fog => "Foggy",
            Cloudy => "Cloudy",
            Overcast => "Overcast",
            LightRainShower(_) => "Light showers",
            Drizzle => "Drizzle",
            LightRain => "Light rain",
            HeavyRainShower(_) => "Heavy showers",
            HeavyRain => "Heavy rain",
            SleetShower(_) => "Sleet showers",
            Sleet => "Sleet",
            HailShower(_) => "Hail showers",
            Hail => "Hail",
            LightSnowShower(_) => "Light snow showers",
            LightSnow => "Light snow",
            HeavySnowShower(_) => "Heavy snow showers",
            HeavySnow => "Heavy snow",
            ThunderShower(_) => "Thundery showers",
            Thunder => "Thundery",
        };
        description.fmt(f)
    }
}

impl From<u8> for WeatherCode {
    /// Convert from the "significant weather code" number.
    ///
    /// Note that (presently) invalid codes, 31 and above, are accepted
    /// but wrapped in the `NotUsed` variant (which otherwise is only
    /// used for a code of 4 by the Met Office).
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
