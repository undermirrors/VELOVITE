use crate::downloader::{Value, WeatherData};
use crate::models::DetailedStation;
use crate::utils::distance;
use crate::{establish_connection, schema};
use chrono::{DateTime, Datelike, NaiveDate, Timelike, Utc};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use log::{info, warn};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::ParallelBridge;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::sync::{Arc, Mutex};

const BENCHMARK_PERCENTAGE: f32 = 20.0;

pub fn benchmark() {
    let mut connection = establish_connection();

    let mut data = read_merged_data_from_file("merged_data");

    info!(
        "🗑️ Removing {}% of the data of each station..",
        BENCHMARK_PERCENTAGE
    );
    let mut removed_data: HashMap<u32, Vec<MergedData>> = HashMap::new();

    let mut len_removed = 0;
    for (key, value) in data.iter_mut() {
        let len = value.len();
        let to_remove = (len as f32 * BENCHMARK_PERCENTAGE / 100.0) as usize;
        removed_data.insert(*key, value.split_off(len - to_remove));
        value.truncate(len - to_remove);
        len_removed = to_remove;
    }
    info!(
        "🗑️ Removed {}% of the data of each station!",
        BENCHMARK_PERCENTAGE
    );
    //display the length of the removed data
    info!("📊 Removed data length :{}", len_removed);

    info!("📊 Benchmarking..");
    let result: HashMap<u32, Vec<u32>> = data
        .par_iter()
        .map(|(key, value)| {
            info!("🔍 Benchmarking station {}..", key);
            let station_result: Vec<u32> = removed_data
                .get(key)
                .unwrap()
                .par_iter()
                .map(|wanted| {
                    let wanted_point = MergedData {
                        id: wanted.id,
                        hour: wanted.hour,
                        day: wanted.day,
                        month: wanted.month,
                        week_day: wanted.week_day,
                        holidays: wanted.holidays,
                        free_stands: wanted.free_stands,
                        available_bikes: wanted.available_bikes,
                        precipitation: wanted.precipitation,
                        temperature: wanted.temperature,
                        wind_speed: wanted.wind_speed,
                    };

                    let nearest_data = value
                        .par_iter()
                        .min_by(|a, b| {
                            distance(a, &wanted_point)
                                .partial_cmp(&distance(b, &wanted_point))
                                .unwrap()
                        })
                        .unwrap();

                    nearest_data
                        .available_bikes
                        .abs_diff(wanted_point.available_bikes)
                })
                .collect();

            (*key, station_result)
        })
        .collect();
    info!("✅ Benchmark done!");

    info!("📊 Results :");
    // Display for each station, the average, the median, the min and the max of the distance
    // then the average of all the stations
    let mut total = 0.0;
    let mut total_len = 0;
    let mut all_distances: Vec<f32> = Vec::new();

    let mut wtr = csv::Writer::from_path("benchmark_results.csv").unwrap();
    wtr.write_record(["Station ID", "Average", "Median", "Min", "Max"])
        .unwrap();

    for (key, value) in result.iter() {
        use schema::station::dsl::station;
        let station_details = match station
            .select(DetailedStation::as_select())
            .filter(schema::station::id.eq(*key as i32))
            .limit(1)
            .first::<DetailedStation>(&mut connection)
        {
            Ok(details) => details,
            Err(_) => {
                warn!("❌ Station {} not found in the database", key);
                continue;
            }
        };

        if value.is_empty() {
            continue;
        }
        let average =
            value.iter().sum::<u32>() as f32 / value.len() as f32 / station_details.capacity as f32
                * 100.0;
        let median = {
            let mut sorted = value.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let len = sorted.len();
            if len % 2 == 0 {
                (sorted[len / 2] + sorted[len / 2 - 1]) as f32 / 2.0
            } else {
                sorted[len / 2] as f32
            }
        } / station_details.capacity as f32
            * 100.0;
        let min = *(value
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()) as f32
            / station_details.capacity as f32
            * 100.0;
        let max = *(value
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()) as f32
            / station_details.capacity as f32
            * 100.0;

        info!(
            "🆔 {} : Average : {} Median : {} Min : {} Max : {}",
            key, average, median, min, max
        );

        wtr.write_record(&[
            key.to_string(),
            average.to_string(),
            median.to_string(),
            min.to_string(),
            max.to_string(),
        ])
        .unwrap();

        total += average * value.len() as f32;
        total_len += value.len();
        all_distances.extend(
            value
                .iter()
                .map(|v| *v as f32 / station_details.capacity as f32 * 100.0),
        );
    }

    wtr.flush().unwrap();

    let average = total / total_len as f32;
    let main_median = {
        all_distances.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less));
        let len = all_distances.len();
        if len % 2 == 0 {
            (all_distances[len / 2] + all_distances[len / 2 - 1]) / 2.0
        } else {
            all_distances[len / 2]
        }
    };
    let main_min = *all_distances
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&0.0);
    let main_max = *all_distances
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&0.0);

    info!("📊 Average of all stations : {}", average);
    info!("📊 Median of all stations : {}", main_median);
    info!("📊 Min of all stations : {}", main_min);
    info!("📊 Max of all stations : {}", main_max);
}
/// Merges the Velov, weather, and school holidays data into a single dataset. and writes it to a file.
///
/// # Returns
///
/// * nothing
///
/// # Panics
///
/// Panics if the weather data is missing for a given date.
///
/// # Examples
///
/// ```rust
/// merge_data();
/// ```
pub fn merge_data() {
    info!("📥 Loading school holidays data..");
    let school_holidays: Vec<SchoolHolidays> = serde_json::from_str(
        &fs::read_to_string("school_holidays.json")
            .unwrap()
            .to_string(),
    )
    .unwrap();
    info!("✅ School holidays data loaded!");

    info!("📥 Loading weather data..");
    let weather: HashMap<DateTime<Utc>, WeatherData> =
        serde_json::from_str(&fs::read_to_string("weather.json").unwrap().to_string()).unwrap();
    info!("✅ Weather data loaded!");

    info!("📥 Loading velov training data..");
    let file = File::open("velov_training_data.json").unwrap();
    let reader = BufReader::new(file);
    let velov: Vec<UsefulData> = serde_json::from_reader(reader).unwrap();
    info!("✅ Velov training data loaded!");

    info!("🔄 Merging data..");
    let merged: Vec<MergedData> = velov
        .par_iter()
        .map(|velov_data| {
            let date = velov_data.date;
            let school_holiday = school_holidays.iter().any(|holiday| {
                date.naive_local().date() >= holiday.start
                    && date.naive_local().date() <= holiday.end
            });

            let weather_data = weather
                .get(
                    &date
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap()
                        .with_nanosecond(0)
                        .unwrap(),
                )
                .unwrap_or_else(|| panic!("❌ No weather data for {}", date));
            let precipitation_data = weather_data.precipitation;
            let temperature_data = weather_data.temperature_2m;
            let wind_speed_data = weather_data.wind_speed_10m;

            MergedData {
                id: velov_data.id,
                hour: date.time().hour(),
                day: date.date_naive().day(),
                month: date.date_naive().month(),
                week_day: date.date_naive().weekday().number_from_monday(),
                holidays: school_holiday,
                free_stands: velov_data.stands,
                available_bikes: velov_data.bikes,
                precipitation: precipitation_data,
                temperature: temperature_data,
                wind_speed: wind_speed_data,
            }
        })
        .collect();

    info!("✅ Data merged!");
    info!("🔄 Hashmapping the data");

    // merge the data per id in a hashmap
    let hashmapped: HashMap<u32, Vec<MergedData>> =
        merged.iter().fold(HashMap::new(), |mut acc, val| {
            acc.entry(val.id)
                .and_modify(|v: &mut Vec<MergedData>| v.push(val.clone()))
                .or_default();
            acc
        });

    info!("✅ Data merged!");

    write_merged_data_to_file("merged_data", hashmapped);
}

