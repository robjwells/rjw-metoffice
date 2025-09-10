//! Daily forecast specific types

use crate::Error;
use crate::parse::RawDailyForecast;
use crate::units::{
    Celsius, Conditions, Degrees, Metres, MetresPerSecond, Pascals, Percentage, UvIndex,
};

/// Forecast for a particular day and the following night
///
/// "Day" is from local dawn to dusk, "night" from dusk to dawn.
#[derive(Debug)]
pub struct Daily {
    /// Time at which this forecast is valid
    pub time: jiff::Zoned,
    /// Daytime forecast data
    ///
    /// Daytime is defined as the period from local dawn to local dusk.
    ///
    /// This is an enum distinguishing between the first (preceding) day, and later (future) days.
    /// The first day is missing several data that are present for future days.
    pub day: Day,
    /// Nighttime forecast data
    ///
    /// Nighttime is defined as the period from local dusk to local dawn.
    pub night: Night,
}

/// Prediction for a maximum or minimum temperature
#[derive(Debug)]
pub struct TemperaturePrediction {
    /// Most likely extreme temperature for a particular day or night
    pub most_likely: Celsius,
    /// 97.5% confidence upper bound of the predicted temperature
    pub upper_bound: Celsius,
    /// 97.5% confidence lower bound of the predicted temperature
    pub lower_bound: Celsius,
}

/// Daytime forecast data
///
/// Daytime is defined as the period from local dawn to local dusk.
///
/// The first `Day` struct in the time period is in the past and missing several data
/// points, while future days have all data.
///
/// Fields given "at midday" are always at 12pm (noon) in the forecast location's local timezone,
/// all others are "during the day", from dawn to dusk.
#[derive(Debug)]
pub enum Day {
    Past {
        /// Mean wind speed at midday
        ///
        /// This is the mean speed over the 10 minutes to midday and is measured at 10m
        /// above ground level.
        wind_speed: MetresPerSecond,
        /// Mean wind direction at midday
        ///
        /// This is the mean direction over the 10 minutes to midday and is measured at 10m
        /// above ground level.
        wind_direction: Degrees,
        /// Mean wind gust speed at midday
        ///
        /// This is the maximum 3-second mean wind speed over the 10 minutes to midday and is
        /// measured at 10m above ground level.
        wind_gust_speed: MetresPerSecond,
        /// Visibility in metres at midday
        visibility: Metres,
        /// Relative humidity at midday
        ///
        /// Measured at screen height, about 1.5m above ground level.
        relative_humidity: Percentage,
        /// Air pressure at mean sea level at midday
        pressure: Pascals,
        /// Maximum air temperature
        ///
        /// Measured at screen height, about 1.5m above ground level.
        temperature_maximum: TemperaturePrediction,
        /// Predicted upper bound for the daytime feels-like temperature
        ///
        /// This is given at 97.5% confidence, ie there is a 97.5% probability that the maximum
        /// feels-like temperature will be below this temperature.
        ///
        /// This is the temperature it feels like taking into account humidity and wind chill but
        /// not radiation.
        temperature_feels_like_maximum_upper_bound: Celsius,
        /// Predicted lower bound for the daytime feels-like temperature
        ///
        /// This is given at 97.5% confidence, ie there is a 97.5% probability that the maximum
        /// feels-like temperature will be above this temperature.
        ///
        /// This is the temperature it feels like taking into account humidity and wind chill but
        /// not radiation.
        temperature_feels_like_maximum_lower_bound: Celsius,
    },
    Future {
        /// Mean wind speed at midday
        ///
        /// This is the mean speed over the 10 minutes to midday and is measured at 10m
        /// above ground level.
        wind_speed: MetresPerSecond,
        /// Mean wind direction at midday
        ///
        /// This is the mean direction over the 10 minutes to midday and is measured at 10m
        /// above ground level.
        wind_direction: Degrees,
        /// Mean wind gust speed at midday
        ///
        /// This is the maximum 3-second mean wind speed over the 10 minutes to midday and is
        /// measured at 10m above ground level.
        wind_gust_speed: MetresPerSecond,
        /// Visibility in metres at midday
        visibility: Metres,
        /// Relative humidity at midday
        ///
        /// Measured at screen height, about 1.5m above ground level.
        relative_humidity: Percentage,
        /// Air pressure at mean sea level at midday
        pressure: Pascals,
        /// Maximum UV index
        uv_index_maximum: UvIndex,
        /// The most significant weather conditions
        conditions: Conditions,
        /// Maximum air temperature
        ///
        /// Measured at screen height, about 1.5m above ground level.
        temperature_maximum: TemperaturePrediction,
        /// Maximum temperature it might feel like
        ///
        /// This takes into account humidity and wind chill but not radiation.
        temperature_feels_like_maximum: TemperaturePrediction,
        /// Probability of any precipitation
        precipitation_probability: Percentage,
        /// Probability of rain
        rain_probability: Percentage,
        /// Probability of heavy rain
        ///
        /// Heavy rain is defined as more than 1mm/hour.
        heavy_rain_probability: Percentage,
        /// Probability of snow.
        snow_probability: Percentage,
        /// Probability of heavy snow
        ///
        /// Heavy snow is defined as more than 1mm/hour of liquid water equivalent,
        /// or 1cm snow per hour.
        heavy_snow_probability: Percentage,
        /// Probability of hail
        hail_probability: Percentage,
        /// Probability of lightning
        lightning_probability: Percentage,
    },
}

