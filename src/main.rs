// DS 210 Final Project - Rust
// Packages
extern crate csv;
extern crate rand;

use rand::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::path::Path;

// Define a struct to represent a patient
#[derive(Debug, Clone)]
struct Patient {
    features: Vec<f64>,
    diagnosis: u8,
}

// Define a struct to represent patient features
struct PatientFeatures {
    rest_bp: f64,
    chest_pain: f64,
    thalassemia: f64,
    age: f64,
    fasting_bs: f64,
    max_hr: f64,
    exercise_angina: f64,
    gender: f64,
    st_slope: f64,
    cholesterol: f64,
    st_depression: f64,
    rest_ecg: f64,
    num_vessels: f64,
}

// Define a struct to represent a patient graph
struct PatientGraph {
    // Define fields to represent the graph (e.g., adjacency list or matrix)
    adjacency_list: Vec<HashSet<usize>>,
}

impl PatientGraph {

    // Constructor
    fn new() -> Self {
        Self {
            adjacency_list: Vec::new(),
        }
    }

    // Method to add an edge between two patients
    fn add_edge(&mut self, u: usize, v: usize) {
        // Assuming an undirected graph
        self.adjacency_list[u].insert(v);
        self.adjacency_list[v].insert(u);
    }

    // K-core decomposition algorithm
    fn k_core_decomposition(&self, k: usize) -> Vec<HashSet<usize>> {
        let mut graph = self.adjacency_list.clone();
        let mut cores = Vec::new();

        while !graph.is_empty() {
            let mut current_core = HashSet::new();
            let mut updated = true;

            // Iteratively remove nodes with degree less than k
            while updated {
                updated = false;
                let mut to_remove = Vec::new();
                for (node, neighbors) in graph.iter().enumerate() {
                    if neighbors.len() < k {
                        to_remove.push(node);
                        updated = true;
                    }
                }
                for node in &to_remove {
                    graph[*node].clear();
                    for neighbors in &mut graph {
                        neighbors.remove(node);
                    }
                }
            }

            // Extract current core nodes
            for node in 0..graph.len() {
                if !graph[node].is_empty() {
                    current_core.insert(node);
                }
            }
            cores.push(current_core.clone());

            // Remove current core from the graph
            for node in &current_core {
                graph[*node].clear();
            }
            for neighbors in &mut graph {
                for node in &current_core {
                    neighbors.remove(node);
                }
            }
        }

        cores
    }
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

// Constructing Patient Graph
fn construct_patient_graph(patients: &[Patient]) -> PatientGraph {
    let num_patients = patients.len();
    let mut adjacency_list = vec![HashSet::new(); num_patients];

    for i in 0..num_patients {
        for j in i + 1..num_patients {
            let similarity = compute_similarity(&patients[i], &patients[j]);

            // Add an edge if similarity exceeds a threshold
            if similarity >= 0.5 {
                adjacency_list[i].insert(j);
                adjacency_list[j].insert(i);
            }
        }
    }

    PatientGraph { adjacency_list }
}

// Computing Similarity of Patients
fn compute_similarity(patient1: &Patient, patient2: &Patient) -> f64 {
    let mut similarity = 0.0;
    for (feature1, feature2) in patient1.features.iter().zip(patient2.features.iter()) {
        if (feature1 - feature2).abs() < 0.001 { // Adjust epsilon according to your data
            similarity += 1.0;
        }
    }
    similarity / patient1.features.len() as f64 // Normalize by the number of features
}

fn main() {

    // Load and Prepare the dataset
    let file_path = "heart_disease.csv";
    let patients = load_and_prepare_data(file_path).expect("Error loading data.");

    // Primary Analysis
    let (diagnosed_with_disease, not_diagnosed_with_disease) = split_diagnosis(&patients);
    println!("Patients Diagnosed with Heart Disease:");
    println!("{:?}", diagnosed_with_disease);
    println!("Patients Not Diagnosed with Heart Disease:");
    println!("{:?}", not_diagnosed_with_disease);

    // Calculate Median for all symptoms in each group
    println!("Data Values: rest_bp,chest_pain,thalassemia,age,fasting_bs,max_hr,exercise_angina,gender,st_slope,cholesterol,st_depression,rest_ecg,num_vessels,diagnosis");

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

    // Construct a graph based on patient features
    let graph = construct_patient_graph(&patients);

    // Perform k-core decomposition
    let k_core_value = 2; // Set the value of k for k-core decomposition
    let cores = graph.k_core_decomposition(k_core_value);

    // Output the k-core subgraphs
    for (i, core) in cores.iter().enumerate() {
        println!("K-Core {}: {:?}", i + 1, core);
    }

}
