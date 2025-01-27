use crate::learning::MergedData;

pub fn distance(a: &MergedData, b: &MergedData) -> f32 {
    (((a.hour - b.hour) as f32 / 24.0).powf(2.0)
        + ((a.day - b.day) as f32 / 7.0).powf(2.0)
        + ((a.temperature - b.temperature) / 40.0).powf(2.0)
        + ((a.precipitation - b.precipitation) / 100.0).powf(2.0)
        + ((a.wind_speed - b.wind_speed) / 100.0).powf(2.0))
    .sqrt()
}
