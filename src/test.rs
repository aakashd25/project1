// Test Code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_and_prepare_data() {
        // Assuming you have a test CSV file for testing purposes
        let file_path = "test_data.csv";
        let result = load_and_prepare_data(file_path);
        assert!(result.is_ok()); // Check if loading and preparation was successful
    }

    #[test]
    fn test_split_diagnosis() {
        // Create some sample patients
        let patients = vec![
            Patient {
                features: vec![1.0, 2.0, 3.0],
                diagnosis: 1,
            },
            Patient {
                features: vec![4.0, 5.0, 6.0],
                diagnosis: 0,
            },
        ];

        let (diagnosed, not_diagnosed) = split_diagnosis(&patients);

        // Assert that patients are correctly split based on diagnosis
        assert_eq!(diagnosed.len(), 1);
        assert_eq!(not_diagnosed.len(), 1);
    }

    #[test]
    fn test_calculate_median() {
        // Create some sample patients with features
        let patients = vec![
            Patient {
                features: vec![1.0, 2.0, 3.0],
                diagnosis: 1,
            },
            Patient {
                features: vec![4.0, 5.0, 6.0],
                diagnosis: 1,
            },
        ];

        let median = calculate_median(&patients);

        // Assert that the median is calculated correctly for each feature
        assert_eq!(median.len(), patients[0].features.len());
    }

    #[test]
    fn test_kmeans() {
        // Create some sample patients with features
        let patients = vec![
            Patient {
                features: vec![1.0, 2.0, 3.0],
                diagnosis: 1,
            },
            Patient {
                features: vec![4.0, 5.0, 6.0],
                diagnosis: 0,
            },
        ];

        let k = 2;
        let max_iter = 100;
        let centroids = kmeans(k, &patients, max_iter);

        assert_eq!(centroids.len(), k);

    let k = 3; // Number of representatives to select from each cluster
    let centroids: Vec<Vec<f64>> = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]; // Example centroids
    let clusters: Vec<Vec<Patient>> = vec![
        vec![
            Patient { features: vec![1.0, 2.0, 3.0], diagnosis: 1 },
            Patient { features: vec![2.0, 3.0, 4.0], diagnosis: 1 },
            // Add more patients for the first cluster
        ],
        vec![
            Patient { features: vec![4.0, 5.0, 6.0], diagnosis: 0 },
            Patient { features: vec![5.0, 6.0, 7.0], diagnosis: 0 },
            // Add more patients for the second cluster
        ],
    ]; // Example clusters

    // Find the k best representatives
    let best_representatives = find_best_representatives(k, &centroids, &clusters);

    // Print the best representatives
    for (i, rep) in best_representatives.iter().enumerate() {
        println!("Representative {}: {:?}", i + 1, rep);
    }

    }
}

