#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    CoordinatesOutOfBounds,
    UnknownCondition(i8),
}

impl core::fmt::Display for Error {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}
impl core::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