/// Nighttime forecast data
///
/// Nighttime is defined as the period from local dusk to local dawn.
///
/// Fields given "at midnight" are always at 12am (midnight) in the forecast location's local
/// timezone, all others are "during the night", from dusk to dawn.
#[derive(Debug)]
pub struct Night {
    /// Mean wind speed at midnight
    ///
    /// This is the mean speed over the 10 minutes to midnight and is measured at 10m
    /// above ground level.
    pub wind_speed: MetresPerSecond,
    /// Mean wind direction at midnight
    ///
    /// This is the mean direction over the 10 minutes to midnight and is measured at 10m
    /// above ground level.
    pub wind_direction: Degrees,
    /// Mean wind gust speed at midnight
    ///
    /// This is the maximum 3-second mean wind speed over the 10 minutes to midnight and is
    /// measured at 10m above ground level.
    pub wind_gust_speed: MetresPerSecond,
    /// Visibility in metres at midnight
    pub visibility: Metres,
    /// Relative humidity at midnight
    ///
    /// Measured at screen height, about 1.5m above ground level.
    pub relative_humidity: Percentage,
    /// Air pressure at mean sea level at midnight
    pub pressure: Pascals,
    /// The most significant weather conditions
    pub conditions: Conditions,
    /// Minimum air temperature
    ///
    /// Measured at screen height, about 1.5m above ground level.
    pub temperature_minimum: TemperaturePrediction,
    /// Minimum feels-like air temperature
    ///
    /// This is the temperature it feels like taking into account humidity and wind chill but
    /// not radiation.
    pub temperature_feels_like_minimum: TemperaturePrediction,
    /// Probability of any precipitation
    pub precipitation_probability: Percentage,
    /// Probability of rain
    pub rain_probability: Percentage,
    /// Probability of heavy rain
    ///
    /// Heavy rain is defined as more than 1mm/hour.
    pub heavy_rain_probability: Percentage,
    /// Probability of snow.
    pub snow_probability: Percentage,
    /// Probability of heavy snow
    ///
    /// Heavy snow is defined as more than 1mm/hour of liquid water equivalent, or 1cm snow per
    /// hour.
    pub heavy_snow_probability: Percentage,
    /// Probability of hail
    pub hail_probability: Percentage,
    /// Probability of lightning
    pub lightning_probability: Percentage,
}

impl TryFrom<RawDailyForecast> for Daily {
    type Error = Error;

