use rjw_metoffice::{Coordinates, Forecast};

const SAMPLE: &str = include_str!("global-spot-hourly-sample.json");

#[test]
pub fn no_error_from_sample() {
    let _f: Forecast = SAMPLE.parse().expect("Failed to parse");
}

#[test]
pub fn has_coordinates() {
    let f: Forecast = SAMPLE.parse().expect("Failed to parse");
    let expected: Coordinates = [-3.474, 50.727, 27.0].try_into().unwrap();
    assert_eq!(f.coordinates, expected)
}
