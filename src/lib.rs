#![no_std]

// TODO: Document expected memory requirements for (separately) JSON text and forecast structs.
extern crate alloc;

mod daily;
mod error;
mod forecast;
mod hourly;
mod parse;
mod sealed;
mod three_hourly;
mod units;

pub use daily::Daily;
pub use error::Error;
pub use forecast::Forecast;
pub use hourly::Hourly;
pub use sealed::TimePeriod;
pub use three_hourly::ThreeHourly;
pub use units::*;
