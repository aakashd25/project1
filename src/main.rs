use std::error::Error;
use std::fs::File;
use std::collections::HashSet;
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
    st_depression: f64,
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

// Calculate Jaccard similarity between two sets
fn jaccard_similarity(set1: &HashSet<u32>, set2: &HashSet<u32>) -> f64 {
    let intersection_size = set1.intersection(&set2).count() as f64;
    let union_size = set1.len() as f64 + set2.len() as f64 - intersection_size;
    if union_size == 0.0 {
        0.0
    } else {
        intersection_size / union_size
    }
}

// Function to calculate Euclidean distance between two vectors
fn euclidean_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    if vec1.len() != vec2.len() {
        panic!("Vector lengths must be equal");
    }
    let squared_diff: f64 = vec1.iter().zip(vec2.iter()).map(|(x, y)| (x - y).powi(2)).sum();
    squared_diff.sqrt()
}

// Function to extract symptom sets from patient records
fn extract_symptoms(records: &[PatientRecord], field_name: &str) -> HashSet<u32> {
    let mut symptoms = HashSet::new();
    for record in records {
        let value = match field_name {
            "rest_bp" => record.rest_bp,
            "chest_pain" => record.chest_pain,
            "thalassemia" => record.thalassemia,
            "age" => record.age,
            "fasting_bs" => record.fasting_bs,
            "max_hr" => record.max_hr,
            "exercise_angina" => record.exercise_angina,
            "gender" => record.gender,
            "st_slope" => record.st_slope,
            "cholesterol" => record.cholesterol,
            "st_depression" => record.st_depression as u32, // Convert to u32 if needed
            "rest_ecg" => record.rest_ecg,
            "num_vessels" => record.num_vessels,
            "diagnosis" => record.diagnosis,
            _ => panic!("Unknown field"),
        };
        symptoms.insert(value);
    }
    symptoms
}

fn main() {
    if let Ok(records) = read_csv("heart_disease.csv") {
        // Preprocessing steps go here
        println!("{:?}", records);
    } else {
        eprintln!("Error reading CSV file");
    }

    if let Ok(records) = read_csv("heart_disease.csv") {
        // Extract symptom sets for chest_pain and thalassemia
        let chest_pain_symptoms = extract_symptoms(&records, "chest_pain");
        let thalassemia_symptoms = extract_symptoms(&records, "thalassemia");

        // Calculate Jaccard similarity between chest_pain and thalassemia symptoms
        let jaccard_sim = jaccard_similarity(&chest_pain_symptoms, &thalassemia_symptoms);
        println!("Jaccard similarity between chest_pain and thalassemia: {}", jaccard_sim);
    } else {
        eprintln!("Error reading CSV file");
    }

    let vec1 = [1.0, 2.0, 3.0];
    let vec2 = [4.0, 5.0, 6.0];
    println!("Euclidean distance: {}", euclidean_distance(&vec1, &vec2));
}
