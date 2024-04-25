// Packages
extern crate csv;

use csv::ReaderBuilder;

// Define a struct to represent a patient
#[derive(Debug, Clone)]
struct Patient {
    features: Vec<f64>,
    diagnosis: u8,
}

// Open and Read Heart Disease Dataset
fn load_and_prepare_data(file_path: &str) -> Result<Vec<Patient>, csv::Error> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let mut patients = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let features: Vec<f64> = record.iter().take(record.len() - 1).map(|v| v.parse().unwrap()).collect();
        let diagnosis: u8 = record[record.len() - 1].parse().unwrap();
        let patient = Patient { features, diagnosis };
        patients.push(patient);
    }

    Ok(patients)
}


// Primary Analysis: Split the data into patients diagnosed with heart disease (1) and those who are not (0)
fn split_diagnosis(patients: &[Patient]) -> (Vec<Patient>, Vec<Patient>) {
    let mut diagnosed_with_disease: Vec<Patient> = Vec::new();
    let mut not_diagnosed_with_disease: Vec<Patient> = Vec::new();

    for patient in patients {
        // Clone the entire Patient object
        let cloned_patient = patient.clone();

        if cloned_patient.diagnosis == 1 {
            diagnosed_with_disease.push(cloned_patient);
        } else {
            not_diagnosed_with_disease.push(cloned_patient);
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
    let median_diagnosed = calculate_median(&diagnosed_with_disease);
    let median_not_diagnosed = calculate_median(&not_diagnosed_with_disease);
    println!("Median Symptoms for Patients Diagnosed with Heart Disease: {:?}", median_diagnosed);
    println!("Median Symptoms for Patients Not Diagnosed with Heart Disease: {:?}", median_not_diagnosed);

    // Rank the representative values from each cluster from highest to lowest of the value of heart disease
    // Placeholder for ranking representatives
}

// Rank the representitive values from each cluster from highest to lowest of the value of heart disease