import { invoke } from "@tauri-apps/api/core";

const safeInvoke = async (command: string, args: any = {}) => {
    try {
        if ((window as any).__TAURI_INTERNALS__) {
            return await invoke(command, args);
        } else {
            console.warn(`[Tauri Mock] Called invoke('${command}') with args:`, args);
            throw new Error("Tauri API is not available in the browser.");
        }
    } catch (e) {
        throw e;
    }
};

document.addEventListener("DOMContentLoaded", () => {
    // ==========================================
    // 1. Tab Switching (Sidebar & Bottom Nav)
    // ==========================================
    const tabLinks = document.querySelectorAll('.tab-link, .mobile-tab-link');
    const tabPanels = document.querySelectorAll('.tab-panel');

    function switchTab(targetId: string) {
        tabPanels.forEach(panel => {
            panel.classList.remove('active');
        });

        const targetPanel = document.getElementById(targetId);
        if (targetPanel) {
            targetPanel.classList.add('active');
        }

        document.querySelectorAll('.tab-link').forEach(link => {
            if (link.getAttribute('data-target') === targetId) {
                link.classList.remove('text-slate-500');
                link.classList.add('text-[#00FF88]', 'bg-[#00FF88]/10', 'border-l-2', 'border-[#00FF88]');
                const icon = link.querySelector('.material-symbols-outlined') as HTMLElement;
                if (icon) icon.style.fontVariationSettings = "'FILL' 1";
            } else {
                link.classList.add('text-slate-500');
                link.classList.remove('text-[#00FF88]', 'bg-[#00FF88]/10', 'border-l-2', 'border-[#00FF88]');
                const icon = link.querySelector('.material-symbols-outlined') as HTMLElement;
                if (icon) icon.style.fontVariationSettings = "'FILL' 0";
            }
        });

        document.querySelectorAll('.mobile-tab-link').forEach(link => {
            if (link.getAttribute('data-target') === targetId) {
                link.classList.remove('text-gray-500');
                link.classList.add('text-[#00FF88]', 'border-t-2', 'border-[#00FF88]');
                const icon = link.querySelector('.material-symbols-outlined') as HTMLElement;
                if (icon) icon.style.fontVariationSettings = "'FILL' 1";
            } else {
                link.classList.add('text-gray-500');
                link.classList.remove('text-[#00FF88]', 'border-t-2', 'border-[#00FF88]');
                const icon = link.querySelector('.material-symbols-outlined') as HTMLElement;
                if (icon) icon.style.fontVariationSettings = "'FILL' 0";
            }
        });
    }

    tabLinks.forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            const target = (e.currentTarget as HTMLElement).getAttribute('data-target');
            if (target) switchTab(target);
        });
    });

    // ==========================================
    // 2. Use Tab Logic
    // ==========================================
    const useInput = document.getElementById('use-input') as HTMLTextAreaElement;
    const useLength = document.getElementById('use-input-length');
    const useHexCheck = document.getElementById('use-hex-check') as HTMLInputElement;
    const useOutput = document.getElementById('use-output') as HTMLTextAreaElement;
    const useEncryptBtn = document.getElementById('use-encrypt-btn');
    const useKeystreamGrid = document.getElementById('use-keystream-grid');

    if (useInput && useLength) {
        useInput.addEventListener('input', () => {
            useLength.textContent = `LENGTH: ${useInput.value.length} BYTES`;
        });
    }

    if (useEncryptBtn && useInput && useOutput && useHexCheck && useKeystreamGrid) {
        useEncryptBtn.addEventListener('click', async () => {
            const text = useInput.value;
            if (!text) return;

            try {
                const result: any = await safeInvoke('encrypt_decrypt', {
                    plaintext: text,
                    hexOutput: useHexCheck.checked
                });

                useOutput.value = result.ciphertext;

                // Update Keystream Grid (dynamic based on input length)
                useKeystreamGrid.innerHTML = '';
                const bytes = result.keystream_bytes || [];
                const inputLength = text.length;
                
                // Calculate grid dimensions (aim for square or close to it)
                const cols = Math.ceil(Math.sqrt(inputLength));
                useKeystreamGrid.style.gridTemplateColumns = `repeat(${cols}, minmax(0, 1fr))`;
                
                for (let i = 0; i < inputLength; i++) {
                    const div = document.createElement('div');

                    const val = bytes[i] || 0;
                    // Grayscale: byte value (0-255) maps to RGB intensity (0-255)
                    const gray = Math.round((val / 255) * 255);
                    const rgbColor = `rgb(${gray}, ${gray}, ${gray})`;
                    div.style.backgroundColor = rgbColor;
                    div.style.border = '1px solid rgba(132, 149, 133, 0.3)';
                    div.style.aspectRatio = '1';
                    div.className = "w-full h-full cursor-pointer transition-opacity hover:opacity-80";
                    
                    // Add tooltip with byte value
                    div.title = `Byte ${i}: 0x${val.toString(16).toUpperCase().padStart(2, '0')} (${val})`;

                    useKeystreamGrid.appendChild(div);
                }
            } catch (error) {
                console.error(error);
                useOutput.value = `Error: ${error}`;
            }
        });
    }

