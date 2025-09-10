use crate::Error;
use crate::parse::RawHourlyForecast;
use crate::units::{
    Celsius, Conditions, Degrees, Metres, MetresPerSecond, Millimetres, MillimetresPerHour,
    Pascals, Percentage, UvIndex,
};

#[derive(Debug)]
pub struct Hourly {
    /// Time at which this forecast is valid.
    pub time: jiff::Zoned,
    /// The most significant weather conditions at this time, taking into account both
    /// instantaneous and preceding conditions.
    pub conditions: Conditions,
    /// Temperature at screen level.
    ///
    /// Stevenson screen height is approximately 1.5m above ground level.
    pub temperature: Celsius,
    /// Maximum air temperature at screen level.
    ///
    /// Appears to be missing after 48 hours.
    pub temperature_maximum: Option<Celsius>,
    /// Minimum air temperature at screen level.
    ///
    /// Appears to be missing after 48 hours.
    pub temperature_minimum: Option<Celsius>,
    /// The temperature it feels like, taking into account humidity and wind chill but
    /// not radiation.
    pub temperature_feels_like: Celsius,
    /// Dew point temperature at screen level.
    ///
    /// Stevenson screen height is approximately 1.5m above ground level.
    pub screen_dew_point_temperature: Celsius,
    /// Probability of precipitation over the hour centred at the validity time.
    pub precipitation_probability: Percentage,
    /// Rate at which liquid water is being deposited on the surface, in mm per hour.
    pub precipitation_rate: MillimetresPerHour,
    /// Implied depth of the layer of liquid water which has been deposited on the
    /// surface since the previous hour.
    ///
    /// Appears to be missing after 48 hours.
    pub precipitation_total: Option<Millimetres>,
    /// Amount of snow that has fallen out of the sky in the last hour.
    ///
    /// This does not reflect snow lying on the ground. Falling snow may not settle at all and may
    /// be accompanied by rain (ie is sleet). Falling snow is stated as liquid water equivalent in
    /// mm, which can be considered approximately the same as cm of fresh snow or as a kilogram per
    /// square metre.
    ///
    /// Appears to be missing after 48 hours.
    pub snow_total: Option<Millimetres>,
    /// Surface wind speed in metres per second.
    ///
    /// Mean wind speed is equivalent to the mean speed observed over the 10 minutes preceding the
    /// validity time. Measured at 10 metres above ground, this is considered surface wind speed.
    pub wind_speed: MetresPerSecond,
    /// Direction from which the wind is blowing in degrees.
    ///
    /// Mean wind direction is equivalent to the mean direction observed over the 10 minutes
    /// preceding the validity time. Measured at 10 metres above ground, this is considered surface
    /// wind direction.
    pub wind_direction: Degrees,
    /// Maximum 3-second mean wind speed observed over the 10 minutes preciding the validity time.
    pub wind_gust_speed: MetresPerSecond,
    /// Maximum 3-second mean wind speed observed over the hour preciding the validity time.
    ///
    /// Appears to be missing after 48 hours.
    pub wind_gust_hourly_maximum_speed: Option<MetresPerSecond>,
    /// Distance in metres at which a known object can be seen horizontally from screen level (1.5m.)
    pub visibility: Metres,
    /// Percent relative humidity at screen level (1.5m).
    pub relative_humidity: Percentage,
    /// Air pressure at mean sea level in Pascals.
    pub pressure: Pascals,
    /// Maxmium UV value over the hour preceding the validity time. Usually a value 0 to 13 but
    /// higher values are possible in extreme situations.
    pub uv_index: UvIndex,
}

impl TryFrom<RawHourlyForecast> for Hourly {
    type Error = Error;

    fn try_from(rf: RawHourlyForecast) -> Result<Self, Self::Error> {
        Ok(Self {
            time: rf.time,
            conditions: rf.significant_weather_code.try_into()?,
            temperature: Celsius(rf.screen_temperature),
            temperature_feels_like: Celsius(rf.feels_like_temperature),
            screen_dew_point_temperature: Celsius(rf.screen_dew_point_temperature),
            temperature_maximum: rf.max_screen_air_temp.map(Celsius),
            temperature_minimum: rf.min_screen_air_temp.map(Celsius),
            precipitation_probability: Percentage(rf.prob_of_precipitation),
            precipitation_rate: MillimetresPerHour(rf.precipitation_rate),
            precipitation_total: rf.total_precip_amount.map(Millimetres),
            snow_total: rf.total_snow_amount.map(Millimetres),
            wind_speed: MetresPerSecond(rf.wind_speed_10m),
            wind_direction: Degrees(rf.wind_direction_from_10m),
            wind_gust_speed: MetresPerSecond(rf.wind_gust_speed_10m),
            wind_gust_hourly_maximum_speed: rf.max_10m_wind_gust.map(MetresPerSecond),
            visibility: Metres(rf.visibility),
            relative_humidity: Percentage(rf.screen_relative_humidity),
            pressure: Pascals(rf.mslp),
            uv_index: UvIndex(rf.uv_index),
        })
    }
}
