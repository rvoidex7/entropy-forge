use serde::{Serialize, Deserialize};
use entropy_forge::bench::PerformanceBench;
use entropy_forge::entropy::SystemEntropy;

#[derive(Serialize, Deserialize)]
pub struct BenchResponse {
    pub throughput_mbps: f64,
    pub latency_us: f64,
    pub bytes_generated: usize,
    pub duration_secs: f64,
}

#[tauri::command]
pub fn run_benchmark(bytes: usize) -> BenchResponse {
    let mut entropy = SystemEntropy::new();
    let result = PerformanceBench::benchmark(&mut entropy, bytes);

    BenchResponse {
        throughput_mbps: result.throughput_mbps,
        latency_us: result.latency_us,
        bytes_generated: result.bytes_generated,
        duration_secs: result.duration.as_secs_f64(),
    }
}
