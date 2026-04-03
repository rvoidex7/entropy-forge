import { invoke } from "@tauri-apps/api/core";
import { Controls } from "../components/controls";

interface EntropyStep {
  step_type: string;
  byte_counts: Record<string, number>;
  probabilities: Record<string, number>;
  entropy_contributions: Record<string, number>;
  current_entropy_sum: number;
  total_entropy: number;
  max_entropy: number;
}

interface EntropyStepsResponse {
  steps: EntropyStep[];
  total: number;
}

export function initEntropyVisualizer() {
  const inputEl = document.getElementById('entropy-input') as HTMLTextAreaElement;
  const startBtn = document.getElementById('entropy-calc-btn') as HTMLButtonElement;
  const vizSection = document.getElementById('entropy-viz-section') as HTMLElement;
  const stepHeader = document.getElementById('entropy-step-header') as HTMLElement;
  const contentArea = document.getElementById('entropy-content-area') as HTMLElement;

  if (!startBtn) return;

  let steps: EntropyStep[] = [];
  let currentIndex = 0;
  let timerId: number | null = null;
  let controls: Controls;

  const renderStep = () => {
    if (steps.length === 0) return;
    const step = steps[currentIndex];

    const titles: Record<string, string> = {
      "CountBytes": "Count Byte Frequencies",
      "CalculateProbabilities": "Calculate Probabilities",
      "CalculateContributions": "Calculate Contributions",
      "SumEntropy": "Sum for Total Entropy",
      "Interpret": "Interpretation"
    };

    stepHeader.textContent = `Step ${currentIndex + 1} of ${steps.length}: ${titles[step.step_type] || step.step_type}`;

    let html = '';

    if (["CountBytes", "CalculateProbabilities", "CalculateContributions"].includes(step.step_type)) {
      html += `
        <table class="entropy-table">
          <thead>
            <tr>
              <th>Byte</th>
              <th>Count</th>
              ${step.step_type !== 'CountBytes' ? '<th>Probability</th>' : ''}
              ${step.step_type === 'CalculateContributions' ? '<th>Contribution</th>' : ''}
              <th>Visual</th>
            </tr>
          </thead>
          <tbody>
      `;

      const keys = Object.keys(step.byte_counts).sort();
      const maxCount = Math.max(...Object.values(step.byte_counts));

      for (const k of keys) {
        const count = step.byte_counts[k];
        const p = step.probabilities[k] || 0;
        const c = step.entropy_contributions[k] || 0;
        const width = (count / maxCount) * 100;

        let charRepr = parseInt(k);
        let display = `'${String.fromCharCode(charRepr)}' (${charRepr})`;
        if (charRepr < 32 || charRepr > 126) display = `0x${charRepr.toString(16)}`;

        html += `<tr>
          <td>${display}</td>
          <td>${count}</td>
          ${step.step_type !== 'CountBytes' ? `<td>${p.toFixed(4)}</td>` : ''}
          ${step.step_type === 'CalculateContributions' ? `<td>${c.toFixed(4)} bits</td>` : ''}
          <td><div class="entropy-bar" style="width: ${width}%"></div></td>
        </tr>`;
      }
      html += `</tbody></table>`;
    } else {
      const eff = step.max_entropy > 0 ? (step.current_entropy_sum / step.max_entropy) * 100 : 0;
      html += `
        <div class="entropy-summary">
          <div class="entropy-val-large">Total Entropy: ${step.current_entropy_sum.toFixed(4)} bits/byte</div>
          <div class="text-muted">Maximum Possible: ${step.max_entropy.toFixed(4)} bits/byte</div>
          <div class="text-muted">Efficiency: ${eff.toFixed(1)}%</div>
        </div>
      `;

      if (step.step_type === "Interpret") {
        html += `<div class="entropy-interpret">`;
        if (step.current_entropy_sum < 2.0) {
          html += `<span class="text-red">Low entropy. The input is very predictable.</span>`;
        } else if (eff > 80.0) {
          html += `<span class="text-green">High entropy! The input looks quite random.</span>`;
        } else {
          html += `<span class="text-orange">Moderate entropy.</span>`;
        }
        html += `</div>`;
      }
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

  controls = new Controls('entropy-controls',
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

  startBtn.addEventListener('click', async () => {
    const text = inputEl.value;
    if (!text) return;

    startBtn.disabled = true;
    startBtn.textContent = "Loading...";

    try {
      const response = await invoke<EntropyStepsResponse>('get_entropy_steps', { text });
      steps = response.steps;
      currentIndex = 0;
      stopTimer();
      vizSection.classList.remove('hidden');
      renderStep();
    } catch (e) {
      alert(`Error: ${e}`);
    } finally {
      startBtn.disabled = false;
      startBtn.textContent = "Calculate";
    }
  });
}
