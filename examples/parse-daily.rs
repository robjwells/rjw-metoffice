fn main() {
    let path = std::env::args().nth(1).unwrap();
    let sample = std::fs::read_to_string(&path).unwrap();
    let forecast: rjw_metoffice::Forecast<rjw_metoffice::Daily> = sample.parse().unwrap();
    println!("{forecast:#?}");
}
