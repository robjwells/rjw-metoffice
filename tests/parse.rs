use rjw_metoffice::Forecast;

const SAMPLE: &str = include_str!("global-spot-hourly-sample.json");

#[test]
pub fn no_error_from_sample() {
    let _f: Forecast = SAMPLE.parse().expect("Failed to parse");
}
