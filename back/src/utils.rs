use crate::learning::MergedData;

const HOUR_COEF: f32 = 1.0;
const DAY_COEF: f32 = 1.0;
const TEMPERATURE_COEF: f32 = 1.0;
const PRECIPITATION_COEF: f32 = 1.0;
const WIND_SPEED_COEF: f32 = 1.0;

pub fn distance(a: &MergedData, b: &MergedData) -> f32 {
    ((HOUR_COEF * ((a.hour - b.hour) as f32 / 24.0).powf(2.0))
        + (DAY_COEF * ((a.day - b.day) as f32 / 7.0).powf(2.0))
        + (TEMPERATURE_COEF * ((a.temperature - b.temperature) / 40.0).powf(2.0))
        + (PRECIPITATION_COEF * ((a.precipitation - b.precipitation) / 100.0).powf(2.0))
        + (WIND_SPEED_COEF * ((a.wind_speed - b.wind_speed) / 100.0).powf(2.0)))
    .sqrt()
}
