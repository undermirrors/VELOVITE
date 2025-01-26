use crate::downloader::Value;
use chrono::{DateTime, NaiveDate, Utc};
use log::info;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelBridge;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

pub fn filter_velov_data() {
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
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        original_stats_tmp.entry(value.number as u32)
                    {
                        e.insert(1);
                    } else {
                        *original_stats_tmp.get_mut(&(value.number as u32)).unwrap() += 1;
                    }

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
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        original_stats_guard.entry(*id)
                    {
                        e.insert(1);
                    } else {
                        *original_stats_guard.get_mut(id).unwrap() += 1;
                    }
                }
            }

            *compliant_data_len.lock().unwrap() += useful_data.len();

            *visited.lock().unwrap() += 1;

            useful_data
        })
        .collect::<Vec<_>>();

    // Display some stats
    //filter stats per id
    let mut compliant_stats: HashMap<u32, u32> = std::collections::HashMap::new();
    for value in data.iter() {
        if let std::collections::hash_map::Entry::Vacant(e) = compliant_stats.entry(value.id) {
            e.insert(1);
        } else {
            *compliant_stats.get_mut(&value.id).unwrap() += 1;
        }
    }

    let original_stats = original_stats.lock().unwrap();
    //display the stats
    info!("📊 Stats per id :");
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
    data.dedup_by(|a, b| a.date == b.date);
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SchoolHolidays {
    start: NaiveDate,
    end: NaiveDate,
}
