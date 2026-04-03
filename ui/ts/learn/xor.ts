import { invoke } from "@tauri-apps/api/core";
import { Controls } from "../components/controls";
import { events } from "../main";

interface BitOp {
  input_bit: boolean;
  key_bit: boolean;
  result_bit: boolean;
  position: number;
}

interface XorStep {
  character: string;
  input_byte: number;
  keystream_byte: number;
  result_byte: number;
  input_binary: string;
  keystream_binary: string;
  result_binary: string;
  bit_ops: BitOp[];
}

interface XorStepsResponse {
  steps: XorStep[];
  total: number;
}

export function initXorVisualizer() {
  const inputEl = document.getElementById('xor-input') as HTMLTextAreaElement;
  const startBtn = document.getElementById('xor-start-btn') as HTMLButtonElement;
  const vizSection = document.getElementById('xor-viz-section') as HTMLElement;
  const stepHeader = document.getElementById('xor-step-header') as HTMLElement;

  const inCharEl = document.getElementById('xor-in-char')!;
  const inDecEl = document.getElementById('xor-in-dec')!;
  const inBinEl = document.getElementById('xor-in-bin')!;

  const keyDecEl = document.getElementById('xor-key-dec')!;
  const keyBinEl = document.getElementById('xor-key-bin')!;

  const outDecEl = document.getElementById('xor-out-dec')!;
  const outBinEl = document.getElementById('xor-out-bin')!;

  const progressContainer = document.getElementById('xor-progress')!;

  if (!startBtn) return;

  let steps: XorStep[] = [];
  let currentIndex = 0;
  let timerId: number | null = null;
  let controls: Controls;

  const renderStep = () => {
    if (steps.length === 0) return;
    const step = steps[currentIndex];

    stepHeader.textContent = `Step ${currentIndex + 1} of ${steps.length}: Encrypting '${step.character}'`;

    inCharEl.textContent = `'${step.character}'`;
    inDecEl.textContent = `(${step.input_byte})`;
    inBinEl.textContent = step.input_binary;

    keyDecEl.textContent = `(${step.keystream_byte})`;
    keyBinEl.textContent = step.keystream_binary;

    outDecEl.textContent = `(${step.result_byte})`;
    outBinEl.textContent = step.result_binary;

    // Render progress badges
    progressContainer.innerHTML = '';
    steps.forEach((s, i) => {
      const badge = document.createElement('span');
      let hex = s.result_byte.toString(16).toUpperCase();
      if (hex.length < 2) hex = '0' + hex;
      badge.textContent = `[${hex}]`;
      badge.className = 'xor-badge';

      if (i < currentIndex) {
        badge.classList.add('past');
      } else if (i === currentIndex) {
        badge.classList.add('current');
      } else {
        badge.classList.add('future');
      }

      progressContainer.appendChild(badge);
    });

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
    if (currentIndex >= steps.length - 1) currentIndex = 0; // wrap around
    timerId = window.setInterval(playNext, 1000 / controls.speed);
  };

  controls = new Controls('xor-controls',
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
      const response = await invoke<XorStepsResponse>('get_xor_steps', { text });
      steps = response.steps;
      currentIndex = 0;
      stopTimer();
      vizSection.classList.remove('hidden');
      renderStep();
    } catch (e) {
      alert(`Error: ${e}`);
    } finally {
      startBtn.disabled = false;
      startBtn.textContent = "Start Visualization";
    }
  });

  events.on('navigate_learn_xor', (text: string) => {
    inputEl.value = text;
    // Auto-switch to XOR subtab
    document.querySelector('.sub-tab-btn[data-subtab="xor"]')?.dispatchEvent(new Event('click'));
    startBtn.click();
  });
}
