use jiff::tz::TimeZone;
use rjw_metoffice::units::{Coordinates, Metres};
use rjw_metoffice::{Forecast, Hourly};

const SAMPLE: &str = include_str!("global-spot-hourly-sample.json");

#[test]
pub fn no_error_from_sample() {
    let _f: Forecast<Hourly> = SAMPLE.parse().expect("Failed to parse");
}

#[test]
pub fn has_coordinates() {
    let f: Forecast<Hourly> = SAMPLE.parse().expect("Failed to parse");
    let expected: Coordinates = [-3.474, 50.727, 27.0].try_into().unwrap();
    assert_eq!(f.coordinates, expected)
}

#[test]
pub fn has_request_point_distance() {
    let f: Forecast<Hourly> = SAMPLE.parse().expect("Failed to parse");
    let expected = Metres(27.9057);
    assert_eq!(f.requested_point_distance, expected)
}

#[test]
pub fn has_zoned_predictions_run_time() {
    let f: Forecast<Hourly> = SAMPLE.parse().expect("Failed to parse");
    let expected = jiff::civil::date(2023, 7, 5)
        .at(10, 0, 0, 0)
        .to_zoned(TimeZone::UTC)
        .unwrap();
    assert_eq!(f.predictions_made_at, expected)
}
