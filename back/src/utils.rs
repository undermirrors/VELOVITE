/// Calculates the distance between two `MergedData` points using a weighted Euclidean distance formula.
///
/// # Parameters
///
/// - `a`: A reference to the first `MergedData` point.
/// - `b`: A reference to the second `MergedData` point.
///
/// # Returns
///
/// A `f32` value representing the distance between the two `MergedData` points.
///
/// The distance is calculated using the following formula:
///
/// ```text
/// distance = sqrt(
///     HOUR_COEF * ((a.hour - b.hour) / 24.0)^2 +
///     DAY_COEF * ((a.day - b.day) / 7.0)^2 +
///     TEMPERATURE_COEF * ((a.temperature - b.temperature) / 40.0)^2 +
///     PRECIPITATION_COEF * ((a.precipitation - b.precipitation) / 100.0)^2 +
///     WIND_SPEED_COEF * ((a.wind_speed - b.wind_speed) / 100.0)^2
/// )
/// ```
///
/// The coefficients `HOUR_COEF`, `DAY_COEF`, `TEMPERATURE_COEF`, `PRECIPITATION_COEF`, and `WIND_SPEED_COEF`
/// are used to weight the respective components of the distance calculation.
use crate::learning::MergedData;

const HOUR_COEF: f32 = 1.0;
const DAY_COEF: f32 = 1.0;
const TEMPERATURE_COEF: f32 = 1.0;
const PRECIPITATION_COEF: f32 = 1.0;
const WIND_SPEED_COEF: f32 = 1.0;
const MONTH_COEF: f32 = 1.0;
const WEEK_DAY_COEF: f32 = 1.0;
const HOLIDAYS_COEF: f32 = 1.0;

pub fn distance(a: &MergedData, b: &MergedData) -> f32 {
    let a_time = (a.hour * 60 + a.minute) as f32 / 1440.0;
    let b_time = (b.hour * 60 + b.minute) as f32 / 1440.0;

    ((HOUR_COEF * (a_time - b_time).powf(2.0))
        + (DAY_COEF * ((a.day - b.day) as f32 / 7.0).powf(2.0))
        + (TEMPERATURE_COEF * ((a.temperature - b.temperature) / 40.0).powf(2.0))
        + (PRECIPITATION_COEF * ((a.precipitation - b.precipitation) / 100.0).powf(2.0))
        + (WIND_SPEED_COEF * ((a.wind_speed - b.wind_speed) / 100.0).powf(2.0))
        + (MONTH_COEF * ((a.month - b.month) as f32 / 12.0).powf(2.0))
        + (WEEK_DAY_COEF * ((a.week_day - b.week_day) as f32 / 7.0).powf(2.0))
        + (HOLIDAYS_COEF * ((a.holidays as i32 - b.holidays as i32) as f32 / 1.0).powf(2.0)))
    .sqrt()
}
