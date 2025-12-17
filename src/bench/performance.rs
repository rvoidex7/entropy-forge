//! Performance benchmarking for entropy sources

use crate::entropy::EntropySource;
use std::time::{Duration, Instant};

/// Performance benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Throughput in megabytes per second
    pub throughput_mbps: f64,
    
    /// Average latency per byte in microseconds
    pub latency_us: f64,
    
    /// Total bytes generated
    pub bytes_generated: usize,
    
    /// Total time taken
    pub duration: Duration,
}

/// Performance benchmarking utility
pub struct PerformanceBench;

impl PerformanceBench {
    /// Benchmark an entropy source
    ///
    /// Generates `total_bytes` and measures throughput and latency.
    ///
    /// # Arguments
    ///
    /// * `source` - Entropy source to benchmark
    /// * `total_bytes` - Number of bytes to generate
    ///
    /// # Examples
    ///
    /// ```
    /// use entropy_weaver::entropy::SystemEntropy;
    /// use entropy_weaver::bench::PerformanceBench;
    ///
    /// let mut source = SystemEntropy::new();
    /// let result = PerformanceBench::benchmark(&mut source, 1_000_000);
    ///
    /// println!("Throughput: {:.2} MB/s", result.throughput_mbps);
    /// println!("Latency: {:.2} µs/byte", result.latency_us);
    /// ```
    pub fn benchmark<E: EntropySource>(source: &mut E, total_bytes: usize) -> BenchmarkResult {
        let mut buffer = vec![0u8; total_bytes];
        
        let start = Instant::now();
        source.fill_bytes(&mut buffer);
        let duration = start.elapsed();
        
        let duration_secs = duration.as_secs_f64();
        let throughput_mbps = (total_bytes as f64 / duration_secs) / 1_000_000.0;
        let latency_us = (duration_secs * 1_000_000.0) / total_bytes as f64;
        
        BenchmarkResult {
            throughput_mbps,
            latency_us,
            bytes_generated: total_bytes,
            duration,
        }
    }
    
    /// Run multiple iterations and return average
    pub fn benchmark_avg<E: EntropySource>(
        source: &mut E,
        bytes_per_iteration: usize,
        iterations: usize,
    ) -> BenchmarkResult {
        let mut total_throughput = 0.0;
        let mut total_latency = 0.0;
        let mut total_duration = Duration::ZERO;
        let total_bytes = bytes_per_iteration * iterations;
        
        for _ in 0..iterations {
            let result = Self::benchmark(source, bytes_per_iteration);
            total_throughput += result.throughput_mbps;
            total_latency += result.latency_us;
            total_duration += result.duration;
        }
        
        BenchmarkResult {
            throughput_mbps: total_throughput / iterations as f64,
            latency_us: total_latency / iterations as f64,
            bytes_generated: total_bytes,
            duration: total_duration,
        }
    }
}

impl BenchmarkResult {
    /// Format result as human-readable string
    pub fn format(&self) -> String {
        format!(
            "Throughput: {:.2} MB/s\n\
             Latency: {:.2} µs/byte\n\
             Generated: {} bytes in {:.2}s",
            self.throughput_mbps,
            self.latency_us,
            self.bytes_generated,
            self.duration.as_secs_f64()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy::SystemEntropy;
    
    #[test]
    fn test_benchmark() {
        let mut source = SystemEntropy::new();
        let result = PerformanceBench::benchmark(&mut source, 10_000);
        
        assert!(result.throughput_mbps > 0.0);
        assert!(result.latency_us > 0.0);
        assert_eq!(result.bytes_generated, 10_000);
    }
    
    #[test]
    fn test_benchmark_avg() {
        let mut source = SystemEntropy::new();
        let result = PerformanceBench::benchmark_avg(&mut source, 1_000, 5);
        
        assert!(result.throughput_mbps > 0.0);
        assert_eq!(result.bytes_generated, 5_000);
    }
}
