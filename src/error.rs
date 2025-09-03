#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    CoordinatesOutOfBounds,
    UnknownCondition(i8),
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

