"use strict";
(() => {
  // node_modules/@tauri-apps/api/external/tslib/tslib.es6.js
  function __classPrivateFieldGet(receiver, state, kind, f) {
    if (kind === "a" && !f)
      throw new TypeError("Private accessor was defined without a getter");
    if (typeof state === "function" ? receiver !== state || !f : !state.has(receiver))
      throw new TypeError("Cannot read private member from an object whose class did not declare it");
    return kind === "m" ? f : kind === "a" ? f.call(receiver) : f ? f.value : state.get(receiver);
  }
  function __classPrivateFieldSet(receiver, state, value, kind, f) {
    if (kind === "m")
      throw new TypeError("Private method is not writable");
    if (kind === "a" && !f)
      throw new TypeError("Private accessor was defined without a setter");
    if (typeof state === "function" ? receiver !== state || !f : !state.has(receiver))
      throw new TypeError("Cannot write private member to an object whose class did not declare it");
    return kind === "a" ? f.call(receiver, value) : f ? f.value = value : state.set(receiver, value), value;
  }

  // node_modules/@tauri-apps/api/core.js
  var _Channel_onmessage;
  var _Channel_nextMessageIndex;
  var _Channel_pendingMessages;
  var _Channel_messageEndIndex;
  var _Resource_rid;
  var SERIALIZE_TO_IPC_FN = "__TAURI_TO_IPC_KEY__";
  function transformCallback(callback, once = false) {
    return window.__TAURI_INTERNALS__.transformCallback(callback, once);
  }
  var Channel = class {
    constructor(onmessage) {
      _Channel_onmessage.set(this, void 0);
      _Channel_nextMessageIndex.set(this, 0);
      _Channel_pendingMessages.set(this, []);
      _Channel_messageEndIndex.set(this, void 0);
      __classPrivateFieldSet(this, _Channel_onmessage, onmessage || (() => {
      }), "f");
      this.id = transformCallback((rawMessage) => {
        const index = rawMessage.index;
        if ("end" in rawMessage) {
          if (index == __classPrivateFieldGet(this, _Channel_nextMessageIndex, "f")) {
            this.cleanupCallback();
          } else {
            __classPrivateFieldSet(this, _Channel_messageEndIndex, index, "f");
          }
          return;
        }
        const message = rawMessage.message;
        if (index == __classPrivateFieldGet(this, _Channel_nextMessageIndex, "f")) {
          __classPrivateFieldGet(this, _Channel_onmessage, "f").call(this, message);
          __classPrivateFieldSet(this, _Channel_nextMessageIndex, __classPrivateFieldGet(this, _Channel_nextMessageIndex, "f") + 1, "f");
          while (__classPrivateFieldGet(this, _Channel_nextMessageIndex, "f") in __classPrivateFieldGet(this, _Channel_pendingMessages, "f")) {
            const message2 = __classPrivateFieldGet(this, _Channel_pendingMessages, "f")[__classPrivateFieldGet(this, _Channel_nextMessageIndex, "f")];
            __classPrivateFieldGet(this, _Channel_onmessage, "f").call(this, message2);
            delete __classPrivateFieldGet(this, _Channel_pendingMessages, "f")[__classPrivateFieldGet(this, _Channel_nextMessageIndex, "f")];
            __classPrivateFieldSet(this, _Channel_nextMessageIndex, __classPrivateFieldGet(this, _Channel_nextMessageIndex, "f") + 1, "f");
          }
          if (__classPrivateFieldGet(this, _Channel_nextMessageIndex, "f") === __classPrivateFieldGet(this, _Channel_messageEndIndex, "f")) {
            this.cleanupCallback();
          }
        } else {
          __classPrivateFieldGet(this, _Channel_pendingMessages, "f")[index] = message;
        }
      });
    }
    cleanupCallback() {
      window.__TAURI_INTERNALS__.unregisterCallback(this.id);
    }
    set onmessage(handler) {
      __classPrivateFieldSet(this, _Channel_onmessage, handler, "f");
    }
    get onmessage() {
      return __classPrivateFieldGet(this, _Channel_onmessage, "f");
    }
    [(_Channel_onmessage = /* @__PURE__ */ new WeakMap(), _Channel_nextMessageIndex = /* @__PURE__ */ new WeakMap(), _Channel_pendingMessages = /* @__PURE__ */ new WeakMap(), _Channel_messageEndIndex = /* @__PURE__ */ new WeakMap(), SERIALIZE_TO_IPC_FN)]() {
      return `__CHANNEL__:${this.id}`;
    }
    toJSON() {
      return this[SERIALIZE_TO_IPC_FN]();
    }
  };
  async function invoke(cmd, args = {}, options) {
    return window.__TAURI_INTERNALS__.invoke(cmd, args, options);
  }
  _Resource_rid = /* @__PURE__ */ new WeakMap();

  // ts/components/grid.ts
  var KeystreamGrid = class {
    constructor(container) {
      this.cells = [];
      this.container = container;
      this.container.classList.add("keystream-grid");
      this.render();
    }
    render() {
      this.container.innerHTML = "";
      this.cells = [];
      for (let i = 0; i < 64; i++) {
        const cell = document.createElement("div");
        cell.className = "keystream-cell";
        cell.title = `Byte ${i}: Empty`;
        this.container.appendChild(cell);
        this.cells.push(cell);
      }
    }
    update(bytes) {
      for (let i = 0; i < 64; i++) {
        const cell = this.cells[i];
        if (i < bytes.length) {
          const val = bytes[i];
          cell.style.backgroundColor = `rgb(${val}, ${val}, ${val})`;
          let hex = val.toString(16).toUpperCase();
          if (hex.length < 2)
            hex = "0" + hex;
          cell.title = `Byte ${i}: 0x${hex} (${val})`;
        } else {
          cell.style.backgroundColor = "transparent";
          cell.title = `Byte ${i}: Empty`;
        }
      }
    }
  };

  // ts/use-tab.ts
  function initUseTab() {
    const inputEl = document.getElementById("use-input");
    const hexCheckEl = document.getElementById("use-hex-check");
    const encryptBtn = document.getElementById("use-encrypt-btn");
    const outputEl = document.getElementById("use-output");
    const gridContainer = document.getElementById("use-keystream-grid");
    const helpLink = document.getElementById("use-help-link");
    if (!inputEl || !encryptBtn || !outputEl || !gridContainer)
      return;
    const grid = new KeystreamGrid(gridContainer);
    let isLoading = false;
    const encrypt = async () => {
      if (isLoading)
        return;
      const plaintext = inputEl.value;
      if (!plaintext)
        return;
      isLoading = true;
      encryptBtn.disabled = true;
      encryptBtn.textContent = "Encrypting...";
      try {
        const hex_output = hexCheckEl.checked;
        const response = await invoke("encrypt_decrypt", {
          plaintext,
          hexOutput: hex_output
        });
        outputEl.value = response.ciphertext;
        grid.update(response.keystream_bytes);
      } catch (e) {
        console.error(e);
        outputEl.value = `Error: ${e}`;
      } finally {
        isLoading = false;
        encryptBtn.disabled = false;
        encryptBtn.textContent = "\u{1F512} Encrypt / Decrypt";
      }
    };
    encryptBtn.addEventListener("click", encrypt);
    if (helpLink) {
      helpLink.addEventListener("click", (e) => {
        e.preventDefault();
        document.querySelector('[data-tab="learn"]')?.dispatchEvent(new Event("click"));
        events.emit("navigate_learn_xor", inputEl.value);
      });
    }
  }

  // ts/components/progress-bar.ts
  var ProgressBar = class {
    constructor(container, label, tooltipText = null, colorThresholds = { green: 0.8, orange: 0.6 }) {
      this.label = label;
      this.tooltipText = tooltipText;
      this.colorThresholds = colorThresholds;
      this.container = container;
      this.container.classList.add("progress-bar-container", "flex-row");
      this.labelEl = document.createElement("div");
      this.labelEl.className = "progress-label";
      this.labelEl.textContent = label;
      const trackEl = document.createElement("div");
      trackEl.className = "progress-track";
      this.fillEl = document.createElement("div");
      this.fillEl.className = "progress-fill";
      trackEl.appendChild(this.fillEl);
      this.valueEl = document.createElement("div");
      this.valueEl.className = "progress-value";
      this.container.appendChild(this.labelEl);
      this.container.appendChild(trackEl);
      this.container.appendChild(this.valueEl);
      if (tooltipText) {
        const tooltip = document.createElement("span");
        tooltip.className = "tooltip-icon";
        tooltip.textContent = "\u2139";
        tooltip.setAttribute("data-tooltip", tooltipText);
        this.container.appendChild(tooltip);
      }
    }
    update(value, max, displayValue) {
      const fraction = value / max;
      this.fillEl.style.width = `${Math.min(100, fraction * 100)}%`;
      this.valueEl.textContent = displayValue;
      if (fraction >= this.colorThresholds.green) {
        this.fillEl.style.backgroundColor = "var(--accent-green)";
      } else if (fraction >= this.colorThresholds.orange) {
        this.fillEl.style.backgroundColor = "var(--accent-orange)";
      } else {
        this.fillEl.style.backgroundColor = "var(--accent-red)";
      }
    }
  };

  // ts/test-tab.ts
  function initTestTab() {
    const sizeSlider = document.getElementById("test-size-slider");
    const sizeLabel = document.getElementById("test-size-label");
    const runBtn = document.getElementById("test-run-btn");
    const resultsSection = document.getElementById("test-results-section");
    const metricsContainer = document.getElementById("test-metrics-container");
    const nistTableBody = document.getElementById("test-nist-tbody");
    if (!sizeSlider || !runBtn || !resultsSection)
      return;
    const updateSliderLabel = () => {
      const val = parseFloat(sizeSlider.value);
      const bytes = Math.round(Math.pow(10, val));
      sizeLabel.textContent = bytes.toLocaleString() + " bytes";
      return bytes;
    };
    sizeSlider.addEventListener("input", updateSliderLabel);
    updateSliderLabel();
    const shannonBar = new ProgressBar(
      document.getElementById("test-shannon-bar"),
      "Shannon Entropy",
      "Measures the unpredictability of data.\n8.0 bits/byte is perfect randomness.\nLower values mean patterns exist.",
      { green: 0.9375, orange: 0.75 }
      // 7.5/8, 6.0/8
    );
    const minBar = new ProgressBar(
      document.getElementById("test-min-bar"),
      "Min-Entropy",
      "The worst-case entropy.\nCrucial for security guarantees.\nIf this is low, an attacker might guess the key.",
      { green: 0.9375, orange: 0.75 }
    );
    const scoreBar = new ProgressBar(
      document.getElementById("test-score-bar"),
      "Overall Score",
      "Weighted quality score: Shannon entropy (50%), Min-entropy (30%), Mean closeness to 127.5 (20%).",
      { green: 0.8, orange: 0.6 }
      // out of 100
    );
    let isLoading = false;
    const runTests = async () => {
      if (isLoading)
        return;
      const bytes = updateSliderLabel();
      isLoading = true;
      runBtn.disabled = true;
      runBtn.textContent = "Running Tests...";
      resultsSection.classList.add("hidden");
      try {
        const response = await invoke("run_quality_tests", {
          sampleSize: bytes
        });
        shannonBar.update(response.shannon_entropy, 8, `${response.shannon_entropy.toFixed(4)} bits/byte`);
        minBar.update(response.min_entropy, 8, `${response.min_entropy.toFixed(4)} bits/byte`);
        scoreBar.update(response.overall_score, 100, `${response.overall_score.toFixed(1)} / 100`);
        document.getElementById("test-mean-val").textContent = `${response.mean.toFixed(2)} (ideal: 127.5)`;
        document.getElementById("test-chi-val").textContent = response.chi_square.toFixed(2);
        document.getElementById("test-run-val").textContent = `${response.longest_run} bits`;
        nistTableBody.innerHTML = "";
        for (const res of response.nist_results) {
          const tr = document.createElement("tr");
          const tdName = document.createElement("td");
          tdName.textContent = res.name;
          const tdPval = document.createElement("td");
          tdPval.textContent = res.p_value.toFixed(4);
          const tdResult = document.createElement("td");
          if (res.passed) {
            tdResult.innerHTML = '<span class="text-green">\u2713 Pass</span>';
          } else {
            tdResult.innerHTML = '<span class="text-red">\u2717 Fail</span>';
          }
          tr.appendChild(tdName);
          tr.appendChild(tdPval);
          tr.appendChild(tdResult);
          nistTableBody.appendChild(tr);
        }
        resultsSection.classList.remove("hidden");
      } catch (e) {
        console.error(e);
        alert(`Error running tests: ${e}`);
      } finally {
        isLoading = false;
        runBtn.disabled = false;
        runBtn.textContent = "\u{1F52C} Run All Tests";
      }
    };
    runBtn.addEventListener("click", runTests);
  }

  // ts/bench-tab.ts
  function initBenchTab() {
    const sizeSlider = document.getElementById("bench-size-slider");
    const sizeLabel = document.getElementById("bench-size-label");
    const runBtn = document.getElementById("bench-run-btn");
    const resultsSection = document.getElementById("bench-results-section");
    if (!sizeSlider || !runBtn || !resultsSection)
      return;
    const updateSliderLabel = () => {
      const val = parseFloat(sizeSlider.value);
      const bytes = Math.round(Math.pow(10, val));
      sizeLabel.textContent = bytes.toLocaleString() + " bytes";
      return bytes;
    };
    sizeSlider.addEventListener("input", updateSliderLabel);
    updateSliderLabel();
    let isLoading = false;
    const runBenchmark = async () => {
      if (isLoading)
        return;
      const bytes = updateSliderLabel();
      isLoading = true;
      runBtn.disabled = true;
      runBtn.textContent = "Benchmarking...";
      resultsSection.classList.add("hidden");
      try {
        const response = await invoke("run_benchmark", { bytes });
        document.getElementById("bench-throughput-val").textContent = `${response.throughput_mbps.toFixed(1)} MB/s`;
        document.getElementById("bench-latency-val").textContent = `${response.latency_us.toFixed(2)} \xB5s/byte`;
        document.getElementById("bench-bytes-val").textContent = response.bytes_generated.toLocaleString();
        document.getElementById("bench-duration-val").textContent = `${response.duration_secs.toFixed(2)} s`;
        resultsSection.classList.remove("hidden");
      } catch (e) {
        console.error(e);
        alert(`Error running benchmark: ${e}`);
      } finally {
        isLoading = false;
        runBtn.disabled = false;
        runBtn.textContent = "\u26A1 Run Benchmark";
      }
    };
    runBtn.addEventListener("click", runBenchmark);
  }

  // ts/components/controls.ts
  var Controls = class {
    constructor(containerId, onPrev, onNext, onTogglePlay, onSpeedChange) {
      this.onPrev = onPrev;
      this.onNext = onNext;
      this.onTogglePlay = onTogglePlay;
      this.onSpeedChange = onSpeedChange;
      this.currentIndex = 0;
      this.totalSteps = 0;
      this.isPlaying = false;
      this.speed = 1;
      this.container = document.getElementById(containerId);
      this.container.innerHTML = `
      <div class="flex-row">
        <button class="control-btn prev-btn">\u25C0 Prev</button>
        <button class="control-btn play-btn">\u25B6 Play</button>
        <button class="control-btn next-btn">\u25B6\u25B6 Next</button>
        <div class="spacer"></div>
        <label>Speed:</label>
        <input type="range" class="speed-slider" min="0.1" max="5.0" step="0.1" value="1.0">
        <span class="speed-label font-mono">1.0 steps/s</span>
      </div>
    `;
      this.prevBtn = this.container.querySelector(".prev-btn");
      this.playBtn = this.container.querySelector(".play-btn");
      this.nextBtn = this.container.querySelector(".next-btn");
      const speedSlider = this.container.querySelector(".speed-slider");
      this.speedLabel = this.container.querySelector(".speed-label");
      this.prevBtn.addEventListener("click", () => {
        if (this.currentIndex > 0)
          this.onPrev();
      });
      this.nextBtn.addEventListener("click", () => {
        if (this.currentIndex < this.totalSteps - 1)
          this.onNext();
      });
      this.playBtn.addEventListener("click", () => {
        this.onTogglePlay();
        this.updateState(this.currentIndex, this.totalSteps, this.isPlaying);
      });
      speedSlider.addEventListener("input", (e) => {
        const target = e.target;
        this.speed = parseFloat(target.value);
        this.speedLabel.textContent = `${this.speed.toFixed(1)} steps/s`;
        this.onSpeedChange(this.speed);
      });
    }
    updateState(current, total, playing) {
      this.currentIndex = current;
      this.totalSteps = total;
      this.isPlaying = playing;
      this.prevBtn.disabled = this.currentIndex === 0;
      this.nextBtn.disabled = this.currentIndex >= this.totalSteps - 1;
      if (this.isPlaying) {
        this.playBtn.textContent = "\u23F8 Pause";
      } else {
        this.playBtn.textContent = "\u25B6 Play";
      }
    }
  };

  // ts/learn/xor.ts
  function initXorVisualizer() {
    const inputEl = document.getElementById("xor-input");
    const startBtn = document.getElementById("xor-start-btn");
    const vizSection = document.getElementById("xor-viz-section");
    const stepHeader = document.getElementById("xor-step-header");
    const inCharEl = document.getElementById("xor-in-char");
    const inDecEl = document.getElementById("xor-in-dec");
    const inBinEl = document.getElementById("xor-in-bin");
    const keyDecEl = document.getElementById("xor-key-dec");
    const keyBinEl = document.getElementById("xor-key-bin");
    const outDecEl = document.getElementById("xor-out-dec");
    const outBinEl = document.getElementById("xor-out-bin");
    const progressContainer = document.getElementById("xor-progress");
    if (!startBtn)
      return;
    let steps = [];
    let currentIndex = 0;
    let timerId = null;
    let controls;
    const renderStep = () => {
      if (steps.length === 0)
        return;
      const step = steps[currentIndex];
      stepHeader.textContent = `Step ${currentIndex + 1} of ${steps.length}: Encrypting '${step.character}'`;
      inCharEl.textContent = `'${step.character}'`;
      inDecEl.textContent = `(${step.input_byte})`;
      inBinEl.textContent = step.input_binary;
      keyDecEl.textContent = `(${step.keystream_byte})`;
      keyBinEl.textContent = step.keystream_binary;
      outDecEl.textContent = `(${step.result_byte})`;
      outBinEl.textContent = step.result_binary;
      progressContainer.innerHTML = "";
      steps.forEach((s, i) => {
        const badge = document.createElement("span");
        let hex = s.result_byte.toString(16).toUpperCase();
        if (hex.length < 2)
          hex = "0" + hex;
        badge.textContent = `[${hex}]`;
        badge.className = "xor-badge";
        if (i < currentIndex) {
          badge.classList.add("past");
        } else if (i === currentIndex) {
          badge.classList.add("current");
        } else {
          badge.classList.add("future");
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
      if (currentIndex >= steps.length - 1)
        currentIndex = 0;
      timerId = window.setInterval(playNext, 1e3 / controls.speed);
    };
    controls = new Controls(
      "xor-controls",
      () => {
        currentIndex--;
        renderStep();
      },
      () => {
        currentIndex++;
        renderStep();
      },
      () => {
        if (timerId !== null)
          stopTimer();
        else
          startTimer();
      },
      (speed) => {
        if (timerId !== null)
          startTimer();
      }
    );
    startBtn.addEventListener("click", async () => {
      const text = inputEl.value;
      if (!text)
        return;
      startBtn.disabled = true;
      startBtn.textContent = "Loading...";
      try {
        const response = await invoke("get_xor_steps", { text });
        steps = response.steps;
        currentIndex = 0;
        stopTimer();
        vizSection.classList.remove("hidden");
        renderStep();
      } catch (e) {
        alert(`Error: ${e}`);
      } finally {
        startBtn.disabled = false;
        startBtn.textContent = "Start Visualization";
      }
    });
    events.on("navigate_learn_xor", (text) => {
      inputEl.value = text;
      document.querySelector('.sub-tab-btn[data-subtab="xor"]')?.dispatchEvent(new Event("click"));
      startBtn.click();
    });
  }

  // ts/learn/entropy.ts
  function initEntropyVisualizer() {
    const inputEl = document.getElementById("entropy-input");
    const startBtn = document.getElementById("entropy-calc-btn");
    const vizSection = document.getElementById("entropy-viz-section");
    const stepHeader = document.getElementById("entropy-step-header");
    const contentArea = document.getElementById("entropy-content-area");
    if (!startBtn)
      return;
    let steps = [];
    let currentIndex = 0;
    let timerId = null;
    let controls;
    const renderStep = () => {
      if (steps.length === 0)
        return;
      const step = steps[currentIndex];
      const titles = {
        "CountBytes": "Count Byte Frequencies",
        "CalculateProbabilities": "Calculate Probabilities",
        "CalculateContributions": "Calculate Contributions",
        "SumEntropy": "Sum for Total Entropy",
        "Interpret": "Interpretation"
      };
      stepHeader.textContent = `Step ${currentIndex + 1} of ${steps.length}: ${titles[step.step_type] || step.step_type}`;
      let html = "";
      if (["CountBytes", "CalculateProbabilities", "CalculateContributions"].includes(step.step_type)) {
        html += `
        <table class="entropy-table">
          <thead>
            <tr>
              <th>Byte</th>
              <th>Count</th>
              ${step.step_type !== "CountBytes" ? "<th>Probability</th>" : ""}
              ${step.step_type === "CalculateContributions" ? "<th>Contribution</th>" : ""}
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
          const width = count / maxCount * 100;
          let charRepr = parseInt(k);
          let display = `'${String.fromCharCode(charRepr)}' (${charRepr})`;
          if (charRepr < 32 || charRepr > 126)
            display = `0x${charRepr.toString(16)}`;
          html += `<tr>
          <td>${display}</td>
          <td>${count}</td>
          ${step.step_type !== "CountBytes" ? `<td>${p.toFixed(4)}</td>` : ""}
          ${step.step_type === "CalculateContributions" ? `<td>${c.toFixed(4)} bits</td>` : ""}
          <td><div class="entropy-bar" style="width: ${width}%"></div></td>
        </tr>`;
        }
        html += `</tbody></table>`;
      } else {
        const eff = step.max_entropy > 0 ? step.current_entropy_sum / step.max_entropy * 100 : 0;
        html += `
        <div class="entropy-summary">
          <div class="entropy-val-large">Total Entropy: ${step.current_entropy_sum.toFixed(4)} bits/byte</div>
          <div class="text-muted">Maximum Possible: ${step.max_entropy.toFixed(4)} bits/byte</div>
          <div class="text-muted">Efficiency: ${eff.toFixed(1)}%</div>
        </div>
      `;
        if (step.step_type === "Interpret") {
          html += `<div class="entropy-interpret">`;
          if (step.current_entropy_sum < 2) {
            html += `<span class="text-red">Low entropy. The input is very predictable.</span>`;
          } else if (eff > 80) {
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
      if (currentIndex >= steps.length - 1)
        currentIndex = 0;
      timerId = window.setInterval(playNext, 1e3 / controls.speed);
    };
    controls = new Controls(
      "entropy-controls",
      () => {
        currentIndex--;
        renderStep();
      },
      () => {
        currentIndex++;
        renderStep();
      },
      () => {
        if (timerId !== null)
          stopTimer();
        else
          startTimer();
      },
      (speed) => {
        if (timerId !== null)
          startTimer();
      }
    );
    startBtn.addEventListener("click", async () => {
      const text = inputEl.value;
      if (!text)
        return;
      startBtn.disabled = true;
      startBtn.textContent = "Loading...";
      try {
        const response = await invoke("get_entropy_steps", { text });
        steps = response.steps;
        currentIndex = 0;
        stopTimer();
        vizSection.classList.remove("hidden");
        renderStep();
      } catch (e) {
        alert(`Error: ${e}`);
      } finally {
        startBtn.disabled = false;
        startBtn.textContent = "Calculate";
      }
    });
  }

  // ts/learn/nist.ts
  function initNistVisualizer() {
    const inputEl = document.getElementById("nist-input");
    const analyzeBtn = document.getElementById("nist-analyze-btn");
    const randomBtn = document.getElementById("nist-random-btn");
    const vizSection = document.getElementById("nist-viz-section");
    const stepHeader = document.getElementById("nist-step-header");
    const contentArea = document.getElementById("nist-content-area");
    if (!analyzeBtn)
      return;
    let steps = [];
    let currentIndex = 0;
    let timerId = null;
    let controls;
    const renderStep = () => {
      if (steps.length === 0)
        return;
      const step = steps[currentIndex];
      const titles = {
        "ConvertToBits": "Convert to Bits",
        "CountOnesZeros": "Count Ones and Zeros",
        "CalculateStatistic": "Calculate Statistic",
        "CalculatePValue": "Calculate P-Value",
        "Interpret": "Interpretation"
      };
      stepHeader.textContent = `Step ${currentIndex + 1} of ${steps.length}: ${titles[step.step_type] || step.step_type}`;
      let html = "";
      html += `<div class="nist-bits font-mono">`;
      for (let i = 0; i < step.bits.length; i++) {
        if (i > 0 && i % 8 === 0)
          html += " ";
        html += `<span class="${step.bits[i] === 1 ? "text-green" : "text-muted"}">${step.bits[i]}</span>`;
      }
      html += `</div>`;
      if (step.step_type !== "ConvertToBits") {
        const total = step.bits.length;
        const onesPct = total > 0 ? step.ones_count / total * 100 : 0;
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
          <div>S_obs (|Sum| / \u221An): <strong>${step.s_obs.toFixed(4)}</strong></div>
      `;
        if (step.step_type === "CalculatePValue" || step.step_type === "Interpret") {
          html += `<div>P-Value (erfc(S_obs/\u221A2)): <strong>${step.p_value.toFixed(4)}</strong></div>`;
        }
        if (step.step_type === "Interpret") {
          if (step.passed) {
            html += `<div class="text-green" style="margin-top: 10px;">\u2705 PASS: The sequence looks random.</div>`;
          } else {
            html += `<div class="text-red" style="margin-top: 10px;">\u274C FAIL: The sequence has too many 1s or 0s.</div>`;
          }
          html += `<div class="text-muted text-sm">(Threshold: P-value \u2265 0.01)</div>`;
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
      if (currentIndex >= steps.length - 1)
        currentIndex = 0;
      timerId = window.setInterval(playNext, 1e3 / controls.speed);
    };
    controls = new Controls(
      "nist-controls",
      () => {
        currentIndex--;
        renderStep();
      },
      () => {
        currentIndex++;
        renderStep();
      },
      () => {
        if (timerId !== null)
          stopTimer();
        else
          startTimer();
      },
      (speed) => {
        if (timerId !== null)
          startTimer();
      }
    );
    const handleResponse = (response) => {
      steps = response.steps;
      if (response.input_text)
        inputEl.value = response.input_text;
      currentIndex = 0;
      stopTimer();
      vizSection.classList.remove("hidden");
      renderStep();
    };
    analyzeBtn.addEventListener("click", async () => {
      const text = inputEl.value;
      if (!text)
        return;
      analyzeBtn.disabled = true;
      try {
        const response = await invoke("get_nist_steps", { text });
        handleResponse(response);
      } catch (e) {
        alert(e);
      } finally {
        analyzeBtn.disabled = false;
      }
    });
    randomBtn.addEventListener("click", async () => {
      randomBtn.disabled = true;
      try {
        const response = await invoke("get_nist_steps_random", { count: 16 });
        handleResponse(response);
      } catch (e) {
        alert(e);
      } finally {
        randomBtn.disabled = false;
      }
    });
  }

  // ts/learn/index.ts
  function initLearnTab() {
    const subTabBtns = document.querySelectorAll(".sub-tab-btn");
    const subTabPanels = document.querySelectorAll(".sub-tab-panel");
    subTabBtns.forEach((btn) => {
      btn.addEventListener("click", () => {
        subTabBtns.forEach((b) => b.classList.remove("active"));
        subTabPanels.forEach((p) => p.classList.add("hidden"));
        btn.classList.add("active");
        const targetId = `learn-${btn.getAttribute("data-subtab")}`;
        const targetPanel = document.getElementById(targetId);
        if (targetPanel) {
          targetPanel.classList.remove("hidden");
        }
      });
    });
    initXorVisualizer();
    initEntropyVisualizer();
    initNistVisualizer();
  }

  // ts/main.ts
  function initTabs() {
    const tabBtns = document.querySelectorAll(".tab-btn");
    const tabPanels = document.querySelectorAll(".tab-panel");
    tabBtns.forEach((btn) => {
      btn.addEventListener("click", () => {
        tabBtns.forEach((b) => b.classList.remove("active"));
        tabPanels.forEach((p) => p.classList.add("hidden"));
        btn.classList.add("active");
        const targetId = `tab-${btn.getAttribute("data-tab")}`;
        const targetPanel = document.getElementById(targetId);
        if (targetPanel) {
          targetPanel.classList.remove("hidden");
        }
      });
    });
  }
  var events = {
    listeners: {},
    on(event, callback) {
      if (!this.listeners[event])
        this.listeners[event] = [];
      this.listeners[event].push(callback);
    },
    emit(event, data) {
      if (this.listeners[event]) {
        this.listeners[event].forEach((cb) => cb(data));
      }
    }
  };
  document.addEventListener("DOMContentLoaded", () => {
    initTabs();
    initUseTab();
    initTestTab();
    initBenchTab();
    initLearnTab();
  });
})();