    fn try_from(rf: RawDailyForecast) -> Result<Self, Self::Error> {
        // Check for a key known to be missing in the previous day's data.
        // Perhaps this is not robust?
        let day = if rf.day_max_feels_like_temp.is_none() {
            Day::Past {
                wind_speed: MetresPerSecond(rf.midday_10m_wind_speed),
                wind_direction: Degrees(rf.midday_10m_wind_direction),
                wind_gust_speed: MetresPerSecond(rf.midday_10m_wind_gust),
                visibility: Metres(rf.midday_visibility),
                relative_humidity: Percentage(rf.midday_relative_humidity),
                pressure: Pascals(rf.midday_mslp),
                temperature_maximum: TemperaturePrediction {
                    most_likely: Celsius(rf.day_max_screen_temperature),
                    upper_bound: Celsius(rf.day_upper_bound_max_temp),
                    lower_bound: Celsius(rf.day_lower_bound_max_temp),
                },
                temperature_feels_like_maximum_upper_bound: Celsius(
                    rf.day_upper_bound_max_feels_like_temp,
                ),
                temperature_feels_like_maximum_lower_bound: Celsius(
                    rf.day_lower_bound_max_feels_like_temp,
                ),
            }
        } else {
            Day::Future {
                wind_speed: MetresPerSecond(rf.midday_10m_wind_speed),
                wind_direction: Degrees(rf.midday_10m_wind_direction),
                wind_gust_speed: MetresPerSecond(rf.midday_10m_wind_gust),
                visibility: Metres(rf.midday_visibility),
                relative_humidity: Percentage(rf.midday_relative_humidity),
                pressure: Pascals(rf.midday_mslp),
                uv_index_maximum: UvIndex(rf.max_uv_index.unwrap()),
                conditions: rf.day_significant_weather_code.unwrap().try_into()?,
                temperature_maximum: TemperaturePrediction {
                    most_likely: Celsius(rf.day_max_screen_temperature),
                    upper_bound: Celsius(rf.day_upper_bound_max_temp),
                    lower_bound: Celsius(rf.day_lower_bound_max_temp),
                },
                temperature_feels_like_maximum: TemperaturePrediction {
                    most_likely: Celsius(rf.day_max_feels_like_temp.unwrap()),
                    upper_bound: Celsius(rf.day_upper_bound_max_feels_like_temp),
                    lower_bound: Celsius(rf.day_lower_bound_max_feels_like_temp),
                },
                precipitation_probability: Percentage(rf.day_probability_of_precipitation.unwrap()),
                rain_probability: Percentage(rf.day_probability_of_rain.unwrap()),
                heavy_rain_probability: Percentage(rf.day_probability_of_heavy_rain.unwrap()),
                snow_probability: Percentage(rf.day_probability_of_snow.unwrap()),
                heavy_snow_probability: Percentage(rf.day_probability_of_heavy_snow.unwrap()),
                hail_probability: Percentage(rf.day_probability_of_hail.unwrap()),
                lightning_probability: Percentage(rf.day_probability_of_sferics.unwrap()),
            }
        };

        Ok(Self {
            time: rf.time,
            day,
            night: Night {
                wind_speed: MetresPerSecond(rf.midnight_10m_wind_speed),
                wind_direction: Degrees(rf.midnight_10m_wind_direction),
                wind_gust_speed: MetresPerSecond(rf.midnight_10m_wind_gust),
                visibility: Metres(rf.midnight_visibility),
                relative_humidity: Percentage(rf.midnight_relative_humidity),
                pressure: Pascals(rf.midnight_mslp),
                conditions: rf.night_significant_weather_code.try_into()?,
                temperature_minimum: TemperaturePrediction {
                    most_likely: Celsius(rf.night_min_screen_temperature),
                    upper_bound: Celsius(rf.night_upper_bound_min_temp),
                    lower_bound: Celsius(rf.night_lower_bound_min_temp),
                },
                temperature_feels_like_minimum: TemperaturePrediction {
                    most_likely: Celsius(rf.night_min_feels_like_temp),
                    upper_bound: Celsius(rf.night_upper_bound_min_feels_like_temp),
                    lower_bound: Celsius(rf.night_lower_bound_min_feels_like_temp),
                },
                precipitation_probability: Percentage(rf.night_probability_of_precipitation),
                rain_probability: Percentage(rf.night_probability_of_rain),
                heavy_rain_probability: Percentage(rf.night_probability_of_heavy_rain),
                snow_probability: Percentage(rf.night_probability_of_snow),
                heavy_snow_probability: Percentage(rf.night_probability_of_heavy_snow),
                hail_probability: Percentage(rf.night_probability_of_hail),
                lightning_probability: Percentage(rf.night_probability_of_sferics),
            },
        })
    }
}
