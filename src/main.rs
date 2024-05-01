extern crate csv;
extern crate rand;

use rand::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::Path;

// Define a struct to represent a patient
#[derive(Debug, Clone)]
struct Patient {
    features: Vec<f64>,
    diagnosis: u8,
}


// Open and Read Heart Disease Dataset
fn load_and_prepare_data(file_path: &str) -> Result<Vec<Patient>, csv::Error> {
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
fn split_diagnosis(patients: &[Patient]) -> (Vec<Patient>, Vec<Patient>) {
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
fn calculate_median(patients: &[Patient]) -> Vec<f64> {
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

// K-means clustering algorithm
fn kmeans(k: usize, patients: &[Patient], max_iter: usize) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let num_features = patients[0].features.len();

    // Initialize clusters with random centroids within the range of data values
    let mut centroids: Vec<Vec<f64>> = (0..k)
    .map(|_| {
        let mut centroid = vec![0.0; num_features];
        for j in 0..num_features {
            let min_val = patients.iter().map(|p| p.features[j]).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let max_val = patients.iter().map(|p| p.features[j]).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            centroid[j] = rng.gen_range(min_val..max_val);
        }
        centroid
    })
    .collect();

    // Iterate until convergence or maximum iterations reached
    for _ in 0..max_iter {
        let mut clusters: Vec<Vec<Patient>> = vec![Vec::new(); k];

        // Assign each patient to the nearest cluster
        for patient in patients {
            let mut min_distance = f64::INFINITY;
            let mut nearest_cluster_idx = 0;

            for (i, centroid) in centroids.iter().enumerate() {
                let distance = euclidean_distance(&patient.features, centroid);
                if distance < min_distance {
                    min_distance = distance;
                    nearest_cluster_idx = i;
                }
            }

            clusters[nearest_cluster_idx].push(patient.clone());
        }

        // Update cluster centroids
        let mut converged = true;
        for (i, cluster) in clusters.iter().enumerate() {
            let num_members = cluster.len() as f64;
            if num_members > 0.0 {
                let mut new_centroid = vec![0.0; num_features];
                for member in cluster {
                    for (j, feature) in member.features.iter().enumerate() {
                        new_centroid[j] += feature / num_members;
                    }
                }

                if new_centroid != centroids[i] {
                    converged = false;
                    centroids[i] = new_centroid;
                }
            }
        }

        if converged {
            break;
        }
    }

    centroids
}

// Calculate Euclidean distance between two vectors
fn euclidean_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(&x, &y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

fn main() {
    // Load and Prepare the dataset
    let file_path = "heart_disease.csv";
    let patients = load_and_prepare_data(file_path).expect("Error loading data.");

    // Primary Analysis
    let (diagnosed_with_disease, not_diagnosed_with_disease) = split_diagnosis(&patients);
    println!("Patients Diagnosed with Heart Disease:");
//    println!("{:?}", diagnosed_with_disease);
    println!("Patients Not Diagnosed with Heart Disease:");
//    println!("{:?}", not_diagnosed_with_disease);

    // Calculate Median for all symptoms in each group
    let median_diagnosed = calculate_median(&diagnosed_with_disease);
    let median_not_diagnosed = calculate_median(&not_diagnosed_with_disease);
    println!("Median Symptoms for Patients Diagnosed with Heart Disease: {:?}", median_diagnosed);
    println!("Median Symptoms for Patients Not Diagnosed with Heart Disease: {:?}", median_not_diagnosed);

    // Perform K-means clustering
    let k = 4;
    let max_iter = 100;
    let centroids = kmeans(k, &patients, max_iter);

    // Print centroids
    for (i, centroid) in centroids.iter().enumerate() {
        println!("Centroid {}: {:?}", i + 1, centroid);
    }
}

