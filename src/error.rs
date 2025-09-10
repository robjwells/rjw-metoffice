/// Wrapper type for all possible errors
#[derive(Debug)]
pub enum Error {
    /// Error encountered while parsing JSON
    Serde(serde_json::Error),
    /// A given latitude or longitude is out of the acceptable range
    GeographicDegreesOutOfBounds,
    /// Significant forecast code does not match a known value
    UnknownWeatherCondition(i8),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s: &'static str = match self {
            Error::Serde(_) => "JSON parsing error",
            Error::GeographicDegreesOutOfBounds => "invalid geographic degrees",
            Error::UnknownWeatherCondition(_) => "unknown significant weather code",
        };
        write!(f, "{s}")
    }
}

impl core::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
