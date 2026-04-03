import { invoke } from "@tauri-apps/api/core";
import { Controls } from "../components/controls";

interface NistStep {
  step_type: string;
  bits: number[];
  ones_count: number;
  zeros_count: number;
  sum: number;
  s_obs: number;
  p_value: number;
  passed: boolean;
}

interface NistStepsResponse {
  steps: NistStep[];
  total: number;
  input_text: string;
}

export function initNistVisualizer() {
  const inputEl = document.getElementById('nist-input') as HTMLTextAreaElement;
  const analyzeBtn = document.getElementById('nist-analyze-btn') as HTMLButtonElement;
  const randomBtn = document.getElementById('nist-random-btn') as HTMLButtonElement;
  const vizSection = document.getElementById('nist-viz-section') as HTMLElement;
  const stepHeader = document.getElementById('nist-step-header') as HTMLElement;
  const contentArea = document.getElementById('nist-content-area') as HTMLElement;

  if (!analyzeBtn) return;

  let steps: NistStep[] = [];
  let currentIndex = 0;
  let timerId: number | null = null;
  let controls: Controls;

  const renderStep = () => {
    if (steps.length === 0) return;
    const step = steps[currentIndex];

    const titles: Record<string, string> = {
      "ConvertToBits": "Convert to Bits",
      "CountOnesZeros": "Count Ones and Zeros",
      "CalculateStatistic": "Calculate Statistic",
      "CalculatePValue": "Calculate P-Value",
      "Interpret": "Interpretation"
    };

    stepHeader.textContent = `Step ${currentIndex + 1} of ${steps.length}: ${titles[step.step_type] || step.step_type}`;

    let html = '';

    // Bits display
    html += `<div class="nist-bits font-mono">`;
    for (let i = 0; i < step.bits.length; i++) {
      if (i > 0 && i % 8 === 0) html += ' ';
      html += `<span class="${step.bits[i] === 1 ? 'text-green' : 'text-muted'}">${step.bits[i]}</span>`;
    }
    html += `</div>`;

    if (step.step_type !== 'ConvertToBits') {
      const total = step.bits.length;
      const onesPct = total > 0 ? (step.ones_count / total) * 100 : 0;

      html += `
        <div class="nist-ratio-container">
          <div class="flex-row">
            <div>Ones: <strong class="text-green">${step.ones_count}</strong></div>
            <div class="spacer"></div>
            <div>Zeros: <strong class="text-muted">${step.zeros_count}</strong></div>
          </div>
          <div class="nist-ratio-bar">
            <div class="nist-ratio-ones" style="width: ${onesPct}%"></div>
            <div class="nist-ratio-center"></div>
          </div>
          <div class="text-center text-sm text-muted">Ideal: 50%</div>
        </div>
      `;
    }

    if (["CalculateStatistic", "CalculatePValue", "Interpret"].includes(step.step_type)) {
      html += `
        <div class="nist-stats">
          <div>Sum (+1 for 1, -1 for 0): <strong>${step.sum}</strong></div>
          <div>S_obs (|Sum| / √n): <strong>${step.s_obs.toFixed(4)}</strong></div>
      `;

      if (step.step_type === "CalculatePValue" || step.step_type === "Interpret") {
        html += `<div>P-Value (erfc(S_obs/√2)): <strong>${step.p_value.toFixed(4)}</strong></div>`;
      }

      if (step.step_type === "Interpret") {
        if (step.passed) {
          html += `<div class="text-green" style="margin-top: 10px;">✅ PASS: The sequence looks random.</div>`;
        } else {
          html += `<div class="text-red" style="margin-top: 10px;">❌ FAIL: The sequence has too many 1s or 0s.</div>`;
        }
        html += `<div class="text-muted text-sm">(Threshold: P-value ≥ 0.01)</div>`;
      }

      html += `</div>`;
    }

    contentArea.innerHTML = html;
    controls.updateState(currentIndex, steps.length, timerId !== null);
  };

  const playNext = () => {
    if (currentIndex < steps.length - 1) {
      currentIndex++;
      renderStep();
    } else {
      stopTimer();
      controls.updateState(currentIndex, steps.length, false);
    }
  };

  const stopTimer = () => {
    if (timerId !== null) {
      clearInterval(timerId);
      timerId = null;
    }
  };

  const startTimer = () => {
    stopTimer();
    if (currentIndex >= steps.length - 1) currentIndex = 0;
    timerId = window.setInterval(playNext, 1000 / controls.speed);
  };

  controls = new Controls('nist-controls',
    () => { currentIndex--; renderStep(); },
    () => { currentIndex++; renderStep(); },
    () => {
      if (timerId !== null) stopTimer();
      else startTimer();
    },
    (speed) => {
      if (timerId !== null) startTimer();
    }
  );

  const handleResponse = (response: NistStepsResponse) => {
    steps = response.steps;
    if (response.input_text) inputEl.value = response.input_text;
    currentIndex = 0;
    stopTimer();
    vizSection.classList.remove('hidden');
    renderStep();
  };

  analyzeBtn.addEventListener('click', async () => {
    const text = inputEl.value;
    if (!text) return;
    analyzeBtn.disabled = true;
    try {
      const response = await invoke<NistStepsResponse>('get_nist_steps', { text });
      handleResponse(response);
    } catch (e) { alert(e); } finally { analyzeBtn.disabled = false; }
  });

  randomBtn.addEventListener('click', async () => {
    randomBtn.disabled = true;
    try {
      const response = await invoke<NistStepsResponse>('get_nist_steps_random', { count: 16 });
      handleResponse(response);
    } catch (e) { alert(e); } finally { randomBtn.disabled = false; }
  });
}
