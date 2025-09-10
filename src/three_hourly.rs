use crate::Error;
use crate::parse::RawThreeHourlyForecast;
use crate::units::{
    Celsius, Conditions, Degrees, Metres, MetresPerSecond, Millimetres, Pascals, Percentage,
    UvIndex,
};

/// Forecast for a three-hour period
#[derive(Debug)]
pub struct ThreeHourly {
    /// Time at which this forecast is valid.
    pub time: jiff::Zoned,
    /// The most significant weather conditions at this time, taking into account both
    /// instantaneous and preceding conditions.
    pub conditions: Conditions,
    /// Maximum air temperature at screen level.
    ///
    /// Stevenson screen height is 1.5m above ground level.
    pub temperature_maximum: Celsius,
    /// Minimum air temperature at screen level.
    ///
    /// Stevenson screen height is 1.5m above ground level.
    pub temperature_minimum: Celsius,
    /// The temperature it feels like, taking into account humidity and wind chill but
    /// not radiation.
    pub temperature_feels_like: Celsius,
    /// Surface wind speed in metres per second.
    ///
    /// Equivalent to the mean speed over the 10 minutes preceding the validity time.
    /// This is measured at 10m but is considered the surface wind.
    pub wind_speed: MetresPerSecond,
    /// Direction from which the wind is blowing in degrees.
    ///
    /// Equivalent to the mean direction over the 10 minutes preceding the validity
    /// time. This is measured at 10m but is considered the surface wind.
    pub wind_direction: Degrees,
    /// Maximum 3-second mean wind speed over the 10 minutes preceding the validity time.
    pub wind_gust_speed: MetresPerSecond,
    /// Most extreme wind speed that might be experienced in this period.
    pub wind_gust_three_hourly_maximum: MetresPerSecond,
    /// Distance in metres at which a known object can be seen horizontally from screen level (1.5m).
    pub visibility: Metres,
    /// Percent relative humidity at screen level (1.5m).
    pub relative_humidity: Percentage,
    /// Air pressure at mean sea level in Pascals.
    pub pressure: Pascals,
    /// Maximum UV value over the previous three hours.
    ///
    /// Usually a value 0 to 13 but higher values are possible in extreme situations.
    pub uv_index: UvIndex,
    /// Implied depth of the layer of liquid water which has been deposited on the
    /// surface in the previous three hours.
    pub precipitation_total: Millimetres,
    /// Amount of snow that has fallen out of the sky in the previous three hours.
    ///
    /// This does not reflect snow lying on the ground. Falling snow may not settle at all and
    /// may be accompanied by rain (ie is sleet). Falling snow is stated as liquid water
    /// equivalent in mm, which can be considered approximately the same as cm of fresh snow or
    /// as a kilogram per square metre.
    pub snow_total: Millimetres,
    /// Probability of precipitation over the three hours centred at the validity time.
    pub precipitation_probability: Percentage,
    /// Probability of rain occuring over the three hours centred at the validity time.
    pub rain_probability: Percentage,
    /// Probability of heavy rain occuring over the three hours centred at the validity time.
    ///
    /// Heavy rain is defined as more than 1mm per hour.
    pub heavy_rain_probability: Percentage,
    /// Probability of snow over the three hours centred at the validity time.
    pub snow_probability: Percentage,
    /// Probability of heavy snow over the three hours centred at the validity time.
    ///
    /// Heavy snow is defined as more than 1mm liquid water equivalent per hour, equivalent to
    /// more than 1cm snow per hour.
    pub heavy_snow_probability: Percentage,
    /// Probability of hail occuring over the three hours centred at the validity time.
    pub hail_probability: Percentage,
    /// Probability of lightning occuring over the three hours centred at the validity time.
    ///
    /// This is the probability of a strike within a radius of 50km (31 miles).
    pub lightning_probability: Percentage,
}

impl TryFrom<RawThreeHourlyForecast> for ThreeHourly {
    type Error = Error;

    fn try_from(rf: RawThreeHourlyForecast) -> Result<Self, Self::Error> {
        Ok(Self {
            time: rf.time,
            conditions: rf.significant_weather_code.try_into()?,
            temperature_feels_like: Celsius(rf.feels_like_temp),
            temperature_maximum: Celsius(rf.max_screen_air_temp),
            temperature_minimum: Celsius(rf.min_screen_air_temp),
            precipitation_probability: Percentage(rf.prob_of_precipitation),
            precipitation_total: Millimetres(rf.total_precip_amount),
            snow_total: Millimetres(rf.total_snow_amount),
            wind_speed: MetresPerSecond(rf.wind_speed_10m),
            wind_direction: Degrees(rf.wind_direction_from_10m),
            wind_gust_speed: MetresPerSecond(rf.wind_gust_speed_10m),
            wind_gust_three_hourly_maximum: MetresPerSecond(rf.max_10m_wind_gust),
            visibility: Metres(rf.visibility),
            relative_humidity: Percentage(rf.screen_relative_humidity),
            pressure: Pascals(rf.mslp),
            uv_index: UvIndex(rf.uv_index),
            snow_probability: Percentage(rf.prob_of_snow),
            heavy_snow_probability: Percentage(rf.prob_of_heavy_snow),
            rain_probability: Percentage(rf.prob_of_rain),
            heavy_rain_probability: Percentage(rf.prob_of_heavy_rain),
            hail_probability: Percentage(rf.prob_of_hail),
            lightning_probability: Percentage(rf.prob_of_sferics),
        })
    }
}
