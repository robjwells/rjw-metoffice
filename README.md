## rjw-metoffice

A Rust crate to help use the Met Office [Global Spot][] site-specific weather forecast API.

[Global Spot]: https://datahub.metoffice.gov.uk/docs/f/category/site-specific/overview

Global Spot provides hourly, three-hourly, and daily forecasts for 20,000
locations worldwide. You will need an API key from the [Weather DataHub
website][], which will allow you to make 360 API requests per day for free.

[Weather DataHub website]: https://datahub.metoffice.gov.uk/

This crate is HTTP client agnostic: it helps you construct URLs to request a
forecast type at a given location, and parses the returned JSON. It does not do
any IO itself, so you will need to pair it with your preferred HTTP client.

### Quickstart

Here's a simple example using the [`ureq`] blocking HTTP client to look up
hourly forecasts for [Sana’a], Yemen.

[`ureq`]: https://crates.io/crates/ureq
[Sana’a]: https://whc.unesco.org/en/list/385

```rust
let [lat, lon] = [15.348333, 44.206389];
let url = Forecast::<Hourly>::url_for_location(lat.try_into()?, lon.try_into()?);
let response = ureq::get(url.to_string())
    .header("apikey", API_KEY)
    .call()?;
let json_bytes = response.into_body().read_to_vec()?;
let forecast: Forecast<Hourly> = json_bytes.as_slice().try_into()?;
for hour in forecast.predictions {
    println!("{}    {}", hour.time.in_tz("Asia/Riyadh")?, hour.temperature)
}
```

### API key HTTP header

You **must** add a `apikey` header to the HTTP request containing your Met
Office Weather DataHub API key. This library cannot do this for you, so please
consult the documentation for the HTTP client you are using.

### Missing data

Note that hourly forecasts are missing certain data in for the last three time
points, and the daily forecasts are missing data for the first day (which is in
the past). This is just what is returned from the API, please see the module
documentation for details.

### `no_std` and memory usage

This crate does not require the Rust standard library (`std`), so it is
possible to use it on embedded devices. However, it does still require a memory
allocator and the JSON text returned from the API is quite large: 24 KiB for
hourly forecasts, 28 KiB for three-hourly, and 12 KiB for daily.

The main `Forecast` struct takes just under 10 KiB for hourly and three-hourly
forecasts, and 2 KiB for daily forecasts. You will need to budget JSON +
`Forecast` as the JSON parsing does allocate.
