extern crate csv;
extern crate rand;

use csv::ReaderBuilder;
use rand::seq::SliceRandom;
use std::error::Error;
use std::fs::File;

// Define a struct to represent a patient
#[derive(Debug)]
struct Patient {
    features: Vec<f64>,
    diagnosis: u8,
}

// Function to load and prepare the dataset
fn load_and_prepare_data(file_path: &str) -> Result<Vec<Patient>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let mut patients = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let features: Vec<f64> = record.iter().take(record.len() - 1).map(|v| v.parse::<f64>().unwrap()).collect();
        let diagnosis: u8 = record[record.len() - 1].parse::<u8>().unwrap();
        let patient = Patient { features, diagnosis };
        patients.push(patient);
    }

    Ok(patients)
}

// Function to perform k-means clustering
fn k_means_clustering(patients: &[Patient], k: usize) -> Result<Vec<usize>, Box<dyn Error>> {
    let features = patients[0].features.len();
    let mut rng = rand::thread_rng();

    // Randomly shuffle indices and take the first k as initial centroids
    let mut indices: Vec<usize> = (0..patients.len()).collect();
    indices.shuffle(&mut rng);
    let mut centroids: Vec<Vec<f64>> = indices.iter()
        .take(k)
        .map(|&i| patients[i].features.clone())
        .collect();

    // Perform k-means clustering
    let mut clusters = vec![0; patients.len()];
    for _ in 0..10 { // Perform 10 iterations
        // Assign each patient to the nearest centroid
        for (i, patient) in patients.iter().enumerate() {
            let mut min_distance = f64::INFINITY;
            let mut cluster_id = 0;
            for (j, centroid) in centroids.iter().enumerate() {
                let distance = euclidean_distance(&patient.features, centroid);
                if distance < min_distance {
                    min_distance = distance;
                    cluster_id = j;
                }
            }
            clusters[i] = cluster_id;
        }

        // Update centroids based on cluster assignments
        for cluster_id in 0..k {
            let mut cluster_patients = Vec::new();
            for (&c, p) in clusters.iter().zip(patients.iter()) {
                if c == cluster_id {
                    cluster_patients.push(p);
                }
            }
            if !cluster_patients.is_empty() {
                let mut new_centroid = vec![0.0; features];
                for patient in &cluster_patients {
                    for (i, &feature) in patient.features.iter().enumerate() {
                        new_centroid[i] += feature;
                    }
                }
                centroids[cluster_id] = new_centroid.iter().map(|&x| x / cluster_patients.len() as f64).collect();
            }
        }
    }

    Ok(clusters)
}

// Function to calculate Euclidean distance between two vectors
fn euclidean_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    let squared_distance: f64 = vec1.iter()
        .zip(vec2.iter())
        .map(|(&x, &y)| (x - y).powi(2))
        .sum();
    squared_distance.sqrt()
}

// Function to select representatives from each cluster
fn select_representatives(patients: &[Patient], clusters: &[usize], k: usize) -> Vec<Vec<f64>> {
    let mut representatives = vec![vec![0.0; patients[0].features.len()]; k];

    // Calculate mean values of features for each cluster
    let mut cluster_counts = vec![0; k];
    for (patient, &cluster_id) in patients.iter().zip(clusters) {
        cluster_counts[cluster_id] += 1;
        for (i, &feature) in patient.features.iter().enumerate() {
            representatives[cluster_id][i] += feature;
        }
    }

    for (i, count) in cluster_counts.iter().enumerate() {
        if *count > 0 {
            representatives[i] = representatives[i].iter().map(|&x| x / *count as f64).collect();
        }
    }

    representatives
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "heart_disease.csv"; // Change to your file path
    let patients = load_and_prepare_data(file_path)?;

    let k = 2; // Number of clusters

    // Perform k-means clustering
    let clusters = k_means_clustering(&patients, k)?;

    // Select representatives from each cluster
    let representatives = select_representatives(&patients, &clusters, k);

    // Print representatives
    println!("Representatives:");
    for (i, rep) in representatives.iter().enumerate() {
        println!("Cluster {}: {:?}", i, rep);
    }

    Ok(())
}