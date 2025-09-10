//! Newtype wrappers for forecast units
use serde::Deserialize;

use crate::Error;

/// Latitude in decimal degrees in the WGS 84 reference system
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Latitude(f64);

impl Latitude {
    /// Construct a latitude from a float
    ///
    /// Returns an error if the given latitude is out of bounds (±90°).
    pub fn new(d: f64) -> Result<Self, Error> {
        if matches!(d, -90.0..=90.0) {
            Ok(Self(d))
        } else {
            Err(Error::GeographicDegreesOutOfBounds)
        }
    }

    /// Underlying decimal degrees
    pub fn as_float(&self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for Latitude {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl core::fmt::Display for Latitude {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let c = if self.0.is_sign_positive() { 'N' } else { 'S' };
        let d = self.0.abs();
        write!(f, "{d:.3}° {c}")
    }
}

/// Latitude in decimal degrees in the WGS 84 reference system
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Longitude(f64);

impl Longitude {
    /// Construct a longitude from a float
    ///
    /// Returns an error if the given longitude is out of bounds (±180°).
    pub fn new(d: f64) -> Result<Self, Error> {
        if matches!(d, -180.0..=180.0) {
            Ok(Self(d))
        } else {
            Err(Error::GeographicDegreesOutOfBounds)
        }
    }

    /// Underlying decimal degrees
    pub fn as_float(&self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for Longitude {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl core::fmt::Display for Longitude {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let c = if self.0.is_sign_positive() { 'W' } else { 'E' };
        let d = self.0.abs();
        write!(f, "{d:.3}° {c}")
    }
}

/// Coordinates in the WGS 84 coordinate reference system
#[derive(Debug, PartialEq, Deserialize, Clone, Copy)]
#[serde(try_from = "[f64; 3]")]
pub struct Coordinates {
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub altitude: Metres,
}

impl TryFrom<[f64; 3]> for Coordinates {
    type Error = Error;

    fn try_from(value: [f64; 3]) -> Result<Self, Self::Error> {
        let [lon, lat, alt] = value;
        let latitude = Latitude::new(lat)?;
        let longitude = Longitude::new(lon)?;
        let altitude = Metres(alt as f32);
        Ok(Self {
            latitude,
            longitude,
            altitude,
        })
    }
}

impl core::fmt::Display for Coordinates {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}, {} {}", self.latitude, self.longitude, self.altitude)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage(pub f32);

impl core::fmt::Display for Percentage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.0}%", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Metres(pub f32);

impl core::fmt::Display for Metres {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.0}m", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MetresPerSecond(pub f32);

impl core::fmt::Display for MetresPerSecond {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.2} m/s", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Millimetres(pub f32);

impl core::fmt::Display for Millimetres {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.2} mm", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MillimetresPerHour(pub f32);

impl core::fmt::Display for MillimetresPerHour {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.2} mm/hour", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(pub f32);

impl core::fmt::Display for Celsius {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.2}°C", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pascals(pub u32);

impl core::fmt::Display for Pascals {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} Pa", self.0)
    }
}

/// Degrees representing an azimuth
///
/// This represents a direction, from the perspective of a weather forecast location, relative to
/// north. For example, `Degrees(90.0)` is due east.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Degrees(pub f32);

impl core::fmt::Display for Degrees {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.0}°", self.0)
    }
}

/// UV index value
///
/// A unitless measure representing the strength of solar radiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UvIndex(pub u8);

impl UvIndex {
    /// Safety advice message for given UV index.
    pub fn advice_message(&self) -> &'static str {
        match self.0 {
            0..=2 => "No protection required. You can safely stay outside.",
            3..=5 => "Seek shade during midday hours, cover up and wear sunscreen.",
            6.. => {
                "Avoid being outside during midday hours. Shirt, sunscreen and hat are essential."
            }
        }
    }
}

impl core::fmt::Display for UvIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Most significant weather conditions
///
/// Derived from a "significant weather code", `Conditions` can be thought of as a
/// summary description for the conditions at a particular time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Conditions {
    TraceRain,
    ClearNight,
    SunnyDay,
    PartlyCloudyNight,
    PartlyCloudyDay,
    Mist,
    Fog,
    Cloudy,
    Overcast,
    LightRainShowerNight,
    LightRainShowerDay,
    Drizzle,
    LightRain,
    HeavyRainShowerNight,
    HeavyRainShowerDay,
    HeavyRain,
    SleetShowerNight,
    SleetShowerDay,
    Sleet,
    HailShowerNight,
    HailShowerDay,
    Hail,
    LightSnowShowerNight,
    LightSnowShowerDay,
    LightSnow,
    HeavySnowShowerNight,
    HeavySnowShowerDay,
    HeavySnow,
    ThunderShowerNight,
    ThunderShowerDay,
    Thunder,
}

impl TryFrom<i8> for Conditions {
    type Error = Error;

    fn try_from(code: i8) -> Result<Self, Self::Error> {
        use Conditions::*;
        Ok(match code {
            -1 => TraceRain,
            0 => ClearNight,
            1 => SunnyDay,
            2 => PartlyCloudyNight,
            3 => PartlyCloudyDay,
            // 4 not used
            5 => Mist,
            6 => Fog,
            7 => Cloudy,
            8 => Overcast,
            9 => LightRainShowerNight,
            10 => LightRainShowerDay,
            11 => Drizzle,
            12 => LightRain,
            13 => HeavyRainShowerNight,
            14 => HeavyRainShowerDay,
            15 => HeavyRain,
            16 => SleetShowerNight,
            17 => SleetShowerDay,
            18 => Sleet,
            19 => HailShowerNight,
            20 => HailShowerDay,
            21 => Hail,
            22 => LightSnowShowerNight,
            23 => LightSnowShowerDay,
            24 => LightSnow,
            25 => HeavySnowShowerNight,
            26 => HeavySnowShowerDay,
            27 => HeavySnow,
            28 => ThunderShowerNight,
            29 => ThunderShowerDay,
            30 => Thunder,
            _ => Err(Error::UnknownWeatherCondition(code))?,
        })
    }
}

impl core::fmt::Display for Conditions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Conditions::*;
        let s: &'static str = match self {
            TraceRain => "Trace of rain",
            ClearNight => "Clear",
            SunnyDay => "Sunny",
            PartlyCloudyNight | PartlyCloudyDay => "Partly Cloudy",
            Mist => "Mist",
            Fog => "Fog",
            Cloudy => "Cloudy",
            Overcast => "Overcast",
            LightRainShowerNight | LightRainShowerDay => "Light rain shower",
            Drizzle => "Drizzle",
            LightRain => "Light rain",
            HeavyRainShowerNight | HeavyRainShowerDay => "Heavy rain shower",
            HeavyRain => "Heavy rain",
            SleetShowerNight | SleetShowerDay => "Sleet shower",
            Sleet => "Sleet",
            HailShowerNight | HailShowerDay => "Hail shower",
            Hail => "Hail",
            LightSnowShowerNight | LightSnowShowerDay => "Light snow shower",
            LightSnow => "Light snow",
            HeavySnowShowerNight | HeavySnowShowerDay => "Heavy snow shower",
            HeavySnow => "Heavy snow",
            ThunderShowerNight | ThunderShowerDay => "Thunder shower",
            Thunder => "Thunder",
        };
        write!(f, "{s}")
    }
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
