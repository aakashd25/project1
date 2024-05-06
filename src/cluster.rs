

use crate::Patient;
use rand::prelude::*;


    // Perform Clustering
    pub fn clustering(k: usize, patients: &[Patient], max_iter: usize) -> Vec<Vec<f64>> {
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
                } else {
                    // Find the cluster with the maximum number of members and initialize the empty cluster centroid with its centroid
                    let max_cluster_idx = clusters.iter().enumerate().max_by_key(|&(_, c)| c.len()).map(|(idx, _)| idx).unwrap();
                    centroids[i] = centroids[max_cluster_idx].clone();
                }
            }

            if converged {
                break;
            }
        }

        centroids
    }

    // Calculate Euclidean distance between two vectors
    pub fn euclidean_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
        vec1.iter()
            .zip(vec2.iter())
            .map(|(&x, &y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }



