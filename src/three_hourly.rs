use crate::{
    Celsius, Conditions, Degrees, Error, Metres, MetresPerSecond, Millimetres, Pascals, Percentage,
    UvIndex, parse::RawThreeHourlyForecast,
};

#[derive(Debug)]
pub struct ThreeHourly {
    /// Time at which this forecast is valid.
    pub time: jiff::Zoned,
    pub conditions: Conditions,
    /// Maximum air temperature at screen level.
    pub screen_maximum_temperature: Celsius,
    /// Minimum air temperature at screen level.
    pub screen_minimum_temperature: Celsius,
    /// The temperature it feels like, taking into account humidity and wind chill but
    /// not radiation.
    pub feels_like_temperature: Celsius,
    /// Surface wind speed in metres per second.
    pub wind_speed: MetresPerSecond,
    /// Direction from which the wind is blowing in degrees.
    pub wind_direction: Degrees,
    /// Maximum 3-second mean wind speed.
    pub gust_speed: MetresPerSecond,
    /// Maximum 3-second mean wind speed.
    pub gust_hourly_maximum_speed: MetresPerSecond,
    /// Distance in metres at which a known object can be seen horizontally from screen level (1.5m.)
    pub visibility: Metres,
    /// Percent relative humidity at screen level (1.5m).
    pub relative_humidity: Percentage,
    /// Air pressure at mean sea level in Pascals.
    pub pressure: Pascals,
    /// Maxmium UV value over the hour preceding the validity time. Usually a value 0 to 13 but
    /// higher values are possible in extreme situations.
    pub uv_index: UvIndex,
    /// Implied depth of the layer of liquid water which has been deposited on the
    /// surface since the previous hour.
    pub precipitation_total: Millimetres,
    /// Amount of snow that has fallen out of the sky in the last hour.
    ///
    /// This does not reflect snow lying on the ground. Falling snow may not settle at all and may
    /// be accompanied by rain (ie is sleet). Falling snow is stated as liquid water equivalent in
    /// mm, which can be considered approximately the same as cm of fresh snow or as a kilogram per
    /// square metre.
    pub snow_total: Millimetres,
    /// Probability of precipitation over the hour centered at the validity time.
    pub precipitation_probability: Percentage,
    pub snow_probability: Percentage,
    pub heavy_snow_probability: Percentage,
    pub rain_probability: Percentage,
    pub heavy_rain_probability: Percentage,
    pub hail_probability: Percentage,
    pub sferics_probability: Percentage,
}

impl TryFrom<RawThreeHourlyForecast> for ThreeHourly {
    type Error = Error;

    fn try_from(rf: RawThreeHourlyForecast) -> Result<Self, Self::Error> {
        Ok(Self {
            time: rf.time,
            conditions: rf.significant_weather_code.try_into()?,
            feels_like_temperature: Celsius(rf.feels_like_temperature),
            screen_maximum_temperature: Celsius(rf.max_screen_air_temp),
            screen_minimum_temperature: Celsius(rf.min_screen_air_temp),
            precipitation_probability: Percentage(rf.prob_of_precipitation),
            precipitation_total: Millimetres(rf.total_precip_amount),
            snow_total: Millimetres(rf.total_snow_amount),
            wind_speed: MetresPerSecond(rf.wind_speed_10m),
            wind_direction: Degrees(rf.wind_direction_from_10m),
            gust_speed: MetresPerSecond(rf.wind_gust_speed_10m),
            gust_hourly_maximum_speed: MetresPerSecond(rf.max_10m_wind_gust),
            visibility: Metres(rf.visibility),
            relative_humidity: Percentage(rf.screen_relative_humidity),
            pressure: Pascals(rf.mslp),
            uv_index: UvIndex(rf.uv_index),
            snow_probability: Percentage(rf.prob_of_snow),
            heavy_snow_probability: Percentage(rf.prob_of_heavy_snow),
            rain_probability: Percentage(rf.prob_of_rain),
            heavy_rain_probability: Percentage(rf.prob_of_heavy_rain),
            hail_probability: Percentage(rf.prob_of_hail),
            sferics_probability: Percentage(rf.prob_of_sferics),
        })
    }
}
