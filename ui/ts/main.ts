import { invoke } from "@tauri-apps/api/core";

const safeInvoke = async (command: string, args: any = {}) => {
    if ((window as any).__TAURI_INTERNALS__) {
        return await invoke(command, args);
    } else {
        console.warn(`[Tauri Mock] Called invoke('${command}') with args:`, args);
        throw new Error("Tauri API is not available in the browser.");
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

     // Learn sub-categories navigation (desktop sidebar only)
     const subTabLinks = document.querySelectorAll('.sub-tab-link');
      subTabLinks.forEach(link => {
          link.addEventListener('click', (e) => {
              e.preventDefault();
              // First switch to tab-learn
              switchTab('tab-learn');
              
              // Get the target learn section (learn-xor, learn-entropy, learn-nist)
              const target = (e.currentTarget as HTMLElement).getAttribute('data-target');
              if (target) {
                  // Extract the learn type (xor, entropy, nist) from data-target
                  const learnType = target.replace('learn-', '');
                  
                  // Activate the correct learn section
                  const learnSections = document.querySelectorAll('.learn-section');
                  learnSections.forEach(section => {
                      if (section.id === target) {
                          section.classList.add('active');
                      } else {
                          section.classList.remove('active');
                      }
                  });
                  
                  // Update learn buttons to show which is active
                  const learnBtns = document.querySelectorAll('.learn-btn');
                  learnBtns.forEach(btn => {
                      const btnTarget = btn.getAttribute('data-learn');
                      if (btnTarget === learnType) {
                          btn.classList.remove('bg-surface-container', 'text-slate-400');
                          btn.classList.add('active', 'bg-primary-container', 'text-on-primary-container');
                      } else {
                          btn.classList.remove('active', 'bg-primary-container', 'text-on-primary-container');
                          btn.classList.add('bg-surface-container', 'text-slate-400');
                      }
                  });
              }
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

                // Update Keystream Grid - calculate columns based on container width
                useKeystreamGrid.innerHTML = '';
                const bytes = result.keystream_bytes || [];
                const inputLength = text.length;
                
                // Calculate available width and columns dynamically
                const parentContainer = useKeystreamGrid.parentElement;
                if (parentContainer) {
                    const containerWidth = parentContainer.clientWidth;
                    const cellSize = 20; // 20px cells
                    const gap = 4; // 4px gap
                    const padding = 16; // 8px * 2 sides
                    
                    // Available space for cells
                    const availableWidth = containerWidth - padding;
                    // Each cell + gap, except last gap
                    const cellWithGap = cellSize + gap;
                    const maxColumns = Math.floor(availableWidth / cellWithGap);
                    
                    // Use calculated columns or minimum 1
                    const columns = Math.max(1, maxColumns);
                    
                    useKeystreamGrid.style.gridTemplateColumns = `repeat(${columns}, 20px)`;
                }
                
                for (let i = 0; i < inputLength; i++) {
                    const div = document.createElement('div');

                    const val = bytes[i] || 0;
                    // Grayscale: byte value (0-255) maps to RGB intensity (0-255)
                    const gray = Math.round((val / 255) * 255);
                    const rgbColor = `rgb(${gray}, ${gray}, ${gray})`;
                    div.style.backgroundColor = rgbColor;
                    
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
                document.getElementById('test-score-val')!.textContent = result.overall_score.toFixed(1);

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
    const benchSlider = document.getElementById('bench-size-slider') as HTMLInputElement;
    const benchLabel = document.getElementById('bench-size-label');
    const benchRunBtn = document.getElementById('bench-run-btn');
    const benchResultsSec = document.getElementById('bench-results-section');

    if (benchSlider && benchLabel) {
        benchSlider.addEventListener('input', () => {
            // Slider: 4-10 = 10^4 to 10^10, but we cap at 10^7 (10M)
            const val = parseFloat(benchSlider.value);
            const bytes = Math.round(Math.pow(10, val));
            
            // Format as human readable
            if (bytes >= 1_000_000) {
                benchLabel.textContent = `${(bytes / 1_000_000).toFixed(1)}M bytes`;
            } else if (bytes >= 1_000) {
                benchLabel.textContent = `${(bytes / 1_000).toFixed(0)}K bytes`;
            } else {
                benchLabel.textContent = `${bytes} bytes`;
            }
        });
        // Initialize
        benchSlider.dispatchEvent(new Event('input'));
    }

    if (benchRunBtn && benchSlider && benchResultsSec) {
        benchRunBtn.addEventListener('click', async () => {
            const val = parseFloat(benchSlider.value);
            const bytes = Math.round(Math.pow(10, val));

            // Show results section
            benchResultsSec.classList.remove('hidden');

            try {
                const result: any = await safeInvoke('run_benchmark', {
                    bytes: bytes
                });

                // Display results
                document.getElementById('bench-throughput')!.textContent = result.throughput_mbps.toFixed(2);
                document.getElementById('bench-latency')!.textContent = result.latency_us.toFixed(2);
                
                // Format details text
                const bytesFormatted = result.bytes_generated.toLocaleString();
                document.getElementById('bench-details')!.textContent = 
                    `Generated ${bytesFormatted} bytes in ${result.duration_secs.toFixed(3)} seconds`;
            } catch (error) {
                console.error(error);
                document.getElementById('bench-details')!.textContent = `Error: ${error}`;
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

         const prog = Math.round(((currentXorStepIdx + 1) / currentXorSteps.length) * 100);
         if (xorProgText) xorProgText.textContent = `${prog}%`;
         if (xorProgSlider) xorProgSlider.value = prog.toString();

         if (xorContent) {
             // Render HTML for XOR visualization with colored bits
             const ptBitsHtml = step.input_binary.split('').map((b: string) => `<span class="w-6 h-8 bg-surface-container-highest flex items-center justify-center font-bold text-cyan-400">${b}</span>`).join('');
             const keyBitsHtml = step.keystream_binary.split('').map((b: string) => `<span class="w-6 h-8 bg-surface-container-highest flex items-center justify-center font-bold text-orange-400">${b}</span>`).join('');
             const resBitsHtml = step.result_binary.split('').map((b: string) => `<span class="w-6 h-8 bg-primary-container/10 border border-green-400/30 flex items-center justify-center font-black text-green-400">${b}</span>`).join('');

             xorContent.innerHTML = `
             <div class="space-y-6">
                 <h3 class="text-lg font-bold text-primary">Step ${currentXorStepIdx + 1} of ${currentXorSteps.length}: Encrypting '${step.character}'</h3>
                 
                 <div class="bg-surface-container-low p-4 relative overflow-hidden border-l-2 border-[#00FF88] rounded">
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
                             <span class="text-[10px] text-cyan-400 text-left uppercase w-20 font-bold">Plaintext</span>
                             <div class="flex gap-1">${ptBitsHtml}</div>
                         </div>

                         <div class="flex items-center gap-4">
                             <div class="h-[1px] flex-grow bg-outline-variant"></div>
                             <span class="text-[#A855F7] font-black italic">⊕ XOR</span>
                             <div class="h-[1px] flex-grow bg-outline-variant"></div>
                         </div>

                         <div class="flex items-center justify-between">
                             <span class="text-[10px] text-orange-400 text-left uppercase w-20 font-bold">Keystream</span>
                             <div class="flex gap-1">${keyBitsHtml}</div>
                         </div>

                         <div class="flex items-center justify-between pt-4 border-t-2 border-green-400/30">
                             <span class="text-[10px] text-green-400 text-left uppercase w-20 font-bold">Result</span>
                             <div class="flex gap-1">${resBitsHtml}</div>
                         </div>
                     </div>
                 </div>
             </div>
             `;
         }

         // Update progress summary
         updateXorProgressSummary();
     }

     function updateXorProgressSummary() {
         const progressContainer = document.getElementById('learn-xor-progress');
         if (!progressContainer) return;

         if (currentXorSteps.length === 0) {
             progressContainer.classList.add('hidden');
             return;
         }

         progressContainer.classList.remove('hidden');

         const progressContent = progressContainer.querySelector('.flex.flex-wrap.gap-2');
         if (!progressContent) return;

         let html = '';
         currentXorSteps.forEach((step: any, idx: number) => {
             const resultByte = step.result_byte.toString(16).toUpperCase().padStart(2, '0');
             let classes = 'px-3 py-2 rounded font-mono text-sm font-bold transition-colors ';

             if (idx === currentXorStepIdx) {
                 classes += 'bg-green-500/30 border border-green-400 text-green-300';
             } else if (idx < currentXorStepIdx) {
                 classes += 'bg-surface-container-highest text-slate-400 border border-outline-variant/20';
             } else {
                 classes += 'bg-surface-container text-slate-500 border border-outline-variant/20';
             }

             html += `<span class="${classes}">${resultByte}</span>`;
         });

         progressContent.innerHTML = html;
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

     // XOR Collapsible "How it works" button
      const xorCollapsibleBtn = document.getElementById('learn-xor-collapsible');
      const xorDetailsDiv = document.getElementById('learn-xor-details');
      if (xorCollapsibleBtn && xorDetailsDiv) {
          xorCollapsibleBtn.addEventListener('click', () => {
              xorDetailsDiv.classList.toggle('hidden');
              const isExpanded = !xorDetailsDiv.classList.contains('hidden');
              xorCollapsibleBtn.setAttribute('aria-expanded', isExpanded ? 'true' : 'false');
              const icon = xorCollapsibleBtn.querySelector('.material-symbols-outlined') as HTMLElement;
              if (icon) {
                  icon.textContent = isExpanded ? 'expand_less' : 'expand_more';
              }
          });
      }

      // Shannon Entropy collapsible handler
      const entCollapsibleBtn = document.getElementById('learn-entropy-collapsible');
      const entDetailsDiv = document.getElementById('learn-entropy-details');
      if (entCollapsibleBtn && entDetailsDiv) {
          entCollapsibleBtn.addEventListener('click', () => {
              entDetailsDiv.classList.toggle('hidden');
              const isExpanded = !entDetailsDiv.classList.contains('hidden');
              entCollapsibleBtn.setAttribute('aria-expanded', isExpanded ? 'true' : 'false');
              const icon = entCollapsibleBtn.querySelector('.material-symbols-outlined') as HTMLElement;
              if (icon) {
                  icon.textContent = isExpanded ? 'expand_less' : 'expand_more';
              }
          });
      }

      // NIST Frequency Test collapsible handler
      const nistCollapsibleBtn = document.getElementById('learn-nist-collapsible');
      const nistDetailsDiv = document.getElementById('learn-nist-details');
      if (nistCollapsibleBtn && nistDetailsDiv) {
          nistCollapsibleBtn.addEventListener('click', () => {
              nistDetailsDiv.classList.toggle('hidden');
              const isExpanded = !nistDetailsDiv.classList.contains('hidden');
              nistCollapsibleBtn.setAttribute('aria-expanded', isExpanded ? 'true' : 'false');
              const icon = nistCollapsibleBtn.querySelector('.material-symbols-outlined') as HTMLElement;
              if (icon) {
                  icon.textContent = isExpanded ? 'expand_less' : 'expand_more';
              }
          });
      }

     // ==========================================
     // 7. Learn Entropy Logic
     // ==========================================
     let currentEntSteps: any[] = [];
     let currentEntStepIdx = 0;
     let entPlayInterval: any = null;
     const learnEntInput = document.getElementById('learn-entropy-input') as HTMLInputElement;
     const btnEntPrev = document.getElementById('learn-entropy-prev');
     const btnEntNext = document.getElementById('learn-entropy-next');
     const btnEntPlayPause = document.getElementById('learn-entropy-play-pause');
     const entStepInfo = document.getElementById('learn-entropy-step-info');
     const entContent = document.getElementById('learn-entropy-content');
     const btnEntCalculate = document.getElementById('learn-entropy-calculate');
     const entSpeedSlider = document.getElementById('learn-entropy-speed-slider') as HTMLInputElement;
     const entSpeedText = document.getElementById('learn-entropy-speed-text');

     async function loadEntSteps() {
         if (!learnEntInput.value) return;
         try {
             const result: any = await safeInvoke('get_entropy_steps', { text: learnEntInput.value });
             currentEntSteps = result.steps;
             currentEntStepIdx = 0;
             if (entPlayInterval) toggleEntPlayPause();
             renderEntStep();
         } catch (e) { console.error(e); }
     }

     function renderEntStep() {
         if (currentEntSteps.length === 0) return;
         const step = currentEntSteps[currentEntStepIdx];
         const stepType = step.step_type;

         if (entStepInfo) entStepInfo.textContent = `Step ${currentEntStepIdx + 1} / ${currentEntSteps.length}`;

         if (entContent) {
             // Build table rows
             let tableHtml = `<table class="w-full text-sm">
                 <thead>
                     <tr class="border-b border-outline-variant/30 text-left">
                         <th class="p-2 font-bold text-primary">Byte</th>
                         <th class="p-2 font-bold text-primary">Count</th>`;

             if (stepType !== 'CountBytes') {
                 tableHtml += `<th class="p-2 font-bold text-primary">Probability</th>`;
             }
             if (stepType === 'CalculateContributions' || stepType === 'SumEntropy' || stepType === 'Interpret') {
                 tableHtml += `<th class="p-2 font-bold text-primary">Contribution</th>`;
             }
             tableHtml += `<th class="p-2 font-bold text-primary">Visual</th>
                     </tr>
                 </thead>
                 <tbody>`;

             // Sort bytes and render rows
             const sortedBytes = Object.keys(step.byte_counts).sort();
             const maxCount = Math.max(...Object.values(step.byte_counts as Record<string, number>), 1);

             for (const byteStr of sortedBytes) {
                 const byteNum = parseInt(byteStr);
                 const count = (step.byte_counts as Record<string, number>)[byteStr];
                 const prob = (step.probabilities as Record<string, number>)?.[byteStr] || 0;
                 const contrib = (step.entropy_contributions as Record<string, number>)?.[byteStr] || 0;
                 const charRepr = byteNum >= 32 && byteNum <= 126 ? String.fromCharCode(byteNum) : `0x${byteNum.toString(16).toUpperCase().padStart(2, '0')}`;
                 const barWidth = (count / maxCount) * 100;
                 const barColor = count === 1 ? '#00FF88' : `rgb(255, ${100 + (155 - Math.min(count * 20, 155))}, 100)`;

                 tableHtml += `<tr class="border-b border-outline-variant/20 hover:bg-surface-container-low transition-colors">
                     <td class="p-2 font-mono text-slate-300">'${charRepr}'</td>
                     <td class="p-2 font-mono text-slate-300">${count}</td>`;

                 if (stepType !== 'CountBytes') {
                     tableHtml += `<td class="p-2 font-mono text-slate-300">${prob.toFixed(3)}</td>`;
                 }
                 if (stepType === 'CalculateContributions' || stepType === 'SumEntropy' || stepType === 'Interpret') {
                     tableHtml += `<td class="p-2 font-mono text-slate-300">${contrib.toFixed(3)} bits</td>`;
                 }
                 tableHtml += `<td class="p-2"><div class="h-6 bg-surface-container rounded" style="width: ${barWidth}%; background-color: ${barColor};"></div></td>
                     </tr>`;
             }

             tableHtml += `</tbody></table>`;

             // Build explanation text
             let explanationHtml = '';
             switch (stepType) {
                 case 'CountBytes':
                     explanationHtml = `<div class="mt-6 p-4 bg-surface-container-low rounded border-l-2 border-primary">
                         <p class="text-sm text-slate-300">
                             <strong>💡 What's happening:</strong> We count how many times each unique byte appears in the input. This frequency distribution is the foundation for entropy calculation.
                         </p>
                     </div>`;
                     break;

                 case 'CalculateProbabilities':
                     explanationHtml = `<div class="mt-6 space-y-3">
                         <div class="p-4 bg-surface-container-low rounded border-l-2 border-primary">
                             <p class="text-sm text-slate-300 mb-2">
                                 <strong>💡 Formula:</strong> P(x) = Count(x) ÷ Total Bytes
                             </p>
                             <p class="text-sm text-slate-300">
                                 This tells us the likelihood of each byte appearing in the input. Higher probability = more common byte.
                             </p>
                         </div>
                         <button class="w-full p-2 bg-surface-container rounded text-xs text-slate-400 hover:bg-surface-container-highest transition-colors text-left">
                             ▶ Learn More: Why does this matter?
                         </button>
                     </div>`;
                     break;

                 case 'CalculateContributions':
                     explanationHtml = `<div class="mt-6 space-y-3">
                         <div class="p-4 bg-surface-container-low rounded border-l-2 border-primary">
                             <p class="text-sm text-slate-300 mb-2">
                                 <strong>💡 Formula:</strong> Contribution = -P(x) × log₂(P(x))
                             </p>
                             <p class="text-sm text-slate-300">
                                 Rare events provide MORE information when they occur. A common byte (50%) is less surprising than a rare one (5%).
                             </p>
                         </div>
                         <button class="w-full p-2 bg-surface-container rounded text-xs text-slate-400 hover:bg-surface-container-highest transition-colors text-left">
                             ▶ Learn More: Information Theory Background
                         </button>
                     </div>`;
                     break;

                  case 'SumEntropy':
                  case 'Interpret': {
                      const efficiency = step.max_entropy > 0 ? (step.total_entropy / step.max_entropy) * 100 : 0;
                      let interpretation = 'Moderate entropy.';
                      let interpretIcon = '⚠️';

                      if (step.current_entropy_sum < 2.0) {
                          interpretation = 'Low entropy. The input is very predictable.';
                          interpretIcon = '📉';
                      } else if (efficiency > 80) {
                          interpretation = 'High entropy! The input looks quite random or uses many different characters.';
                          interpretIcon = '🔥';
                      }

                      explanationHtml = `<div class="mt-6 space-y-4">
                          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                              <div class="bg-surface-container-low p-4 rounded border border-primary/30">
                                  <div class="text-xs text-slate-400 uppercase mb-1">Total Entropy</div>
                                  <div class="text-2xl font-bold text-primary">${step.current_entropy_sum.toFixed(4)}</div>
                                  <div class="text-xs text-slate-500 mt-1">bits/byte</div>
                              </div>
                              <div class="bg-surface-container-low p-4 rounded border border-primary/30">
                                  <div class="text-xs text-slate-400 uppercase mb-1">Maximum Possible</div>
                                  <div class="text-2xl font-bold text-primary">${step.max_entropy.toFixed(4)}</div>
                                  <div class="text-xs text-slate-500 mt-1">bits/byte (${sortedBytes.length} unique)</div>
                              </div>
                              <div class="bg-surface-container-low p-4 rounded border border-[#A855F7]/30">
                                  <div class="text-xs text-slate-400 uppercase mb-1">Efficiency</div>
                                  <div class="text-2xl font-bold text-[#A855F7]">${efficiency.toFixed(1)}%</div>
                                  <div class="text-xs text-slate-500 mt-1">how close to max</div>
                              </div>
                          </div>

                          <div class="p-4 bg-surface-container-low rounded border-l-2 border-[#A855F7]">
                              <p class="text-sm text-slate-300 mb-2">
                                  <strong>${interpretIcon} Interpretation:</strong>
                              </p>
                              <p class="text-sm text-slate-300">
                                  ${interpretation}
                              </p>
                              ${step.current_entropy_sum < 2.0 ? `<p class="text-sm text-slate-400 mt-2 italic">For cryptography, you generally want entropy > 7 bits/byte for strong randomness.</p>` : ''}
                          </div>
                      </div>`;
                      break;
                  }
             }

             entContent.innerHTML = tableHtml + explanationHtml;
         }
     }

     function toggleEntPlayPause() {
         if (!btnEntPlayPause) return;
         const icon = btnEntPlayPause.querySelector('.material-symbols-outlined') as HTMLElement;
         const label = btnEntPlayPause.querySelector('span:last-child') as HTMLElement;

         if (entPlayInterval) {
             clearInterval(entPlayInterval);
             entPlayInterval = null;
             if (icon) icon.textContent = 'play_arrow';
             if (label) label.textContent = 'Play';
         } else {
             if (currentEntStepIdx >= currentEntSteps.length - 1) {
                 currentEntStepIdx = 0;
             }
             const speedMs = parseInt(entSpeedSlider.value, 10);
             entPlayInterval = setInterval(() => {
                 if (btnEntNext) btnEntNext.click();
             }, speedMs);
             if (icon) icon.textContent = 'pause';
             if (label) label.textContent = 'Pause';
         }
     }

     if (btnEntCalculate) {
         btnEntCalculate.addEventListener('click', loadEntSteps);
     }
     if (btnEntPrev) {
         btnEntPrev.addEventListener('click', () => { if (currentEntStepIdx > 0) { currentEntStepIdx--; renderEntStep(); } });
     }
     if (btnEntNext) {
         btnEntNext.addEventListener('click', () => {
             if (currentEntStepIdx < currentEntSteps.length - 1) {
                 currentEntStepIdx++;
                 renderEntStep();
             } else if (entPlayInterval) {
                 toggleEntPlayPause();
             }
         });
     }
     if (btnEntPlayPause) {
         btnEntPlayPause.addEventListener('click', toggleEntPlayPause);
     }
     if (entSpeedSlider && entSpeedText) {
         entSpeedSlider.addEventListener('input', () => {
             const speedSec = (parseInt(entSpeedSlider.value, 10) / 1000).toFixed(1);
             entSpeedText.textContent = `${speedSec}s / step`;
             if (entPlayInterval) {
                 toggleEntPlayPause();
                 toggleEntPlayPause();
             }
         });
     }

     // ==========================================
     // 8. Learn NIST Logic
     // ==========================================
     let currentNistSteps: any[] = [];
     let currentNistStepIdx = 0;
     let nistPlayInterval: any = null;
     const nistInput = document.getElementById('learn-nist-input') as HTMLInputElement;
     const btnNistAnalyze = document.getElementById('learn-nist-analyze');
     const btnNistRandom = document.getElementById('learn-nist-random');
     const btnNistPrev = document.getElementById('learn-nist-prev');
     const btnNistNext = document.getElementById('learn-nist-next');
     const btnNistPlayPause = document.getElementById('learn-nist-play-pause');
     const nistStepInfo = document.getElementById('learn-nist-step-info');
     const nistContent = document.getElementById('learn-nist-content');
     const nistSpeedSlider = document.getElementById('learn-nist-speed-slider') as HTMLInputElement;
     const nistSpeedText = document.getElementById('learn-nist-speed-text');

     async function loadNistSteps(isRandom: boolean = true, text: string = '') {
         try {
             let result: any;
             if (isRandom) {
                 result = await safeInvoke('get_nist_steps_random', { count: 64 });
             } else {
                 result = await safeInvoke('get_nist_steps', { text });
             }
             currentNistSteps = result.steps;
             currentNistStepIdx = 0;
             if (nistPlayInterval) toggleNistPlayPause();
             renderNistStep();
         } catch (e) { console.error(e); }
     }

     function renderNistStep() {
         if (currentNistSteps.length === 0) return;
         const step = currentNistSteps[currentNistStepIdx];
         const stepType = step.step_type;

         if (nistStepInfo) nistStepInfo.textContent = `Step ${currentNistStepIdx + 1} / ${currentNistSteps.length}`;

         if (nistContent) {
             // Bit sequence with grouping by 8
             let bitsHtml = '';
             for (let i = 0; i < step.bits.length; i++) {
                 if (i > 0 && i % 8 === 0) {
                     bitsHtml += '<span class="mx-2"></span>';
                 }
                 const bit = step.bits[i];
                 const color = bit === 1 ? 'text-[#00FF88]' : 'text-slate-400';
                 bitsHtml += `<span class="inline-block w-4 text-center font-bold ${color}">${bit}</span>`;
             }

             let content = `<div class="space-y-6">
                 <h3 class="text-lg font-bold text-[#A855F7]">Step ${currentNistStepIdx + 1} of ${currentNistSteps.length}: ${stepType.replace(/([A-Z])/g, ' $1').trim()}</h3>

                 <div>
                     <h4 class="text-sm font-bold text-slate-300 mb-2">Bit Sequence (${step.bits.length} bits):</h4>
                     <div class="bg-surface-container-lowest p-4 rounded border border-outline-variant/20 font-mono text-sm break-all">
                         ${bitsHtml}
                     </div>
                 </div>`;

             const total = step.bits.length;
             const onesPct = total > 0 ? (step.ones_count / total) * 100 : 0;

             if (stepType !== 'ConvertToBits') {
                 // Ones/Zeros counter boxes
                 content += `<div class="grid grid-cols-2 gap-4">
                     <div class="bg-surface-container-low p-4 rounded border border-outline-variant/20">
                         <div class="text-xs text-slate-400 uppercase font-bold mb-1">Ones Count</div>
                         <div class="text-3xl font-bold text-[#00FF88]">${step.ones_count}</div>
                     </div>
                     <div class="bg-surface-container-low p-4 rounded border border-outline-variant/20">
                         <div class="text-xs text-slate-400 uppercase font-bold mb-1">Zeros Count</div>
                         <div class="text-3xl font-bold text-slate-400">${step.zeros_count}</div>
                     </div>
                 </div>

                 <!-- Balance Visualization -->
                 <div>
                     <h4 class="text-sm font-bold text-slate-300 mb-2">Distribution Balance (Ideal: 50% Ones)</h4>
                     <div class="relative h-12 bg-slate-600 rounded overflow-hidden">
                         <div class="absolute h-full bg-[#00FF88] transition-all" style="width: ${onesPct}%;"></div>
                         <div class="absolute top-1/2 -translate-y-1/2 left-1/2 -translate-x-1/2 w-0.5 h-full bg-black"></div>
                         <div class="absolute inset-0 flex items-center justify-center font-bold text-sm text-white mix-blend-multiply">
                             ${onesPct.toFixed(1)}% Ones (${step.ones_count}/${total} bits)
                         </div>
                     </div>
                 </div>`;
             }

             // Explanation based on step type
             let explanationHtml = '';
             switch (stepType) {
                 case 'ConvertToBits':
                     explanationHtml = `<div class="p-4 bg-surface-container-low rounded border-l-2 border-[#A855F7]">
                         <p class="text-sm text-slate-300">
                             <strong>💡 What's happening:</strong> Each byte becomes 8 binary digits. White = 0, Green = 1.
                         </p>
                     </div>`;
                     break;

                 case 'CountOnesZeros':
                     explanationHtml = `<div class="p-4 bg-surface-container-low rounded border-l-2 border-[#A855F7]">
                         <p class="text-sm text-slate-300 mb-2">
                             <strong>💡 What's happening:</strong> If the data is truly random, we expect roughly 50% ones and 50% zeros, like fair coin flips.
                         </p>
                         <p class="text-sm text-slate-400 italic">
                             Current ratio: ${onesPct.toFixed(1)}% ones vs ${(100 - onesPct).toFixed(1)}% zeros
                         </p>
                     </div>`;
                     break;

                 case 'CalculateStatistic':
                     explanationHtml = `<div class="space-y-3">
                         <div class="bg-surface-container-low p-4 rounded border-l-2 border-[#A855F7]">
                             <p class="text-sm text-slate-300 mb-2">
                                 <strong>💡 Formula:</strong> S_obs = |Sum| ÷ √(n)
                             </p>
                             <p class="text-sm text-slate-400">Where Sum = (count_ones × +1) + (count_zeros × -1)</p>
                             <p class="text-sm text-slate-400">Result: ${step.sum} ÷ √${total} = ${step.s_obs.toFixed(4)}</p>
                         </div>
                         <div class="p-3 bg-surface-container rounded text-xs text-slate-400">
                             <p>S_obs tells us how far from balanced (0) the sequence is. Closer to 0 = more balanced and random-looking.</p>
                         </div>
                     </div>`;
                     break;

                 case 'CalculatePValue':
                     explanationHtml = `<div class="space-y-3">
                         <div class="bg-surface-container-low p-4 rounded border-l-2 border-[#A855F7]">
                             <p class="text-sm text-slate-300 mb-2">
                                 <strong>💡 Formula:</strong> P-value = erfc(S_obs ÷ √2)
                             </p>
                             <p class="text-sm text-slate-400">Result: erfc(${step.s_obs.toFixed(4)} ÷ 1.414) = ${step.p_value.toFixed(4)}</p>
                         </div>
                         <div class="p-3 bg-surface-container rounded text-xs text-slate-400">
                             <p><strong>Interpretation:</strong> P-value = ${(step.p_value * 100).toFixed(1)}% probability that a truly random sequence would show this much deviation.</p>
                         </div>
                     </div>`;
                     break;

                  case 'Interpret': {
                      const statusColor = step.passed ? '#00FF88' : '#ff6b6b';
                      const statusIcon = step.passed ? '✅' : '❌';
                      const statusText = step.passed ? 'PASS' : 'FAIL';
                      const statusMsg = step.passed ? 'The sequence looks RANDOM!' : 'The sequence is BIASED toward 1s or 0s.';

                      explanationHtml = `<div class="space-y-4">
                          <div class="p-4 rounded border-l-4" style="border-color: ${statusColor}; background: rgba(0,0,0,0.2);">
                              <div class="flex items-center gap-2 mb-2">
                                  <span style="color: ${statusColor}" class="text-2xl font-bold">${statusIcon} ${statusText}</span>
                              </div>
                              <p class="text-lg text-slate-300 font-bold mb-2">${statusMsg}</p>
                          </div>

                          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                              <div class="bg-surface-container-low p-4 rounded">
                                  <div class="text-xs text-slate-400 uppercase mb-1">Test Statistic (S_obs)</div>
                                  <div class="text-2xl font-bold text-slate-300">${step.s_obs.toFixed(4)}</div>
                              </div>
                              <div class="bg-surface-container-low p-4 rounded">
                                  <div class="text-xs text-slate-400 uppercase mb-1">P-Value</div>
                                  <div class="text-2xl font-bold" style="color: ${statusColor};">${step.p_value.toFixed(4)}</div>
                                  <div class="text-xs text-slate-500 mt-1">vs Threshold: 0.01</div>
                              </div>
                          </div>

                          <div class="p-4 bg-surface-container-low rounded border border-outline-variant/20 text-sm text-slate-300">
                              <p><strong>Why this matters:</strong> P-value ≥ 0.01 indicates the data passes randomness test. Lower values suggest bias.</p>
                              <p class="mt-2 text-xs text-slate-400">For cryptography, randomness is critical. Biased sequences can be predicted and exploited.</p>
                          </div>
                      </div>`;
                      break;
                  }
             }

             content += explanationHtml + '</div>';
             nistContent.innerHTML = content;
         }
     }

     function toggleNistPlayPause() {
         if (!btnNistPlayPause) return;
         const icon = btnNistPlayPause.querySelector('.material-symbols-outlined') as HTMLElement;
         const label = btnNistPlayPause.querySelector('span:last-child') as HTMLElement;

         if (nistPlayInterval) {
             clearInterval(nistPlayInterval);
             nistPlayInterval = null;
             if (icon) icon.textContent = 'play_arrow';
             if (label) label.textContent = 'Play';
         } else {
             if (currentNistStepIdx >= currentNistSteps.length - 1) {
                 currentNistStepIdx = 0;
             }
             const speedMs = parseInt(nistSpeedSlider.value, 10);
             nistPlayInterval = setInterval(() => {
                 if (btnNistNext) btnNistNext.click();
             }, speedMs);
             if (icon) icon.textContent = 'pause';
             if (label) label.textContent = 'Pause';
         }
     }

     if (btnNistAnalyze) {
         btnNistAnalyze.addEventListener('click', () => {
             if (nistInput.value) {
                 loadNistSteps(false, nistInput.value);
             }
         });
     }

     if (btnNistRandom) {
         btnNistRandom.addEventListener('click', () => {
             nistInput.value = '';
             loadNistSteps(true);
         });
     }

     if (btnNistPrev) {
         btnNistPrev.addEventListener('click', () => { if (currentNistStepIdx > 0) { currentNistStepIdx--; renderNistStep(); } });
     }

     if (btnNistNext) {
         btnNistNext.addEventListener('click', () => {
             if (currentNistStepIdx < currentNistSteps.length - 1) {
                 currentNistStepIdx++;
                 renderNistStep();
             } else if (nistPlayInterval) {
                 toggleNistPlayPause();
             }
         });
     }

     if (btnNistPlayPause) {
         btnNistPlayPause.addEventListener('click', toggleNistPlayPause);
     }

     if (nistSpeedSlider && nistSpeedText) {
         nistSpeedSlider.addEventListener('input', () => {
             const speedSec = (parseInt(nistSpeedSlider.value, 10) / 1000).toFixed(1);
             nistSpeedText.textContent = `${speedSec}s / step`;
             if (nistPlayInterval) {
                 toggleNistPlayPause();
                 toggleNistPlayPause();
             }
         });
     }

      // Load NIST randomly once on startup
      setTimeout(() => loadNistSteps(true), 500);

      // ==========================================
      // KEYBOARD NAVIGATION
      // ==========================================

      // Enter key on input fields to trigger actions
      useInput?.addEventListener('keydown', (e) => {
          if (e.key === 'Enter' && !e.shiftKey) {
              // Shift+Enter allows multiline input, but Enter alone triggers encrypt
              e.preventDefault();
              useEncryptBtn?.click();
          }
      });

      learnInput?.addEventListener('keydown', (e: KeyboardEvent) => {
          if (e.key === 'Enter') {
              e.preventDefault();
              // Auto-generate next steps or navigate
              if (btnXorNext) {
                  btnXorNext.click();
              }
          }
      });

      learnEntInput?.addEventListener('keydown', (e: KeyboardEvent) => {
          if (e.key === 'Enter') {
              e.preventDefault();
              btnEntCalculate?.click();
          }
      });

      nistInput?.addEventListener('keydown', (e: KeyboardEvent) => {
          if (e.key === 'Enter') {
              e.preventDefault();
              btnNistAnalyze?.click();
          }
      });

       // Escape key to close collapsibles
       document.addEventListener('keydown', (e) => {
           if (e.key === 'Escape') {
               const xorDetails = document.getElementById('learn-xor-details');
               const entDetails = document.getElementById('learn-entropy-details');
               const nistDetails = document.getElementById('learn-nist-details');

               if (xorDetails && !xorDetails.classList.contains('hidden')) {
                   xorDetails.classList.add('hidden');
                   xorCollapsibleBtn?.setAttribute('aria-expanded', 'false');
                   const icon = xorCollapsibleBtn?.querySelector('.material-symbols-outlined');
                   if (icon) icon.textContent = 'expand_more';
               }

               if (entDetails && !entDetails.classList.contains('hidden')) {
                   entDetails.classList.add('hidden');
                   entCollapsibleBtn?.setAttribute('aria-expanded', 'false');
                   const icon = entCollapsibleBtn?.querySelector('.material-symbols-outlined');
                   if (icon) icon.textContent = 'expand_more';
               }

               if (nistDetails && !nistDetails.classList.contains('hidden')) {
                   nistDetails.classList.add('hidden');
                   nistCollapsibleBtn?.setAttribute('aria-expanded', 'false');
                   const icon = nistCollapsibleBtn?.querySelector('.material-symbols-outlined');
                   if (icon) icon.textContent = 'expand_more';
               }
           }
       });

      // Arrow key support for next/previous step navigation in Learn tab
      document.addEventListener('keydown', (e) => {
          const activeTab = document.querySelector('.tab-panel.active');
          if (!activeTab || activeTab.id !== 'tab-learn') return;

          if (e.key === 'ArrowRight' && (e.ctrlKey || e.metaKey)) {
              e.preventDefault();
              // Move to next step in active learn section
              const activeSection = activeTab.querySelector('.learn-section.active');
              if (activeSection?.id === 'learn-xor') btnXorNext?.click();
              else if (activeSection?.id === 'learn-entropy') btnEntNext?.click();
              else if (activeSection?.id === 'learn-nist') btnNistNext?.click();
          }

          if (e.key === 'ArrowLeft' && (e.ctrlKey || e.metaKey)) {
              e.preventDefault();
              // Move to previous step in active learn section
              const activeSection = activeTab.querySelector('.learn-section.active');
              if (activeSection?.id === 'learn-xor') btnXorPrev?.click();
              else if (activeSection?.id === 'learn-entropy') btnEntPrev?.click();
              else if (activeSection?.id === 'learn-nist') btnNistPrev?.click();
          }

          // Space bar to play/pause
          if (e.key === ' ' && e.ctrlKey) {
              e.preventDefault();
              const activeSection = activeTab.querySelector('.learn-section.active');
              if (activeSection?.id === 'learn-xor') btnXorPlayPause?.click();
              else if (activeSection?.id === 'learn-entropy') btnEntPlayPause?.click();
              else if (activeSection?.id === 'learn-nist') btnNistPlayPause?.click();
          }
      });

      // Keyboard shortcuts for tab switching (Ctrl+1, Ctrl+2, etc.)
      document.addEventListener('keydown', (e) => {
          if (e.ctrlKey && !e.shiftKey && !e.altKey && !e.metaKey) {
              const keyNum = parseInt(e.key, 10);
              if (keyNum >= 1 && keyNum <= 4) {
                  e.preventDefault();
                  const tabMap: Record<number, string> = {
                      1: 'tab-use',
                      2: 'tab-test',
                      3: 'tab-bench',
                      4: 'tab-learn'
                  };
                  const tabId = tabMap[keyNum];
                  if (tabId) switchTab(tabId);
              }
          }
      });
 
});
