use std::error::Error;

use metoffice::SiteForecast;

fn main() -> Result<(), Box<dyn Error>> {
    // {
    //     let file = File::open("shanklin.json")?;
    //     let result: Response = serde_json::from_reader(file)?;
    //     println!("{:#?}", result);
    // }
    // {
    //     let file = File::open("shanklin.json")?;
    //     let forecast = metoffice::site_forecast_from_reader(file)?;
    //     println!("{:#?}", forecast);
    // }

    let mut creds = include_str!("../.creds.txt").lines();
    let id = creds.next().unwrap().to_owned();
    let secret = creds.next().unwrap().to_owned();
    let key = ApiKey { id, secret };
    let forecast = fetch(&key, 52.63444, -1.131944)?;

    for f in forecast.time_series {
        println!("{}\t{:?}\t{}", f.time, f.significant_weather_code, f.feels_like_temperature);
    }

    Ok(())
}

struct ApiKey {
    id: String,
    secret: String,
}

fn fetch(key: &ApiKey, latitude: f64, longitude: f64) -> Result<SiteForecast, Box<dyn Error>> {
    let lat = format!("{latitude}");
    let lon = format!("{longitude}");
    let query_params = [
        ("includeLocationName", "true"),
        ("excludeParameterMetadata", "true"),
        ("latitude", &lat),
        ("longitude", &lon),
    ];
    let client = reqwest::blocking::Client::new()
        .get("https://api-metoffice.apiconnect.ibmcloud.com/v0/forecasts/point/hourly")
        .query(&query_params)
        .header("X-IBM-Client-Id", &key.id)
        .header("X-IBM-Client-Secret", &key.secret);

    let response = client.send()?;
    if cfg!(debug_assertions) {
        let ratelimit_header = response.headers().get("x-ratelimit-remaining");
        debug_assert!(ratelimit_header.is_some());
        let remaining = ratelimit_header
            .unwrap()
            .to_str()
            .unwrap()
            .split(',')
            .nth(1)
            .unwrap()
            .replace(';', "");
        eprintln!(
            "DEBUG :: API :: Remaining Met Office API calls :: {}",
            remaining
        );
    }

    let body = response.text()?;
    let parsed = metoffice::site_forecast_from_reader(body.as_bytes()).unwrap();
    Ok(parsed)
}
