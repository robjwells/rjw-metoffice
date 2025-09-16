//! `rjw-metoffice` helps construct URLs for and parse the response from the Met Office Global Spot
//! site-specific weather forecast API. Global Spot provides deterministic forecasts for 20,000
//! locations worldwide, giving the "most likely" prediction for a given place.
//!
//! You will need a [Weather DataHub] API key to use the service, which can be obtained
//! for free from the [website][Weather DataHub], and permits 360 API requests per day.
//!
//! [Weather DataHub]: https://datahub.metoffice.gov.uk/
//!
//! ## Quickstart
//!
//! Here's a simple example using the [`ureq`] blocking HTTP client to look up hourly forecasts for
//! [Sana’a], Yemen.
//!
//! [`ureq`]: https://crates.io/crates/ureq
//! [Sana’a]: https://whc.unesco.org/en/list/385
//!
//! ```no_run
//! # use rjw_metoffice::{Forecast, Hourly};
//! # fn main() -> anyhow::Result<()> {
//! # let API_KEY: &str = "";
//! let [lat, lon] = [15.348333, 44.206389];
//! let url = Forecast::<Hourly>::url_for_location(lat.try_into()?, lon.try_into()?);
//! let response = ureq::get(url.to_string())
//!     .header("apikey", API_KEY)
//!     .call()?;
//! let json_bytes = response.into_body().read_to_vec()?;
//! let forecast: Forecast<Hourly> = json_bytes.as_slice().try_into()?;
//! for hour in forecast.predictions {
//!     println!("{}    {}", hour.time.in_tz("Asia/Riyadh")?, hour.temperature)
//! }
//! # Ok(())
//! # }
//! ````
//!
//! ## HTTP client agnostic
//!
//! The library doesn't perform any IO itself, so you can use whichever HTTP client you like, and
//! use it in both synchronous or asynchronous contexts. In effect, you "bring your own HTTP
//! client": use the library to construct a location-specific URL, use your HTTP client to make the
//! request, and pass the received bytes to the library for parsing.
//!
//! ## API key HTTP header
//!
//! You **must** add a `apikey` header to the HTTP request containing your Met Office Weather
//! DataHub API key. This library cannot do this for you, so please consult the documentation for
//! the HTTP client you are using.
//!
//! ## Forecast time periods and the `Forecast` struct
//!
//! The Global Spot API offers forecasts at three levels of granularity: hourly (for 48
//! hours), three-hourly (168 hours, ie 7 days), and daily (7 days).
//!
//! These time periods are used as a generic parameter to the [`Forecast`] struct, which which
//! contains general information and a vector of time-period-specific structs that hold the
//! prediction data. This generic parameter is also needed when constructing a URL, for instance
//! here we construct URLs for all three time periods for "[null island]".
//!
//! [null island]: https://en.wikipedia.org/wiki/Null_Island
//!
//! ```
//! # use rjw_metoffice::{Forecast, Hourly, ThreeHourly, Daily, Latitude, Longitude};
//! let lat = Latitude::new(0.0).unwrap();
//! let lon = Longitude::new(0.0).unwrap();
//! let hourly_url = Forecast::<Hourly>::url_for_location(lat, lon);
//! let three_hourly_url = Forecast::<ThreeHourly>::url_for_location(lat, lon);
//! let daily_url = Forecast::<Daily>::url_for_location(lat, lon);
//! ````
//!
//! Similarly, the generic is used to determine parsing behaviour via string or byte slices
//! (`FromStr` or `TryFrom<&[u8]>`).
//!
//! ## Prediction field names
//!
//! Generally, the Met Office field names are not used. This is to make field names consistent
//! across time point structs (versus, for example, `feelsLikeTemperature` in hourly forecasts and
//! `feelsLikeTemp` in three-hourly forecasts), as well as to provide arguably more intuitive and
//! discoverable names (for example, `pressure` instead of `mslp`).
//!
//! Please see the documentation for each time point for a table mapping field names back to the
//! originals that are present in the Met Office JSON and official API documentation.
//!
//! If you feel particularly strongly that anything is misnamed, please open an issue.
//!
//! ## Missing data
//!
//! **Hourly** forecasts contain 49 separate sets of predictions (start + 48 hours), but I have
//! observed that the final 3 hours are missing five data points (of 19):
//!
//! - Maximum temperature
//! - Minimum temperature
//! - Total precipitation
//! - Total snowfall
//! - Maximum gust speed over the previous hour
//!
//! As well, these seem to be missing from some forecast locations entirely. These are represented
//! as `Option`s in the [`Hourly`] struct. If you are interested only in particular locations, I
//! recommend checking which data you receive at which time points, after which you should be OK to
//! `unwrap` the reliable ones.
//!
//! [`Hourly`]: crate::Hourly
//!
//! Meanwhile, **[daily]** forecasts have an 8-element time series (previous day + 7 days). The
//! first of these is missing 10 daytime data points (of 21; there are 21 predictions each for day
//! and night). Because this is such a big difference, the [`Day`] predictions are an enum of
//! either the [past] day (missing predictions) or [future] days (with full predictions).
//!
//! [daily]: crate::daily::Daily
//! [`Day`]: crate::daily::Day
//! [past]: crate::daily::Day::Past
//! [future]: crate::daily::Day::Future
//!
//! ## Memory usage
//!
//! While this crate is `no_std`, it still requires a memory allocator (ie, uses `alloc`).
//! The JSON returned from the Met Office spot APIs is roughly 24 KiB for the hourly
//! forecasts, 28 KiB for three-hourly, and 12 KiB for daily. The `Forecast` struct takes
//! a bit under 10 KiB for hourly and three-hourly forecasts, and 2 KiB for daily.
//! The JSON parsing does allocate, so you'll want to budget JSON + `Forecast`.

#![no_std]

extern crate alloc;

pub mod daily;
mod error;
mod forecast;
mod hourly;
mod parse;
mod sealed;
mod three_hourly;
pub mod units;

pub use daily::Daily;
pub use error::Error;
pub use forecast::Forecast;
pub use hourly::Hourly;
pub use sealed::TimePeriod;
pub use three_hourly::ThreeHourly;
pub use units::{Latitude, Longitude};
