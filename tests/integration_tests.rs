// Test Code
#[cfg(test)]
mod tests {
    use super::*;
    pub extern crate rand;
    pub use rand::prelude::*;

    // Define a struct to represent a patient
    #[derive(Debug, Clone)]
    #[derive(PartialEq)]
    pub struct Patient {
        pub features: Vec<f64>,
        pub diagnosis: u8,
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

    // Perform Clustering
    fn clustering(k: usize, patients: &[Patient], max_iter: usize) -> Vec<Vec<f64>> {
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
    fn euclidean_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
        vec1.iter()
            .zip(vec2.iter())
            .map(|(&x, &y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    // Function to find the k best representatives from each cluster
    fn find_best_representatives(k: usize, centroids: &[Vec<f64>], clusters: &[Vec<Patient>]) -> Vec<(Vec<f64>, u8)> {
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
    #[test]
    fn test_split_diagnosis() {
        let patients = vec![
            Patient { features: vec![1.0, 2.0], diagnosis: 0 },
            Patient { features: vec![3.0, 4.0], diagnosis: 1 },
            // Add more sample patients here
        ];
        let (diagnosed, not_diagnosed) = split_diagnosis(&patients);
        assert_eq!(diagnosed.len(), 1); // Check if the diagnosed patients are separated correctly
        assert_eq!(not_diagnosed.len(), 1); // Check if the not diagnosed patients are separated correctly
    }

    #[test]
    fn test_clustering() {
        // Create sample patients for testing
        let patients = vec![
            Patient { features: vec![1.0, 2.0], diagnosis: 0 },
            Patient { features: vec![2.0, 3.0], diagnosis: 1 },
            Patient { features: vec![3.0, 4.0], diagnosis: 0 },
            Patient { features: vec![4.0, 5.0], diagnosis: 1 },
        ];

        // Perform clustering
        let k = 2;
        let max_iter = 100;
        let centroids = clustering(k, &patients, max_iter);

        // Check if the number of centroids is correct
        assert_eq!(centroids.len(), k);
    }

    #[test]
    fn test_find_best_representatives() {
        let k = 2;
        let centroids = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
        ];
        let clusters = vec![
            vec![
                Patient { features: vec![1.1, 2.2], diagnosis: 0 },
                Patient { features: vec![1.5, 2.6], diagnosis: 1 },
            ],
            vec![
                Patient { features: vec![3.1, 4.2], diagnosis: 1 },
                Patient { features: vec![3.5, 4.6], diagnosis: 0 },
            ],
        ];
        let best_reps = find_best_representatives(k, &centroids, &clusters);
        assert_eq!(best_reps.len(), k); // Check if the correct number of best representatives is found
    }
}