import { invoke } from "@tauri-apps/api/core";

export interface BenchResponse {
  throughput_mbps: number;
  latency_us: number;
  bytes_generated: number;
  duration_secs: number;
}

export function initBenchTab() {
  const sizeSlider = document.getElementById('bench-size-slider') as HTMLInputElement;
  const sizeLabel = document.getElementById('bench-size-label') as HTMLElement;
  const runBtn = document.getElementById('bench-run-btn') as HTMLButtonElement;
  const resultsSection = document.getElementById('bench-results-section') as HTMLElement;

  if (!sizeSlider || !runBtn || !resultsSection) return;

  const updateSliderLabel = () => {
    // Logarithmic: 10,000 to 10,000,000 => 10^4 to 10^7
    const val = parseFloat(sizeSlider.value);
    const bytes = Math.round(Math.pow(10, val));
    sizeLabel.textContent = bytes.toLocaleString() + " bytes";
    return bytes;
  };

  sizeSlider.addEventListener('input', updateSliderLabel);
  updateSliderLabel(); // initial

  let isLoading = false;

  const runBenchmark = async () => {
    if (isLoading) return;
    const bytes = updateSliderLabel();

    isLoading = true;
    runBtn.disabled = true;
    runBtn.textContent = "Benchmarking...";
    resultsSection.classList.add('hidden');

    try {
      const response = await invoke<BenchResponse>('run_benchmark', { bytes });

      document.getElementById('bench-throughput-val')!.textContent = `${response.throughput_mbps.toFixed(1)} MB/s`;
      document.getElementById('bench-latency-val')!.textContent = `${response.latency_us.toFixed(2)} µs/byte`;
      document.getElementById('bench-bytes-val')!.textContent = response.bytes_generated.toLocaleString();
      document.getElementById('bench-duration-val')!.textContent = `${response.duration_secs.toFixed(2)} s`;

      resultsSection.classList.remove('hidden');

    } catch (e) {
      console.error(e);
      alert(`Error running benchmark: ${e}`);
    } finally {
      isLoading = false;
      runBtn.disabled = false;
      runBtn.textContent = "⚡ Run Benchmark";
    }
  };

  runBtn.addEventListener('click', runBenchmark);
}
