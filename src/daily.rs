use crate::parse::RawDailyForecast;
use crate::{
    Celsius, Conditions, Degrees, Error, Metres, MetresPerSecond, Pascals, Percentage, UvIndex,
};

#[derive(Debug)]
pub struct Daily {
    /// Time at which this forecast is valid.
    pub time: jiff::Zoned,
    pub day: Day,
    pub night: Night,
}

#[derive(Debug)]
pub struct TemperaturePrediction {
    pub most_likely: Celsius,
    pub upper_bound: Celsius,
    pub lower_bound: Celsius,
}
#[derive(Debug)]
pub enum Day {
    Past {
        wind_speed: MetresPerSecond,
        wind_direction: Degrees,
        wind_gust_speed: MetresPerSecond,
        visibility: Metres,
        relative_humidity: Percentage,
        pressure: Pascals,
        temperature_maximum: TemperaturePrediction,
        temperature_feels_like_maximum_upper_bound: Celsius,
        temperature_feels_like_maximum_lower_bound: Celsius,
    },
    Future {
        wind_speed: MetresPerSecond,
        wind_direction: Degrees,
        wind_gust_speed: MetresPerSecond,
        visibility: Metres,
        relative_humidity: Percentage,
        pressure: Pascals,
        uv_index_maximum: UvIndex,
        conditions: Conditions,
        temperature_maximum: TemperaturePrediction,
        temperature_feels_like_maximum: TemperaturePrediction,
        precipitation_probability: Percentage,
        rain_probability: Percentage,
        heavy_rain_probability: Percentage,
        snow_probability: Percentage,
        heavy_snow_probability: Percentage,
        hail_probability: Percentage,
        lightning_probability: Percentage,
    },
}

#[derive(Debug)]
pub struct Night {
    pub wind_speed: MetresPerSecond,
    pub wind_direction: Degrees,
    pub wind_gust_speed: MetresPerSecond,
    pub visibility: Metres,
    pub relative_humidity: Percentage,
    pub pressure: Pascals,
    pub conditions: Conditions,
    pub temperature_minimum: TemperaturePrediction,
    pub temperature_feels_like_minimum: TemperaturePrediction,
    pub precipitation_probability: Percentage,
    pub rain_probability: Percentage,
    pub heavy_rain_probability: Percentage,
    pub snow_probability: Percentage,
    pub heavy_snow_probability: Percentage,
    pub hail_probability: Percentage,
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
