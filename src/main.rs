use std::error::Error;

use clap::Parser;
use metoffice::ApiKey;

/// Fetches hourly data from the Met Office DataHub API.
///
/// The DataHub API will choose the weather station closest to the given
/// latitude and longitude.
///
/// It is *highly recommended* to pass your API key ID and secret via
/// the MET_OFFICE_DATAHUB_KEY_ID and MET_OFFICE_DATAHUB_KEY_SECRET
/// environment variables rather than the command-line arguments.
#[derive(Debug, Parser)]
struct Args {
    /// DataHub key ID.
    ///
    /// Prefer to pass this as an environment variable.
    #[arg(long, env = "MET_OFFICE_DATAHUB_KEY_ID")]
    key_id: String,

    /// DataHub key secret.
    ///
    /// Prefer to pass this as an environment variable.
    #[arg(long, env = "MET_OFFICE_DATAHUB_KEY_SECRET")]
    key_secret: String,

    /// Location latitude in decimal degrees.
    #[arg(long = "lat")]
    latitude: f64,

    /// Location longitude in decimal degrees.
    #[arg(long = "lon")]
    longitude: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args {
        key_id,
        key_secret,
        latitude,
        longitude,
    } = Args::parse();
    let key = ApiKey {
        id: key_id,
        secret: key_secret,
    };
    let forecast_data = metoffice::fetch_hourly_forecasts(&key, latitude, longitude)?;

    for forecast in forecast_data.time_series {
        println!(
            "{}\t{}, {}",
            forecast.time.format("%-l%p %a %-d"),
            forecast.significant_weather_code,
            forecast.screen_temperature,
        );
    }

    Ok(())
}
