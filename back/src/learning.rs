use crate::downloader::Value;
use anyhow::Error;
use chrono::{DateTime, Utc};
use log::info;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelBridge;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

pub fn learn() {
    let data = load_data().unwrap();
    info!("Writing data to file..");
    fs::write("./velov_training_data.json", serde_json::to_string(&data).unwrap()).unwrap();
    info!("Training data written to velov_training_data.json !");
}

fn load_data() -> Result<Vec<UsefulData>, Error> {
    let original_data_len = Arc::new(Mutex::new(0));
    let compliant_data_len = Arc::new(Mutex::new(0));

    let read_dir_length = fs::read_dir("./velov_datas")?.count();
    let visited = Mutex::new(0);

    let mut data = fs::read_dir("./velov_datas")?
        .par_bridge()
        .map(|e| e.unwrap())
        .flat_map(|entry| {
            info!(
                "{}% Reading {}",
                ((*visited.lock().unwrap() as f32 / read_dir_length as f32) * 10000.0).round()
                    / 100.0,
                entry.path().display()
            );

            let file = File::open(entry.path()).unwrap();
            let reader = BufReader::new(file);

            let file_data: Vec<Value> = serde_json::from_reader(reader).unwrap();
            *original_data_len.lock().unwrap() += file_data.len();

            let useful_data = file_data
                .iter()
                .map(|value| UsefulData {
                    id: value.number as u32,
                    date: value.horodate,
                    capacity: value.total_stands.capacity as u32,
                    bikes: value.total_stands.availabilities.bikes as u32,
                    stands: value.total_stands.availabilities.stands as u32,
                })
                .filter(|value| value.capacity == (value.bikes + value.stands))
                .collect::<Vec<UsefulData>>();

            *compliant_data_len.lock().unwrap() += useful_data.len();

            *visited.lock().unwrap() += 1;

            useful_data
        })
        .collect::<Vec<_>>();

    let original_data_len = *original_data_len.lock().unwrap();
    let compliant_data_len = *compliant_data_len.lock().unwrap();

    info!(
        "Compliant data : {}/{} ({}%)",
        compliant_data_len,
        original_data_len,
        (compliant_data_len as f32 / original_data_len as f32) * 100.0
    );

    info!("Deduping..");
    let before_dedup = data.len();
    data.dedup_by(|a, b| a.date == b.date);
    info!("Deduped {} entries !", before_dedup - data.len());

    info!("Sorting data");
    data.sort_by(|a, b| a.date.cmp(&b.date));
    info!("Done !");

    Ok(data)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UsefulData {
    id: u32,
    date: DateTime<Utc>,
    capacity: u32,
    bikes: u32,
    stands: u32,
}
