use serde::{Serialize, Deserialize};
use entropy_forge::quality::{QualityMetrics, NistTests};
use entropy_forge::entropy::{SystemEntropy, EntropySource};

#[derive(Serialize, Deserialize)]
pub struct NistResult {
    pub name: String,
    pub p_value: f64,
    pub passed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct QualityResponse {
    pub shannon_entropy: f64,
    pub min_entropy: f64,
    pub chi_square: f64,
    pub mean: f64,
    pub longest_run: usize,
    pub overall_score: f64,
    pub total_bytes: usize,
    pub nist_results: Vec<NistResult>,
}

#[tauri::command]
pub fn run_quality_tests(sample_size: usize) -> QualityResponse {
    let mut entropy = SystemEntropy::new();

    // Run quality metrics
    let metrics = QualityMetrics::analyze(&mut entropy, sample_size);

    // Run NIST tests
    let mut data = vec![0u8; sample_size];
    entropy.fill_bytes(&mut data);

    let nist = NistTests::run_all_tests(&data);
    let nist_results = nist.into_iter().map(|(name, p_value)| {
        NistResult {
            name: name.to_string(),
            p_value,
            passed: p_value >= 0.01
        }
    }).collect();

    QualityResponse {
        shannon_entropy: metrics.shannon_entropy,
        min_entropy: metrics.min_entropy,
        chi_square: metrics.chi_square,
        mean: metrics.mean,
        longest_run: metrics.longest_run,
        overall_score: metrics.overall_score(),
        total_bytes: sample_size,
        nist_results,
    }
}
