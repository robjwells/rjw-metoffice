pub use error::Error;
use serde::Deserialize;

mod error {
    #[derive(Debug)]
    pub enum Error {
        Serde(serde_json::Error),
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

pub struct Forecast;

impl std::str::FromStr for Forecast {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

impl From<RawForecast> for Forecast {
    fn from(_value: RawForecast) -> Self {
        Forecast
    }
}

fn parse(s: &str) -> Result<Forecast, Error> {
    let rf: RawForecast = serde_json::from_str(s)?;
    Ok(Forecast::from(rf))
}

#[derive(Debug, Deserialize)]
struct RawForecast {}