/// Filters the Velov data to keep only the useful data and writes it to a file.
///
/// # Returns
///
/// * nothing
///
/// # Examples
///
/// ```rust
/// filter_velov_data();
/// ```
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
        .filter(|entry| entry.file_name() != ".gitkeep")
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
                for (id, value) in original_stats_tmp.iter() {
                    original_stats_guard
                        .entry(*id)
                        .and_modify(|e| *e += value)
                        .or_insert(*value);
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
    let original_stats = original_stats.lock().unwrap();
    for (id, _) in compliant_stats.iter() {
        info!(
            "🆔 {} : {}/{} ({}%)",
            id,
            compliant_stats.get(id).unwrap(),
            original_stats.get(id).unwrap(),
            (*compliant_stats.get(id).unwrap() as f32 / *original_stats.get(id).unwrap() as f32)
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

    info!("🔄 Serializing data..");
    let file = File::create("./velov_training_data.json").unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &data).unwrap();
    info!("✅ velov_training_data.json written!");
}

/// Reads the merged data from the files and returns it as a hashmap.
///
/// # Arguments
///
/// * `path` - The path to the directory containing the files.
///
/// # Returns
///
/// * A hashmap containing the merged data.
///
/// # Examples
///
/// ```rust
/// let data = read_merged_data_from_file("merged_data");
/// ```
pub fn read_merged_data_from_file(path: &str) -> HashMap<u32, Vec<MergedData>> {
    info!("📖 Reading data from files..");
    // List all files in the directory
    let mut files: Vec<_> = fs::read_dir(path)
        .unwrap()
        .map(|f| f.unwrap())
        .filter(|entry| entry.file_name() != ".gitkeep")
        .collect();
    files.sort_by_key(|a| a.path());

    let data = Arc::new(Mutex::new(HashMap::<u32, Vec<MergedData>>::new()));

    files.par_iter().for_each(|file| {
        let file = File::open(file.path()).unwrap();
        let reader = BufReader::new(file);
        let file_data: Vec<MergedData> = serde_json::from_reader(reader).unwrap();

        let mut data_guard = data.lock().unwrap();
        data_guard.insert(file_data.first().unwrap().id, file_data);
    });
    info!("✅ Data read from files!");
    Arc::try_unwrap(data).unwrap().into_inner().unwrap()
}

/// Writes the merged data to files.
///
/// # Arguments
///
/// * `path` - The path to the directory where the files will be written.
///
/// # Returns
///
/// * nothing
///
/// # Examples
///
/// ```rust
/// write_merged_data_to_file("merged_data", data);
/// ```
fn write_merged_data_to_file(path: &str, data: HashMap<u32, Vec<MergedData>>) {
    info!("✍️ Splitting data into files..");
    data.par_iter().for_each(|(key, value)| {
        let file_path = format!("{}/{}.json", path, key);
        let file = File::create(file_path).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, value).unwrap();
    });
    info!("✅ Data written to files!");
}

