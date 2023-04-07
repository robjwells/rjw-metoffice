use std::error::Error;

use clap::Parser;
use metoffice::ApiKey;

#[derive(Parser)]
struct Args {
    /// DataHub key ID.
    #[arg(long)]
    key_id: String,

    /// DataHub key secret.
    #[arg(long)]
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
