pub use error::Error;
use serde::Deserialize;

mod error {
    #[derive(Debug)]
    pub enum Error {
        Serde(serde_json::Error),
        CoordinatesOutOfBounds,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
    impl core::error::Error for Error {}

    impl From<serde_json::Error> for Error {
        fn from(value: serde_json::Error) -> Self {
            Self::Serde(value)
        }
    }
}

#[derive(Debug)]
pub struct Forecast {
    pub coordinates: Coordinates,
}

impl std::str::FromStr for Forecast {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

impl From<RawForecast> for Forecast {
    fn from(mut value: RawForecast) -> Self {
        let feature = value.features.remove(0);
        Forecast {
            coordinates: feature.geometry.coordinates,
        }
    }
}

fn parse(s: &str) -> Result<Forecast, Error> {
    let rf: RawForecast = serde_json::from_str(s)?;
    Ok(Forecast::from(rf))
}

#[derive(Debug, Deserialize)]
struct RawForecast {
    features: Vec<RawFeature>,
}

#[derive(Debug, Deserialize)]
struct RawFeature {
    geometry: Geometry,
}

#[derive(Debug, Deserialize)]
struct Geometry {
    coordinates: Coordinates,
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

#[cfg(test)]
mod test {
    use crate::Coordinates;

    #[test]
    fn coordinates_only_in_bounds() {
        let oob: Vec<[f64; 3]> = vec![
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
