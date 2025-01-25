use crate::downloader::Value;
use anyhow::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::BufReader;

pub fn learn() {
    let data = load_data().unwrap();
    fs::write("./data2.json", serde_json::to_string(&data).unwrap()).unwrap();
}

fn load_data() -> Result<Vec<UsefulData>, Error> {
    let mut original_data_len = 0;
    let mut compliant_data_len = 0;
    
    let mut all_data = vec![];
    
    for entry in fs::read_dir("./data")?.flatten() {
        println!("Reading {}", entry.path().display());

        let file = File::open(entry.path())?;
        let reader = BufReader::new(file);

        let file_data: Vec<Value> = serde_json::from_reader(reader)?;
        original_data_len += file_data.len();

        let mut useful_data = file_data
            .iter()
            .map(|value| UsefulData {
                id: value.number as u32,
                date: value.horodate,
                capacity: value.total_stands.capacity as u32,
                bikes: value.total_stands.availabilities.bikes as u32,
                stands: value.total_stands.availabilities.stands as u32,
            })
            .filter(|value| {
                value.capacity == (value.bikes + value.stands)
            })
            .collect::<Vec<UsefulData>>();
        
        compliant_data_len += useful_data.len();
        
        all_data.append(&mut useful_data);
    }
   

    println!(
        "Compliant data : {}/{} ({}%)",
        compliant_data_len,
        original_data_len,
        (compliant_data_len as f32 / original_data_len as f32) * 100.0
    );

    Ok(all_data)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UsefulData {
    id: u32,
    date: DateTime<Utc>,
    capacity: u32,
    bikes: u32,
    stands: u32,
}
