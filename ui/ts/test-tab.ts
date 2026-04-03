import { invoke } from "@tauri-apps/api/core";
import { ProgressBar } from "./components/progress-bar";

export interface NistResult {
  name: string;
  p_value: number;
  passed: boolean;
}

export interface QualityResponse {
  shannon_entropy: number;
  min_entropy: number;
  chi_square: number;
  mean: number;
  longest_run: number;
  overall_score: number;
  total_bytes: number;
  nist_results: NistResult[];
}

export function initTestTab() {
  const sizeSlider = document.getElementById('test-size-slider') as HTMLInputElement;
  const sizeLabel = document.getElementById('test-size-label') as HTMLElement;
  const runBtn = document.getElementById('test-run-btn') as HTMLButtonElement;
  const resultsSection = document.getElementById('test-results-section') as HTMLElement;
  const metricsContainer = document.getElementById('test-metrics-container') as HTMLElement;
  const nistTableBody = document.getElementById('test-nist-tbody') as HTMLElement;

  if (!sizeSlider || !runBtn || !resultsSection) return;

  // Setup slider
  const updateSliderLabel = () => {
    // Logarithmic slider mapping: 1,000 to 1,000,000
    // min = 3 (10^3), max = 6 (10^6)
    const val = parseFloat(sizeSlider.value);
    const bytes = Math.round(Math.pow(10, val));
    sizeLabel.textContent = bytes.toLocaleString() + " bytes";
    return bytes;
  };

  sizeSlider.addEventListener('input', updateSliderLabel);
  updateSliderLabel(); // initial

  // Create progress bars
  const shannonBar = new ProgressBar(
    document.getElementById('test-shannon-bar')!,
    "Shannon Entropy",
    "Measures the unpredictability of data.\n8.0 bits/byte is perfect randomness.\nLower values mean patterns exist.",
    { green: 0.9375, orange: 0.75 } // 7.5/8, 6.0/8
  );

  const minBar = new ProgressBar(
    document.getElementById('test-min-bar')!,
    "Min-Entropy",
    "The worst-case entropy.\nCrucial for security guarantees.\nIf this is low, an attacker might guess the key.",
    { green: 0.9375, orange: 0.75 }
  );

  const scoreBar = new ProgressBar(
    document.getElementById('test-score-bar')!,
    "Overall Score",
    "Weighted quality score: Shannon entropy (50%), Min-entropy (30%), Mean closeness to 127.5 (20%).",
    { green: 0.8, orange: 0.6 } // out of 100
  );

  let isLoading = false;

  const runTests = async () => {
    if (isLoading) return;
    const bytes = updateSliderLabel();

    isLoading = true;
    runBtn.disabled = true;
    runBtn.textContent = "Running Tests...";

    // Hide results temporarily
    resultsSection.classList.add('hidden');

    try {
      const response = await invoke<QualityResponse>('run_quality_tests', {
        sampleSize: bytes
      });

      // Update metrics
      shannonBar.update(response.shannon_entropy, 8.0, `${response.shannon_entropy.toFixed(4)} bits/byte`);
      minBar.update(response.min_entropy, 8.0, `${response.min_entropy.toFixed(4)} bits/byte`);
      scoreBar.update(response.overall_score, 100.0, `${response.overall_score.toFixed(1)} / 100`);

      document.getElementById('test-mean-val')!.textContent = `${response.mean.toFixed(2)} (ideal: 127.5)`;
      document.getElementById('test-chi-val')!.textContent = response.chi_square.toFixed(2);
      document.getElementById('test-run-val')!.textContent = `${response.longest_run} bits`;

      // Update NIST table
      nistTableBody.innerHTML = '';
      for (const res of response.nist_results) {
        const tr = document.createElement('tr');

        const tdName = document.createElement('td');
        tdName.textContent = res.name;

        const tdPval = document.createElement('td');
        tdPval.textContent = res.p_value.toFixed(4);

        const tdResult = document.createElement('td');
        if (res.passed) {
          tdResult.innerHTML = '<span class="text-green">✓ Pass</span>';
        } else {
          tdResult.innerHTML = '<span class="text-red">✗ Fail</span>';
        }

        tr.appendChild(tdName);
        tr.appendChild(tdPval);
        tr.appendChild(tdResult);
        nistTableBody.appendChild(tr);
      }

      resultsSection.classList.remove('hidden');

    } catch (e) {
      console.error(e);
      alert(`Error running tests: ${e}`);
    } finally {
      isLoading = false;
      runBtn.disabled = false;
      runBtn.textContent = "🔬 Run All Tests";
    }
  };

  runBtn.addEventListener('click', runTests);
}
