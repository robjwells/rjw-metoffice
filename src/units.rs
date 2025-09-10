use serde::Deserialize;

use crate::Error;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Latitude(f64);

impl Latitude {
    pub fn new(d: f64) -> Result<Self, Error> {
        if matches!(d, -90.0..=90.0) {
            Ok(Self(d))
        } else {
            Err(Error::CoordinatesOutOfBounds)
        }
    }

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

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Longitude(f64);

impl Longitude {
    pub fn new(d: f64) -> Result<Self, Error> {
        if matches!(d, -180.0..=180.0) {
            Ok(Self(d))
        } else {
            Err(Error::CoordinatesOutOfBounds)
        }
    }

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

#[derive(Debug, PartialEq, Deserialize)]
#[serde(try_from = "[f64; 3]")]
/// Coordinates in the WGS 84 coordinate reference system.
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage(pub f32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Metres(pub f32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MetresPerSecond(pub f32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Millimetres(pub f32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MillimetresPerHour(pub f32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(pub f32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pascals(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Degrees(pub f32);

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
            _ => Err(Error::UnknownCondition(code))?,
        })
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
