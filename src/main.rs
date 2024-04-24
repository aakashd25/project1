use std::error::Error;
use std::fs::File;
use std::collections::HashMap;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PatientRecord {
    rest_bp: u32,
    chest_pain: u32,
    thalassemia: u32,
    age: u32,
    fasting_bs: u32,
    max_hr: u32,
    exercise_angina: u32,
    gender: u32,
    st_slope: u32,
    cholesterol: u32,
    st_depression: f32,
    rest_ecg: u32,
    num_vessels: u32,
    diagnosis: u32,
}

fn read_csv(file_path: &str) -> Result<Vec<PatientRecord>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut records = Vec::new();

    for result in reader.deserialize::<PatientRecord>() {
        let record: PatientRecord = result?;
        records.push(record);
    }

    Ok(records)
}

fn main() {
    if let Ok(records) = read_csv("heart_disease.csv") {
        // Preprocessing steps go here
        println!("{:?}", records);
    } else {
        eprintln!("Error reading CSV file");
    }
}
