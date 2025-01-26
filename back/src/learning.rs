use crate::downloader::{Value, WeatherRoot};
use chrono::{DateTime, Datelike, NaiveDate, Timelike, Utc};
use log::{error, info};
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelBridge;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

pub fn merged_data() {
    info!("📥 Loading school holidays data..");
    let school_holidays: Vec<SchoolHolidays> = serde_json::from_str(
        &fs::read_to_string("school_holidays.json")
            .unwrap()
            .to_string(),
    )
    .unwrap();
    info!("✅ School holidays data loaded!");

    info!("📥 Loading weather data..");
    let weather: WeatherRoot =
        serde_json::from_str(&fs::read_to_string("weather.json").unwrap().to_string()).unwrap();
    info!("✅ Weather data loaded!");

    info!("📥 Loading velov training data..");
    let velov: Vec<UsefulData> = serde_json::from_str(
        &fs::read_to_string("velov_training_data.json")
            .unwrap()
            .to_string(),
    )
    .unwrap();
    info!("✅ Velov training data loaded!");

    info!("🔄 Merging data..");
    let mut merged_data: Vec<MergedData> = Vec::new();
    velov.iter().for_each(|velov_data| {
        let date = velov_data.date;
        let school_holiday = school_holidays.iter().any(|holiday| {
            date.naive_local().date() >= holiday.start && date.naive_local().date() <= holiday.end
        });

        let weather_index = match weather.hourly.time.iter().position(|time| {
            let time_without_seconds = time.with_second(0).unwrap().with_nanosecond(0).unwrap();
            let date_without_seconds = date.with_second(0).unwrap().with_nanosecond(0).unwrap();
            time_without_seconds == date_without_seconds
        }) {
            Some(index) => index,
            None => {
                error!("❌ Failed to find weather data for date: {}", date);
                return;
            }
        };

        let precipitation_data = weather.hourly.precipitation[weather_index];
        let temperature_data = weather.hourly.temperature_2m[weather_index];
        let wind_speed_data = weather.hourly.wind_speed_10m[weather_index];

        merged_data.push(MergedData {
            id: velov_data.id,
            hour: date.time().hour() as u8,
            day: date.date_naive().day() as u8,
            month: date.date_naive().month() as u8,
            week_day: date.date_naive().weekday().number_from_monday() as u8,
            holidays: school_holiday,
            free_stands: velov_data.stands,
            precipitation: precipitation_data,
            temperature: temperature_data,
            wind_speed: wind_speed_data,
        });
    });
    info!("✅ Data merged!");

    //Serializing data..");
    info!("🔄 Serializing data..");
    let json = serde_json::to_string(&merged_data).unwrap();
    info!("✍️ Writing merged data to file..");
    fs::write("./merged_data.json", json).unwrap();
    info!("✅ Merged data written to merged_data.json!");
}
pub fn filter_velov_data() {
    info!("🧹 Filtering velov data...");
    let original_stats = Arc::new(Mutex::new(HashMap::<u32, u32>::new()));
    let original_data_len = Arc::new(Mutex::new(0));
    let compliant_data_len = Arc::new(Mutex::new(0));

    let read_dir_length = fs::read_dir("./velov_datas").unwrap().count();
    let visited = Mutex::new(0);

    let mut data = fs::read_dir("./velov_datas")
        .unwrap()
        .par_bridge()
        .map(|e| e.unwrap())
        .flat_map(|entry| {
            info!(
                "📖 {}% Reading {}",
                ((*visited.lock().unwrap() as f32 / read_dir_length as f32) * 10000.0).round()
                    / 100.0,
                entry.path().display()
            );

            let file = File::open(entry.path()).unwrap();
            let reader = BufReader::new(file);

            let file_data: Vec<Value> = serde_json::from_reader(reader).unwrap();
            *original_data_len.lock().unwrap() += file_data.len();

            let mut original_stats_tmp = HashMap::<u32, u32>::new();

            let useful_data = file_data
                .iter()
                .map(|value| {
                    original_stats_tmp
                        .entry(value.number as u32)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);

                    UsefulData {
                        id: value.number as u32,
                        date: value.horodate,
                        capacity: value.total_stands.capacity as u32,
                        bikes: value.total_stands.availabilities.bikes as u32,
                        stands: value.total_stands.availabilities.stands as u32,
                    }
                })
                .filter(|value| value.capacity == (value.bikes + value.stands))
                .collect::<Vec<UsefulData>>();

            {
                let mut original_stats_guard = original_stats.lock().unwrap();
                for (id, _) in original_stats_tmp.iter() {
                    original_stats_guard
                        .entry(*id)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }

            *compliant_data_len.lock().unwrap() += useful_data.len();

            *visited.lock().unwrap() += 1;

            useful_data
        })
        .collect::<Vec<_>>();

    // Display some stats
    //filter stats per id
    let mut compliant_stats: HashMap<u32, u32> = HashMap::new();
    for value in data.iter() {
        compliant_stats
            .entry(value.id)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    //display the stats
    info!("📊 Stats per id :");
    for (id, _) in compliant_stats.iter() {
        info!(
            "🆔 {} : {}/{} ({}%)",
            id,
            compliant_stats.get(id).unwrap(),
            original_stats.lock().unwrap().get(id).unwrap(),
            (*compliant_stats.get(id).unwrap() as f32
                / *original_stats.lock().unwrap().get(id).unwrap() as f32)
                * 100.0
        );
    }

    let original_data_len = *original_data_len.lock().unwrap();
    let compliant_data_len = *compliant_data_len.lock().unwrap();

    info!(
        "✅ Compliant data : {}/{} ({}%)",
        compliant_data_len,
        original_data_len,
        (compliant_data_len as f32 / original_data_len as f32) * 100.0
    );

    info!("🧹 Deduping..");
    let before_dedup = data.len();
    data.dedup_by(|a, b| a.date == b.date && a.id == b.id);
    info!("🗑️ Deduped {} entries !", before_dedup - data.len());

    info!("🔄 Sorting data");
    data.sort_by(|a, b| a.date.cmp(&b.date));
    info!("✅ Done !");

    info!("✍️ Writing data to file..");
    fs::write(
        "./velov_training_data.json",
        serde_json::to_string(&data).unwrap(),
    )
    .unwrap();
    info!("✅ Training data written to velov_training_data.json !");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UsefulData {
    id: u32,
    date: DateTime<Utc>,
    capacity: u32,
    bikes: u32,
    stands: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MergedData {
    id: u32,
    hour: u8,
    day: u8,
    month: u8,
    week_day: u8,
    holidays: bool,
    free_stands: u32,
    precipitation: f32,
    temperature: f32,
    wind_speed: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SchoolHolidays {
    start: NaiveDate,
    end: NaiveDate,
}