/// Data structure representing the availability of bikes and free stands at a station.
///
/// # Fields
///
/// * `id` - The ID of the station.
/// * `date` - The date and time of the data.
/// * `capacity` - The total capacity of the station.
/// * `bikes` - The number of available bikes.
/// * `stands` - The number of free stands.
///
/// # Examples
///
/// ```rust
/// let data = UsefulData {
///    id: 1,
///    date: Utc::now(),
///    capacity: 20,
///    bikes: 10,
///    stands: 10,
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
struct UsefulData {
    id: u32,
    date: DateTime<Utc>,
    capacity: u32,
    bikes: u32,
    stands: u32,
}

/// Data structure representing the availability of bikes and free stands at a station.
///
/// # Fields
///
/// * `id` - The ID of the station.
/// * `hour` - The hour of the data.
/// * `day` - The day of the data.
/// * `month` - The month of the data.
/// * `week_day` - The day of the week of the data.
/// * `holidays` - A flag indicating whether the data is on a holiday.
/// * `free_stands` - The number of free stands available.
/// * `available_bikes` - The number of bikes available.
/// * `precipitation` - The amount of precipitation.
/// * `temperature` - The temperature.
/// * `wind_speed` - The wind speed.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MergedData {
    pub id: u32,
    pub hour: u32,
    pub day: u32,
    pub month: u32,
    pub week_day: u32,
    pub holidays: bool,
    pub free_stands: u32,
    pub available_bikes: u32,
    pub precipitation: f32,
    pub temperature: f32,
    pub wind_speed: f32,
}

/// Data structure representing the school holidays.
///
/// # Fields
///
/// * `start` - The start date of the holidays.
/// * `end` - The end date of the holidays.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchoolHolidays {
    pub start: NaiveDate,
    pub end: NaiveDate,
}
