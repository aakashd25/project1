// DS 210 Final Project - Rust
// Packages
mod cluster;

pub extern crate csv;
pub extern crate rand;

pub use rand::prelude::*;
pub use std::error::Error;
pub use std::fs::File;
pub use std::path::Path;
pub use csv::ReaderBuilder;


// Define a struct to represent a patient
#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub struct Patient {
    pub features: Vec<f64>,
    pub diagnosis: u8,
}

// Open and Read Heart Disease Dataset
pub fn load_and_prepare_data(file_path: &str) -> Result<Vec<Patient>, csv::Error> {
    let mut rdr = csv::ReaderBuilder::new().from_path(file_path)?;
    let mut patients = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let features: Vec<f64> = record
            .iter()
            .take(record.len() - 1)
            .map(|v| v.parse().unwrap())
            .collect();
        let diagnosis: u8 = record[record.len() - 1].parse().unwrap();
        let patient = Patient { features, diagnosis };
        patients.push(patient);
    }

    Ok(patients)
}

// Primary Analysis: Split the data into patients diagnosed with heart disease (1) and those who are not (0)
pub fn split_diagnosis(patients: &[Patient]) -> (Vec<Patient>, Vec<Patient>) {
    let mut diagnosed_with_disease = Vec::new();
    let mut not_diagnosed_with_disease = Vec::new();

    for patient in patients {
        if patient.diagnosis == 1 {
            diagnosed_with_disease.push(patient.clone());
        } else {
            not_diagnosed_with_disease.push(patient.clone());
        }
    }

    (diagnosed_with_disease, not_diagnosed_with_disease)
}

// Calculate median for all symptoms in each group and output the values
pub fn calculate_median(patients: &[Patient]) -> Vec<f64> {
    let num_symptoms = patients[0].features.len();
    let mut medians = vec![0.0; num_symptoms];

    for i in 0..num_symptoms {
        let mut values: Vec<f64> = patients.iter().map(|p| p.features[i]).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap()); // Sort the values

        let median = if values.len() % 2 == 0 {
            let mid = values.len() / 2;
            (values[mid - 1] + values[mid]) / 2.0 // Calculate median for even number of elements
        } else {
            values[values.len() / 2] // Calculate median for odd number of elements
        };

        medians[i] = median;
    }

    medians
}


// Function to find the k best representatives from each cluster
pub fn find_best_representatives(k: usize, centroids: &[Vec<f64>], clusters: &[Vec<Patient>]) -> Vec<(Vec<f64>, u8)> {
    let mut best_representatives = Vec::new();

    // Iterate over each cluster
    for cluster in clusters {
        let mut min_avg_distance = f64::MAX;
        let mut best_representative = Vec::new();
        let mut best_diagnosis = 0;

        // Iterate over each point in the cluster
        for point in cluster {
            let mut total_distance = 0.0;

            // Calculate the distance of the point to its centroid
            for (feature, centroid_feature) in point.features.iter().zip(centroids[clusters.iter().position(|x| x == cluster).unwrap()].iter()) {
                total_distance += (feature - centroid_feature).abs();
            }

            // Calculate the average distance
            let avg_distance = total_distance / point.features.len() as f64;

            // Update the best representative if the average distance is lower
            if avg_distance < min_avg_distance {
                min_avg_distance = avg_distance;
                best_representative = point.features.clone();
                best_diagnosis = point.diagnosis;
            }
        }

        // Add the best representative of the cluster to the result
        best_representatives.push((best_representative, best_diagnosis));
    }

    best_representatives
}


fn main() -> Result<(), Box<dyn Error>> {
    // Load and Prepare the dataset
    let file_path = "heart_disease.csv";
    let patients = load_and_prepare_data(file_path).expect("Error loading data.");

    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let headers = rdr.headers()?.clone();

    // Print column headings
    println!("Column Headings:");
    for heading in headers.iter() {
        print!("{}, ", heading);
    }
    println!(); // New line after printing headings

    // Primary Analysis
    let (diagnosed_with_disease, not_diagnosed_with_disease) = split_diagnosis(&patients);

    // Calculate Median for all symptoms in each group
    let median_diagnosed = calculate_median(&diagnosed_with_disease);
    let median_not_diagnosed = calculate_median(&not_diagnosed_with_disease);
    println!("Median Symptoms for Patients Diagnosed with Heart Disease: {:?}", median_diagnosed);
    println!("Median Symptoms for Patients Not Diagnosed with Heart Disease: {:?}", median_not_diagnosed);

    // Perform Clustering
    let k = 2;
    let max_iter = 100;
    let centroids = cluster::clustering(k, &patients, max_iter);

    // Print centroids
    for (i, centroid) in centroids.iter().enumerate() {
        println!("Centroid {}: {:?}", i + 1, centroid);
    }

    // Split patients into clusters based on the centroids
    let mut clusters = vec![Vec::new(); k];
    for patient in &patients {
        let mut min_distance = f64::INFINITY;
        let mut nearest_cluster_idx = 0;

        for (i, centroid) in centroids.iter().enumerate() {
            let distance = cluster::euclidean_distance(&patient.features, centroid);
            if distance < min_distance {
                min_distance = distance;
                nearest_cluster_idx = i;
            }
        }

        clusters[nearest_cluster_idx].push(patient.clone());
    }

    // Find the k best representatives
    let best_representatives = find_best_representatives(k, &centroids, &clusters);

    // Print the best representatives
    println!("Best Representatives:");
    for (i, representative) in best_representatives.iter().enumerate() {
        println!("Cluster {}: {:?}", i + 1, representative);
    }
    Ok(())
}