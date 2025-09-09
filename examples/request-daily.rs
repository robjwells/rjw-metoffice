use rjw_metoffice::{Daily, Forecast};

fn main() {
    let apikey = std::env::var("MET_OFFICE_DATAHUB_KEY")
        .expect("MET_OFFICE_DATAHUB_KEY environment variable must be set");
    let mut args = std::env::args();
    let lat: f64 = args
        .nth(1)
        .expect("Provide latitude as the first argument")
        .parse()
        .expect("Could not parse first argument as a floating-point number");
    let lon: f64 = args
        .next()
        .expect("Provide longitude as the second argument")
        .parse()
        .expect("Could not parse second argument as a floating-point number");

    let url = rjw_metoffice::daily_predictions_url_for_location(lat, lon);
    let resp = ureq::get(url.to_string())
        .header("apikey", apikey)
        .call()
        .expect("Did not successfully make request and receive response");
    let bytes = resp
        .into_body()
        .read_to_vec()
        .expect("Failed to read response into vec.");
    let forecast: Forecast<Daily> = bytes
        .as_slice()
        .try_into()
        .expect("Failed to parse body as forecast");
    println!("{forecast:#?}");
}