const useHelpLink = document.getElementById('use-help-link');
if (useHelpLink) {
    useHelpLink.addEventListener('click', () => {
        switchTab('tab-learn');
        // Pre-fill the learn input with use input
        const learnInput = document.getElementById('learn-input') as HTMLInputElement;
        if (learnInput && useInput && useInput.value) {
            learnInput.value = useInput.value;
            // dispatch input event to trigger loadXorSteps
            learnInput.dispatchEvent(new Event('input'));
        }
    });
}

    // ==========================================
    // 3. Test Tab Logic
    // ==========================================
    const testSlider = document.getElementById('test-size-slider') as HTMLInputElement;
    const testLabel = document.getElementById('test-size-label');
    const testRunBtn = document.getElementById('test-run-btn');
    const testResultsSec = document.getElementById('test-results-section');

    if (testSlider && testLabel) {
        testSlider.addEventListener('input', () => {
            const val = parseFloat(testSlider.value);
            const bytes = Math.round(Math.pow(10, val));
            testLabel.textContent = `${bytes.toLocaleString()} BYTES`;
        });
    }

    if (testRunBtn && testSlider && testResultsSec) {
        testRunBtn.addEventListener('click', async () => {
            const val = parseFloat(testSlider.value);
            const bytes = Math.round(Math.pow(10, val));

            // Show loading state
            testResultsSec.classList.remove('hidden');
            const tbody = document.getElementById('test-nist-tbody');
            if (tbody) tbody.innerHTML = '<tr><td colspan="3" class="text-center py-4">Running tests...</td></tr>';

            try {
                const result: any = await safeInvoke('run_quality_tests', {
                    sampleSize: bytes
                });

                // Update metrics
                document.getElementById('test-shannon-val')!.textContent = result.shannon_entropy.toFixed(4);
                document.getElementById('test-shannon-bar')!.style.width = `${(result.shannon_entropy / 8.0) * 100}%`;

                document.getElementById('test-min-val')!.textContent = result.min_entropy.toFixed(4);
                document.getElementById('test-min-bar')!.style.width = `${(result.min_entropy / 8.0) * 100}%`;

                document.getElementById('test-mean-val')!.textContent = result.mean.toFixed(3);
                document.getElementById('test-chi-val')!.textContent = result.chi_square.toFixed(2);
                document.getElementById('test-longest-run-val')!.textContent = result.longest_run.toString();

                // Update NIST Table
                if (tbody) {
                    tbody.innerHTML = '';
                    result.nist_results.forEach((t: any) => {
                        const row = document.createElement('tr');
                        row.className = "border-b border-outline-variant/10 hover:bg-surface-container-low transition-colors group";

                        const statusHtml = t.passed
                            ? `<span class="bg-[#00FF88]/10 text-[#00FF88] border border-[#00FF88]/30 px-3 py-1 font-bold">✓ PASS</span>`
                            : `<span class="bg-error-container/20 text-error border border-error/30 px-3 py-1 font-bold">✗ FAIL</span>`;

                        row.innerHTML = `
                            <td class="px-6 py-4 font-bold uppercase tracking-tight">${t.name}</td>
                            <td class="px-6 py-4 font-mono">${t.p_value.toFixed(6)}</td>
                            <td class="px-6 py-4 text-right">${statusHtml}</td>
                        `;
                        tbody.appendChild(row);
                    });
                }
            } catch (error) {
                console.error(error);
                if (tbody) tbody.innerHTML = `<tr><td colspan="3" class="text-center text-error py-4">Error: ${error}</td></tr>`;
            }
        });
    }

    // ==========================================
    // 4. Bench Tab Logic
    // ==========================================
    const benchSlider = document.getElementById('bench-volume-slider') as HTMLInputElement;
    const benchLabel = document.getElementById('bench-volume-label');
    const benchRunBtn = document.getElementById('bench-run-btn');

    if (benchSlider && benchLabel) {
        benchSlider.addEventListener('input', () => {
            benchLabel.textContent = `${benchSlider.value} MB`;
        });
    }

    if (benchRunBtn && benchSlider) {
        benchRunBtn.addEventListener('click', async () => {
            const mbs = parseInt(benchSlider.value, 10);
            const bytes = mbs * 1024 * 1024;

            const durationVal = document.getElementById('bench-duration-val');
            const bytesVal = document.getElementById('bench-bytes-val');
            if (durationVal) durationVal.textContent = '...';
            if (bytesVal) bytesVal.textContent = '...';

            try {
                const result: any = await safeInvoke('run_benchmark', {
                    bytes: bytes
                });

                document.getElementById('bench-throughput')!.textContent = result.throughput_mbps.toFixed(1);
                document.getElementById('bench-latency')!.textContent = result.latency_us.toFixed(3);

                // Animate bar (fake peak calculation)
                const peak = Math.min((result.throughput_mbps / 2000.0) * 100, 100);
                document.getElementById('bench-throughput-bar')!.style.width = `${peak}%`;

                if (durationVal) durationVal.textContent = `${result.duration_secs.toFixed(4)}s`;
                if (bytesVal) bytesVal.textContent = result.bytes_generated.toLocaleString();
            } catch (error) {
                console.error(error);
                if (durationVal) durationVal.textContent = `Error`;
                if (bytesVal) bytesVal.textContent = `Error`;
            }
        });
    }

    // ==========================================
    // 5. Learn Tab Logic (Sub-tabs)
    // ==========================================
    const learnBtns = document.querySelectorAll('.learn-btn');
    const learnSections = document.querySelectorAll('.learn-section');

    learnBtns.forEach(btn => {
        btn.addEventListener('click', (e) => {
            const targetLearn = (e.currentTarget as HTMLElement).getAttribute('data-learn');

            learnBtns.forEach(b => {
                b.classList.remove('active', 'bg-primary-container', 'text-on-primary-container');
                b.classList.add('bg-surface-container', 'text-slate-400');
            });
            (e.currentTarget as HTMLElement).classList.remove('bg-surface-container', 'text-slate-400');
            (e.currentTarget as HTMLElement).classList.add('active', 'bg-primary-container', 'text-on-primary-container');

             learnSections.forEach(section => {
                 if (section.id === `learn-${targetLearn}`) {
                     section.classList.add('active');
                 } else {
                     section.classList.remove('active');
                 }
             });
        });
    });

    // ==========================================
    // 6. Learn XOR Logic
    // ==========================================
    let currentXorSteps: any[] = [];
    let currentXorStepIdx = 0;
    let playInterval: any = null;

    const learnInput = document.getElementById('learn-input') as HTMLInputElement;
    const btnXorPrev = document.getElementById('learn-prev');
    const btnXorNext = document.getElementById('learn-next');
    const btnXorPlayPause = document.getElementById('learn-play-pause');
    const learnSpeedSlider = document.getElementById('learn-speed-slider') as HTMLInputElement;
    const learnSpeedText = document.getElementById('learn-speed-text');
    const xorStepInfo = document.getElementById('learn-xor-step-info');
    const xorContent = document.getElementById('learn-xor-content');
    const xorProgText = document.getElementById('learn-progress-text');
    const xorProgSlider = document.getElementById('learn-progress-slider') as HTMLInputElement;

    async function loadXorSteps() {
        if (!learnInput.value) return;
        try {
            const result: any = await safeInvoke('get_xor_steps', { text: learnInput.value });
            currentXorSteps = result.steps;
            currentXorStepIdx = 0;
            if (playInterval) togglePlayPause(); // Pause if loading new
            renderXorStep();
        } catch (e) {
            console.error(e);
        }
    }

    function renderXorStep() {
        if (currentXorSteps.length === 0) {
            if(xorContent) xorContent.innerHTML = "No steps.";
            return;
        }

        const step = currentXorSteps[currentXorStepIdx];
        if (xorStepInfo) xorStepInfo.textContent = `STEP_IDX: ${(currentXorStepIdx + 1).toString().padStart(3, '0')} / ${currentXorSteps.length.toString().padStart(3, '0')}`;

        let prog = Math.round(((currentXorStepIdx + 1) / currentXorSteps.length) * 100);
        if (xorProgText) xorProgText.textContent = `${prog}%`;
        if (xorProgSlider) xorProgSlider.value = prog.toString();

        if (xorContent) {
            // Render HTML for XOR visualization
            let ptBitsHtml = step.input_binary.split('').map((b: string) => `<span class="w-6 h-8 bg-surface-container-highest flex items-center justify-center font-bold text-primary">${b}</span>`).join('');
            let keyBitsHtml = step.keystream_binary.split('').map((b: string) => `<span class="w-6 h-8 bg-surface-container-highest flex items-center justify-center font-bold text-[#A855F7]">${b}</span>`).join('');
            let resBitsHtml = step.result_binary.split('').map((b: string) => `<span class="w-6 h-8 bg-primary-container/10 border border-primary-container/30 flex items-center justify-center font-black text-primary-container">${b}</span>`).join('');

            xorContent.innerHTML = `
            <div class="bg-surface-container-low p-4 relative overflow-hidden border-l-2 border-[#00FF88]">
                <div class="flex justify-between items-start mb-4">
                    <div>
                        <span class="text-[10px] text-secondary uppercase font-bold">Input Character</span>
                        <div class="text-4xl font-black text-primary">'${step.character}'</div>
                    </div>
                    <div class="text-right">
                        <span class="text-[10px] text-outline-variant uppercase">ASCII Decimal</span>
                        <div class="text-xl font-bold">${step.input_byte}</div>
                    </div>
                </div>

                <div class="space-y-4 pt-4 border-t border-outline-variant/30">
                    <div class="flex items-center justify-between">
                        <span class="text-[10px] text-outline text-left uppercase w-20">Plaintext</span>
                        <div class="flex gap-1">${ptBitsHtml}</div>
                    </div>

                    <div class="flex items-center gap-4">
                        <div class="h-[1px] flex-grow bg-outline-variant"></div>
                        <span class="text-[#A855F7] font-black italic">⊕ XOR</span>
                        <div class="h-[1px] flex-grow bg-outline-variant"></div>
                    </div>

                    <div class="flex items-center justify-between">
                        <span class="text-[10px] text-[#A855F7] text-left uppercase w-20">Keystream</span>
                        <div class="flex gap-1">${keyBitsHtml}</div>
                    </div>

                    <div class="flex items-center justify-between pt-4 border-t-2 border-primary-container">
                        <span class="text-[10px] text-primary-container text-left uppercase w-20 font-bold">Result</span>
                        <div class="flex gap-1">${resBitsHtml}</div>
                    </div>
                </div>
            </div>
            `;
        }
    }

    if (learnInput) {
        learnInput.addEventListener('input', () => {
            loadXorSteps();
        });
        // Initial load
        setTimeout(loadXorSteps, 500);
    }

    if (btnXorPrev) {
        btnXorPrev.addEventListener('click', () => {
            if (currentXorStepIdx > 0) {
                currentXorStepIdx--;
                renderXorStep();
            }
        });
    }

    if (btnXorNext) {
        btnXorNext.addEventListener('click', () => {
            if (currentXorStepIdx < currentXorSteps.length - 1) {
                currentXorStepIdx++;
                renderXorStep();
            } else if (playInterval) {
                togglePlayPause(); // pause if reached end
            }
        });
    }

    function togglePlayPause() {
        if (!btnXorPlayPause) return;
        const icon = btnXorPlayPause.querySelector('.material-symbols-outlined') as HTMLElement;

        if (playInterval) {
            clearInterval(playInterval);
            playInterval = null;
            if (icon) icon.textContent = 'play_arrow';
        } else {
            if (currentXorStepIdx >= currentXorSteps.length - 1) {
                currentXorStepIdx = 0; // restart if at end
            }
            const speedMs = parseInt(learnSpeedSlider.value, 10);
            playInterval = setInterval(() => {
                if (btnXorNext) btnXorNext.click();
            }, speedMs);
            if (icon) icon.textContent = 'pause';
        }
    }

    if (btnXorPlayPause) {
        btnXorPlayPause.addEventListener('click', togglePlayPause);
    }

    if (learnSpeedSlider && learnSpeedText) {
        learnSpeedSlider.addEventListener('input', () => {
            const speedSec = (parseInt(learnSpeedSlider.value, 10) / 1000).toFixed(1);
            learnSpeedText.textContent = `${speedSec}s / step`;
            if (playInterval) {
                // restart interval with new speed
                togglePlayPause();
                togglePlayPause();
            }
        });
    }

    // ==========================================
    // 7. Learn Entropy Logic
    // ==========================================
    let currentEntSteps: any[] = [];
    let currentEntStepIdx = 0;
    const learnEntInput = document.getElementById('learn-entropy-input') as HTMLInputElement;
    const btnEntPrev = document.getElementById('learn-entropy-prev');
    const btnEntNext = document.getElementById('learn-entropy-next');
    const entStepInfo = document.getElementById('learn-entropy-step-info');
    const entContent = document.getElementById('learn-entropy-content');

    async function loadEntSteps() {
        if (!learnEntInput.value) return;
        try {
            const result: any = await safeInvoke('get_entropy_steps', { text: learnEntInput.value });
            currentEntSteps = result.steps;
            currentEntStepIdx = 0;
            renderEntStep();
        } catch (e) { console.error(e); }
    }

    function renderEntStep() {
        if (currentEntSteps.length === 0) return;
        const step = currentEntSteps[currentEntStepIdx];
        if (entStepInfo) entStepInfo.textContent = `Step ${currentEntStepIdx + 1} / ${currentEntSteps.length}`;

        if (entContent) {
            let probs = '';
            for(const [k, v] of Object.entries(step.probabilities)) {
                let probVal = v as number;
                let colorClass = probVal > 0.1 ? 'text-[#00FF88]' : 'text-error';
                probs += `<div class="flex justify-between border-b border-outline-variant/30 py-1">
                            <span class="font-mono text-slate-300">byte[${k}]</span>
                            <span class="font-mono ${colorClass}">${probVal.toFixed(4)}</span>
                          </div>`;
            }

            entContent.innerHTML = `
                <div class="mb-4">
                    <span class="text-xs uppercase text-[#A855F7] tracking-widest font-bold block mb-1">State</span>
                    <span class="text-lg text-primary">${step.step_type}</span>
                </div>
                <div class="grid grid-cols-2 gap-8">
                    <div>
                        <h4 class="text-xs uppercase text-slate-500 mb-2">Probabilities</h4>
                        <div class="max-h-48 overflow-y-auto pr-2 no-scrollbar">
                            ${probs}
                        </div>
                    </div>
                    <div>
                        <h4 class="text-xs uppercase text-slate-500 mb-2">Current Entropy</h4>
                        <div class="text-4xl font-black text-primary-container">${step.current_entropy_sum.toFixed(4)}</div>
                        <div class="text-xs text-slate-500 mt-2">Maximum possible: ${step.max_entropy.toFixed(4)}</div>
                    </div>
                </div>
            `;
        }
    }

    if (learnEntInput) {
        learnEntInput.addEventListener('input', loadEntSteps);
        setTimeout(loadEntSteps, 500);
    }
    if (btnEntPrev) {
        btnEntPrev.addEventListener('click', () => { if (currentEntStepIdx > 0) { currentEntStepIdx--; renderEntStep(); } });
    }
    if (btnEntNext) {
        btnEntNext.addEventListener('click', () => { if (currentEntStepIdx < currentEntSteps.length - 1) { currentEntStepIdx++; renderEntStep(); } });
    }

    // ==========================================
    // 8. Learn NIST Logic
    // ==========================================
    let currentNistSteps: any[] = [];
    let currentNistStepIdx = 0;
    const btnNistPrev = document.getElementById('learn-nist-prev');
    const btnNistNext = document.getElementById('learn-nist-next');
    const nistStepInfo = document.getElementById('learn-nist-step-info');
    const nistContent = document.getElementById('learn-nist-content');

    async function loadNistSteps() {
        try {
            const result: any = await safeInvoke('get_nist_steps_random', { count: 64 });
            currentNistSteps = result.steps;
            currentNistStepIdx = 0;
            renderNistStep();
        } catch (e) { console.error(e); }
    }

    function renderNistStep() {
        if (currentNistSteps.length === 0) return;
        const step = currentNistSteps[currentNistStepIdx];
        if (nistStepInfo) nistStepInfo.textContent = `Step ${currentNistStepIdx + 1} / ${currentNistSteps.length}`;

        if (nistContent) {
            let bitsHtml = step.bits.map((b: number) => {
                const color = b === 1 ? 'text-[#00FF88]' : 'text-slate-500';
                return `<span class="inline-block w-4 text-center font-bold ${color}">${b}</span>`;
            }).join('');

            nistContent.innerHTML = `
                <div class="mb-4">
                    <span class="text-xs uppercase text-slate-500 tracking-widest font-bold block mb-1">State</span>
                    <span class="text-lg text-[#A855F7]">${step.step_type}</span>
                </div>

                <div class="bg-surface-container-highest p-4 font-mono break-all mb-6">
                    ${bitsHtml}
                </div>

                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <div class="bg-surface-container-low p-3">
                        <div class="text-[10px] text-slate-500 uppercase">1s Count</div>
                        <div class="text-xl font-bold">${step.ones_count}</div>
                    </div>
                    <div class="bg-surface-container-low p-3">
                        <div class="text-[10px] text-slate-500 uppercase">0s Count</div>
                        <div class="text-xl font-bold">${step.zeros_count}</div>
                    </div>
                    <div class="bg-surface-container-low p-3">
                        <div class="text-[10px] text-slate-500 uppercase">S_obs</div>
                        <div class="text-xl font-bold">${step.s_obs.toFixed(4)}</div>
                    </div>
                    <div class="bg-surface-container-low p-3 border-b-2 ${step.passed ? 'border-[#00FF88]' : 'border-error'}">
                        <div class="text-[10px] text-slate-500 uppercase">P-Value</div>
                        <div class="text-xl font-bold ${step.passed ? 'text-[#00FF88]' : 'text-error'}">${step.p_value.toFixed(4)}</div>
                    </div>
                </div>
            `;
        }
    }

    if (btnNistPrev) {
        btnNistPrev.addEventListener('click', () => { if (currentNistStepIdx > 0) { currentNistStepIdx--; renderNistStep(); } });
    }
    if (btnNistNext) {
        btnNistNext.addEventListener('click', () => { if (currentNistStepIdx < currentNistSteps.length - 1) { currentNistStepIdx++; renderNistStep(); } });
    }

    // Load NIST randomly once on startup
    setTimeout(loadNistSteps, 500);

});
