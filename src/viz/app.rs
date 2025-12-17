//! Main GUI application

use eframe::egui;
use crate::entropy::{EntropySource, SystemEntropy};
use crate::crypto::StreamCipher;
use crate::quality::{QualityMetrics, NistTests};
use crate::bench::{PerformanceBench, BenchmarkResult};
use crate::learn::{EncryptionProcess, EntropyProcess, NistProcess};

/// Main application state
pub struct EntropyForgeApp {
    // Entropy source
    entropy: Box<dyn EntropySource>,
    
    // Current tab
    current_tab: Tab,
    
    // Use tab state
    cipher_input: String,
    cipher_output: String,
    cipher_hex: bool,
    cipher_state: Vec<u8>,
    
    // Test tab state
    quality_metrics: Option<QualityMetrics>,
    nist_results: Vec<(String, f64)>,
    quality_sample_size: usize,
    is_testing: bool,
    
    // Benchmark tab state
    bench_result: Option<BenchmarkResult>,
    bench_size: usize,
    is_benchmarking: bool,

    // Learn tab state
    learn_mode: LearnMode,
    learn_process: EncryptionProcess,
    learn_input: String,

    entropy_process: EntropyProcess,
    nist_process: NistProcess,
}

#[derive(PartialEq, Clone, Copy)]
enum LearnMode {
    XorCipher,
    ShannonEntropy,
    NistFrequency,
}

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Use,
    Test,
    Benchmark,
    Learn,
}

impl Default for EntropyForgeApp {
    fn default() -> Self {
        Self {
            entropy: Box::new(SystemEntropy::new()),
            current_tab: Tab::Use,
            cipher_input: String::from("Hello, World!"),
            cipher_output: String::new(),
            cipher_hex: true,
            cipher_state: Vec::new(),
            quality_metrics: None,
            nist_results: Vec::new(),
            quality_sample_size: 100_000,
            is_testing: false,
            bench_result: None,
            bench_size: 1_000_000,
            is_benchmarking: false,
            learn_mode: LearnMode::XorCipher,
            learn_process: EncryptionProcess::new(),
            learn_input: String::from("Hello"),
            entropy_process: EntropyProcess::new(),
            nist_process: NistProcess::new(),
        }
    }
}

impl eframe::App for EntropyForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.heading("🔐 Entropy Forge");
            ui.label(format!("Source: {}", self.entropy.name()));
            ui.add_space(5.0);
            ui.separator();
            
            // Tabs
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Use, "📝 Use");
                ui.selectable_value(&mut self.current_tab, Tab::Test, "🔬 Test");
                ui.selectable_value(&mut self.current_tab, Tab::Benchmark, "⚡ Benchmark");
                ui.selectable_value(&mut self.current_tab, Tab::Learn, "🎓 Learn");
            });
            
            ui.separator();
            ui.add_space(10.0);
            
            // Tab content
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.current_tab {
                    Tab::Use => self.render_use_tab(ui),
                    Tab::Test => self.render_test_tab(ui),
                    Tab::Benchmark => self.render_benchmark_tab(ui),
                    Tab::Learn => self.render_learn_tab(ui),
                }
            });
        });
    }
}

impl EntropyForgeApp {
    /// Helper to render consistent educational tooltips
    fn render_explanation_tooltip(ui: &mut egui::Ui, label: &str, text: &str) {
        ui.horizontal(|ui| {
            ui.label(label);
            ui.label("ℹ").on_hover_text(text);
        });
    }

    /// Render the "Use" tab (stream cipher)
    fn render_use_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Stream Cipher");
        ui.label("Encrypt or decrypt data using the entropy source as keystream.");
        ui.add_space(10.0);
        
        // Input
        ui.horizontal(|ui| {
            ui.label("Input:");
            ui.text_edit_singleline(&mut self.cipher_input);
            if ui.button("Help: How does this work?").clicked() {
                self.current_tab = Tab::Learn;
                self.learn_input = self.cipher_input.clone();
                self.learn_process.start(&self.learn_input);
            }
        });
        
        ui.checkbox(&mut self.cipher_hex, "Display output as hex");
        
        ui.add_space(10.0);
        
        // Encrypt button
        if ui.button("🔒 Encrypt / Decrypt").clicked() {
            // Create a temporary entropy source for the cipher
            let temp_entropy = SystemEntropy::new();
            let mut cipher = StreamCipher::new(temp_entropy);
            let output = cipher.process(self.cipher_input.as_bytes());

            self.cipher_output = if self.cipher_hex {
                hex::encode(&output)
            } else {
                String::from_utf8_lossy(&output).to_string()
            };

            self.cipher_state = cipher.state().to_vec();
        }
        
        ui.add_space(10.0);
        
        // Output
        ui.label("Output:");
        ui.add(
            egui::TextEdit::multiline(&mut self.cipher_output.as_str())
                .desired_width(f32::INFINITY)
                .desired_rows(3)
        );
        
        ui.add_space(20.0);
        
        // Cipher state visualization
        if !self.cipher_state.is_empty() {
            ui.heading("Cipher State (first 64 bytes of keystream)");
            ui.add_space(5.0);
            
            // Display as 8x8 grid
            egui::Grid::new("cipher_state_grid")
                .spacing([4.0, 4.0])
                .show(ui, |ui| {
                    for (i, &byte) in self.cipher_state.iter().take(64).enumerate() {
                        if i % 8 == 0 && i > 0 {
                            ui.end_row();
                        }
                        
                        let color = egui::Color32::from_rgb(byte, byte, byte);
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(30.0, 30.0),
                            egui::Sense::hover()
                        );
                        ui.painter().rect_filled(rect, 2.0, color);

                        // Tooltip
                        response.on_hover_text(format!("Byte {}: 0x{:02X} ({})", i, byte, byte));
                    }
                });
        }
    }
    
    /// Render the "Test" tab (quality metrics)
    fn render_test_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Quality Analysis");
        ui.label("Analyze entropy quality with statistical tests.");
        ui.add_space(10.0);
        
        // Sample size selector
        ui.horizontal(|ui| {
            ui.label("Sample size:");
            ui.add(egui::Slider::new(&mut self.quality_sample_size, 1_000..=1_000_000)
                .logarithmic(true)
                .suffix(" bytes"));
        });
        
        ui.add_space(10.0);
        
        // Run tests button
        ui.horizontal(|ui| {
            if ui.button("🔬 Run All Tests").clicked() && !self.is_testing {
                self.is_testing = true;
                
                // Run quality metrics
                self.quality_metrics = Some(
                    QualityMetrics::analyze(&mut *self.entropy, self.quality_sample_size)
                );
                
                // Run NIST tests
                let mut data = vec![0u8; self.quality_sample_size];
                self.entropy.fill_bytes(&mut data);
                
                self.nist_results = NistTests::run_all_tests(&data)
                    .into_iter()
                    .map(|(name, p_value)| (name.to_string(), p_value))
                    .collect();
                
                self.is_testing = false;
            }
            
            if self.is_testing {
                ui.spinner();
                ui.label("Testing...");
            }
        });
        
        ui.add_space(20.0);
        
        // Display results
        if let Some(ref metrics) = self.quality_metrics {
            ui.heading("Entropy Metrics");
            ui.add_space(5.0);
            
            // Shannon entropy
            Self::render_explanation_tooltip(
                ui,
                "Shannon Entropy:",
                "Measures the unpredictability of data. \n8.0 bits/byte is perfect randomness.\nLower values mean patterns exist."
            );
            ui.label(format!("{:.4} bits/byte", metrics.shannon_entropy));

            let shannon_progress = (metrics.shannon_entropy / 8.0) as f32;
            ui.add(egui::ProgressBar::new(shannon_progress)
                .text(format!("{:.1}%", shannon_progress * 100.0)));
            
            ui.add_space(5.0);
            
            // Min-entropy
            Self::render_explanation_tooltip(
                ui,
                "Min-Entropy:",
                "The worst-case entropy. \nCrucial for security guarantees.\nIf this is low, an attacker might guess the key."
            );
            ui.label(format!("{:.4} bits/byte", metrics.min_entropy));
            let min_progress = (metrics.min_entropy / 8.0) as f32;
            ui.add(egui::ProgressBar::new(min_progress)
                .text(format!("{:.1}%", min_progress * 100.0)));
            
            ui.add_space(10.0);
            
            // Other metrics
            ui.label(format!("Mean byte value: {:.2} (ideal: 127.5)", metrics.mean));
            ui.label(format!("Chi-square: {:.2}", metrics.chi_square));
            ui.label(format!("Longest run: {} bits", metrics.longest_run));
            
            ui.add_space(10.0);
            
            // Overall score
            let score = metrics.overall_score();
            ui.horizontal(|ui| {
                ui.label("Overall Quality Score:");
                ui.label(format!("{:.1}/100", score));
            });
            
            ui.add_space(20.0);
            
            // NIST tests
            ui.horizontal(|ui| {
                ui.heading("NIST SP 800-22 Tests");
                ui.label("ℹ").on_hover_text(
                    "Standard statistical tests used by the US government to certify random number generators.\nWe check if the data looks indistinguishable from a coin flip."
                );
            });
            ui.label("P-values ≥ 0.01 indicate randomness (Pass)");
            ui.add_space(5.0);
            
            egui::Grid::new("nist_results")
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Test");
                    ui.label("P-Value");
                    ui.label("Result");
                    ui.end_row();
                    
                    for (name, p_value) in &self.nist_results {
                        ui.label(name);
                        ui.label(format!("{:.4}", p_value));
                        
                        if *p_value >= 0.01 {
                            ui.colored_label(egui::Color32::GREEN, "✓ Pass");
                        } else {
                            ui.colored_label(egui::Color32::RED, "✗ Fail");
                        }
                        ui.end_row();
                    }
                });
        } else {
            ui.label("Click 'Run All Tests' to analyze entropy quality.");
        }
    }
    
    /// Render the "Benchmark" tab
    fn render_benchmark_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Performance Benchmark");
        ui.label("Measure entropy generation speed.");
        ui.add_space(10.0);
        
        // Benchmark size selector
        ui.horizontal(|ui| {
            ui.label("Benchmark size:");
            ui.add(egui::Slider::new(&mut self.bench_size, 10_000..=10_000_000)
                .logarithmic(true)
                .suffix(" bytes"));
        });
        
        ui.add_space(10.0);
        
        // Run benchmark button
        ui.horizontal(|ui| {
            if ui.button("⚡ Run Benchmark").clicked() && !self.is_benchmarking {
                self.is_benchmarking = true;
                self.bench_result = Some(
                    PerformanceBench::benchmark(&mut *self.entropy, self.bench_size)
                );
                self.is_benchmarking = false;
            }
            
            if self.is_benchmarking {
                ui.spinner();
                ui.label("Benchmarking...");
            }
        });
        
        ui.add_space(20.0);
        
        // Display results
        if let Some(ref result) = self.bench_result {
            ui.heading("Results");
            ui.add_space(10.0);
            
            // Throughput
            ui.horizontal(|ui| {
                Self::render_explanation_tooltip(
                    ui,
                    "Throughput:",
                    "How fast we can generate entropy.\nImportant for bulk encryption (e.g., file encryption, video streaming)."
                );
                ui.heading(format!("{:.2} MB/s", result.throughput_mbps));
            });
            
            ui.add_space(5.0);
            
            // Latency
            ui.horizontal(|ui| {
                Self::render_explanation_tooltip(
                    ui,
                    "Latency per byte:",
                    "Time to generate a single byte.\nImportant for real-time applications (e.g., key generation, handshakes)."
                );
                ui.label(format!("{:.2} µs", result.latency_us));
            });
            
            ui.add_space(5.0);
            
            // Details
            ui.label(format!("Generated {} bytes in {:.3} seconds",
                result.bytes_generated,
                result.duration.as_secs_f64()
            ));
        } else {
            ui.label("Click 'Run Benchmark' to measure performance.");
        }
    }

    /// Render the "Learn" tab
    fn render_learn_tab(&mut self, ui: &mut egui::Ui) {
        // Sub-tabs for Learn Mode
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.learn_mode, LearnMode::XorCipher, "XOR Cipher");
            ui.selectable_value(&mut self.learn_mode, LearnMode::ShannonEntropy, "Shannon Entropy");
            ui.selectable_value(&mut self.learn_mode, LearnMode::NistFrequency, "NIST Frequency");
        });
        ui.separator();
        ui.add_space(10.0);

        match self.learn_mode {
            LearnMode::XorCipher => self.render_xor_visualizer(ui),
            LearnMode::ShannonEntropy => self.render_entropy_visualizer(ui),
            LearnMode::NistFrequency => self.render_nist_visualizer(ui),
        }
    }

    fn render_xor_visualizer(&mut self, ui: &mut egui::Ui) {
        // Update animation state if playing
        let time = ui.input(|i| i.time);
        self.learn_process.update(time);
        if self.learn_process.is_playing {
            ui.ctx().request_repaint();
        }

        ui.heading("Learn Cryptography: XOR Cipher");
        ui.label("Visualize how stream ciphers work step-by-step.");
        ui.add_space(10.0);

        // Input Section
        ui.horizontal(|ui| {
            ui.label("Input:");
            ui.text_edit_singleline(&mut self.learn_input);
            if ui.button("Start Visualization").clicked() {
                self.learn_process.start(&self.learn_input);
            }
        });

        ui.add_space(20.0);

        // Visualization Section
        if let Some(step) = self.learn_process.current_step() {
            let total_steps = self.learn_process.steps.len();
            let current_idx = self.learn_process.current_step_index + 1;

            ui.heading(format!("Step {} of {}: Encrypting '{}'", current_idx, total_steps, step.character));
            ui.add_space(10.0);

            // Frame for visualization
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.set_min_width(500.0);
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);

                    let input_color = egui::Color32::from_rgb(100, 200, 255); // Cyan-ish
                    let key_color = egui::Color32::from_rgb(255, 200, 100);   // Orange-ish
                    let result_color = egui::Color32::from_rgb(100, 255, 100); // Green-ish

                    // Input Row
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Input:").strong());
                        ui.add_space(20.0);
                        ui.label(format!("'{}'", step.character));
                        ui.label("=");
                        ui.label(format!("{}", step.input_byte));
                        ui.label("=");

                        for bit_op in &step.bit_ops {
                            let text = if bit_op.input_bit { "1" } else { "0" };
                            ui.label(egui::RichText::new(text).color(input_color).font(egui::FontId::monospace(20.0)));
                            ui.add_space(2.0);
                        }
                    });

                    ui.add_space(5.0);
                    ui.label(egui::RichText::new("↓").strong());
                    ui.add_space(5.0);

                    // Key Row
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Key:  ").strong());
                        ui.add_space(20.0);
                        ui.label("   "); // align with char
                        ui.label(" ");
                        ui.label(format!("{}", step.keystream_byte));
                        ui.label("=");

                        for bit_op in &step.bit_ops {
                            let text = if bit_op.key_bit { "1" } else { "0" };
                            ui.label(egui::RichText::new(text).color(key_color).font(egui::FontId::monospace(20.0)));
                            ui.add_space(2.0);
                        }
                    });

                    ui.add_space(5.0);
                    // XOR symbol row or line
                    ui.separator();
                    ui.add_space(5.0);

                    // Result Row
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("XOR:  ").strong());
                        ui.add_space(20.0);
                        ui.label("   "); // align
                        ui.label(" ");
                        ui.label(format!("{}", step.result_byte));
                        ui.label("=");

                        for bit_op in &step.bit_ops {
                            let text = if bit_op.result_bit { "1" } else { "0" };
                            ui.label(egui::RichText::new(text).color(result_color).font(egui::FontId::monospace(20.0)));
                            ui.add_space(2.0);
                        }
                    });

                    ui.add_space(10.0);

                    // Explanation of XOR for current step
                    ui.label("Operation: 0⊕1=1, 1⊕0=1, 0⊕0=0, 1⊕1=0");
                    ui.label("If bits are different, result is 1. If same, result is 0.");

                    ui.add_space(10.0);
                });
            });

            ui.add_space(20.0);

            // Controls
            ui.horizontal(|ui| {
                if ui.button("⬅ Previous").clicked() {
                    self.learn_process.prev_step();
                }

                let play_label = if self.learn_process.is_playing { "⏸ Pause" } else { "▶ Play" };
                if ui.button(play_label).clicked() {
                    self.learn_process.toggle_play();
                }

                if ui.button("Next ➡").clicked() {
                    self.learn_process.next_step();
                }

                ui.add_space(20.0);
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.learn_process.speed, 0.1..=5.0).text("steps/s"));
            });

            ui.add_space(20.0);

            // Progress/Output Summary
            ui.group(|ui| {
                ui.heading("Encryption Progress");
                ui.horizontal_wrapped(|ui| {
                    for (i, s) in self.learn_process.steps.iter().enumerate() {
                        let text = format!("{:02X}", s.result_byte);
                        let mut rich_text = egui::RichText::new(text).monospace();

                        if i == self.learn_process.current_step_index {
                            rich_text = rich_text.strong().color(egui::Color32::GREEN).background_color(egui::Color32::from_gray(60));
                        } else if i < self.learn_process.current_step_index {
                             rich_text = rich_text.color(egui::Color32::LIGHT_GRAY);
                        } else {
                             rich_text = rich_text.color(egui::Color32::DARK_GRAY);
                        }

                        ui.label(rich_text);
                    }
                });
            });

            // Educational Context
            ui.add_space(20.0);
            ui.collapsing("How it works", |ui| {
                ui.label("1. Convert character to ASCII number (e.g., 'H' = 72)");
                ui.label("2. Convert to binary (72 = 01001000)");
                ui.label("3. Generate a random keystream byte from entropy source");
                ui.label("4. XOR the input byte with the keystream byte");
                ui.label("5. The result is the encrypted ciphertext byte");
            });

        } else {
            ui.label("Enter text and click 'Start Visualization' to begin.");
        }
    }

    fn render_entropy_visualizer(&mut self, ui: &mut egui::Ui) {
        use crate::learn::entropy_visual::EntropyStepType;

        // Update animation state if playing
        let time = ui.input(|i| i.time);
        self.entropy_process.update(time);
        if self.entropy_process.is_playing {
            ui.ctx().request_repaint();
        }

        ui.heading("Learn Shannon Entropy");
        ui.label("Measures 'randomness' or 'unpredictability' of data.");
        ui.add_space(10.0);

        // Input Section
        ui.horizontal(|ui| {
            ui.label("Input:");
            ui.text_edit_singleline(&mut self.entropy_process.input);
            if ui.button("Calculate").clicked() {
                self.entropy_process.start(&self.entropy_process.input.clone());
            }
        });

        ui.add_space(20.0);

        if let Some(step) = self.entropy_process.current_step() {
             let total_steps = self.entropy_process.steps.len();
             let current_idx = self.entropy_process.current_step_index + 1;

             let step_title = match step.step_type {
                 EntropyStepType::CountBytes => "Count Byte Frequencies",
                 EntropyStepType::CalculateProbabilities => "Calculate Probabilities",
                 EntropyStepType::CalculateContributions => "Calculate Entropy Contributions",
                 EntropyStepType::SumEntropy => "Sum for Total Entropy",
                 EntropyStepType::Interpret => "Interpretation",
             };

             ui.heading(format!("Step {} of {}: {}", current_idx, total_steps, step_title));

             ui.add_space(10.0);

             // Visualization Area
             egui::Frame::canvas(ui.style()).show(ui, |ui| {
                 ui.set_min_width(600.0);
                 ui.vertical(|ui| {
                    ui.add_space(10.0);

                    // Show Frequency Table
                    if !step.byte_counts.is_empty() {
                         egui::Grid::new("entropy_grid").striped(true).spacing([20.0, 5.0]).show(ui, |ui| {
                            ui.label(egui::RichText::new("Byte").strong());
                            ui.label(egui::RichText::new("Count").strong());
                            if step.step_type != EntropyStepType::CountBytes {
                                ui.label(egui::RichText::new("Probability").strong());
                                ui.label(egui::RichText::new("Contribution").strong());
                            }
                            ui.label(egui::RichText::new("Visual").strong());
                            ui.end_row();

                            // Sort bytes for consistent display
                            let mut bytes: Vec<_> = step.byte_counts.keys().collect();
                            bytes.sort();

                            let max_count = step.byte_counts.values().max().copied().unwrap_or(1) as f32;

                            for &byte in bytes {
                                let count = step.byte_counts[&byte];
                                let char_repr = if byte >= 32 && byte <= 126 {
                                    (byte as char).to_string()
                                } else {
                                    format!("0x{:02X}", byte)
                                };

                                ui.label(format!("'{}'", char_repr));
                                ui.label(format!("{}", count));

                                if step.step_type != EntropyStepType::CountBytes {
                                    let p = step.probabilities.get(&byte).unwrap_or(&0.0);
                                    let contrib = step.entropy_contributions.get(&byte).unwrap_or(&0.0);

                                    ui.label(format!("{:.3}", p));

                                    if matches!(step.step_type, EntropyStepType::CalculateContributions | EntropyStepType::SumEntropy | EntropyStepType::Interpret) {
                                         ui.label(format!("{:.3} bits", contrib));
                                    } else {
                                        ui.label("-");
                                    }
                                }

                                // Bar chart
                                let width = (count as f32 / max_count) * 100.0;
                                let color = if count == 1 { egui::Color32::GREEN } else { egui::Color32::from_rgb(255, 100 + (155 - (count * 20).min(155)) as u8, 100) };
                                let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 15.0), egui::Sense::hover());
                                ui.painter().rect_filled(rect, 2.0, color);
                                ui.end_row();
                            }
                         });
                    }

                    ui.add_space(20.0);

                    // Explanation / Formula area
                    match step.step_type {
                        EntropyStepType::CountBytes => {
                            ui.label("We count how many times each unique byte appears in the input.");
                        },
                        EntropyStepType::CalculateProbabilities => {
                            ui.label("Probability P(x) = Count(x) / Total Bytes");
                            ui.label("This tells us how likely each character is to appear.");
                        },
                        EntropyStepType::CalculateContributions => {
                            ui.label("Contribution = -P(x) * log2(P(x))");
                            ui.label("Rare events provide MORE information (higher contribution) per occurrence.");
                        },
                        EntropyStepType::SumEntropy | EntropyStepType::Interpret => {
                            ui.heading(format!("Total Entropy: {:.4} bits/byte", step.current_entropy_sum));
                            ui.label(format!("Maximum Possible: {:.4} bits/byte", step.max_entropy));

                            let efficiency = if step.max_entropy > 0.0 { (step.current_entropy_sum / step.max_entropy) * 100.0 } else { 0.0 };
                             ui.label(format!("Efficiency: {:.1}%", efficiency));

                             if step.step_type == EntropyStepType::Interpret {
                                 ui.separator();
                                 if step.current_entropy_sum < 2.0 {
                                     ui.label("Low entropy. The input is very predictable.");
                                 } else if efficiency > 80.0 {
                                     ui.label("High entropy! The input looks quite random or uses many different characters.");
                                 } else {
                                     ui.label("Moderate entropy.");
                                 }
                             }
                        }
                    }
                    ui.add_space(10.0);
                 });
             });

             ui.add_space(20.0);

             // Controls
             ui.horizontal(|ui| {
                if ui.button("⬅ Previous").clicked() {
                    self.entropy_process.prev_step();
                }

                let play_label = if self.entropy_process.is_playing { "⏸ Pause" } else { "▶ Play" };
                if ui.button(play_label).clicked() {
                    self.entropy_process.toggle_play();
                }

                if ui.button("Next ➡").clicked() {
                    self.entropy_process.next_step();
                }

                ui.add_space(20.0);
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.entropy_process.speed, 0.1..=5.0).text("steps/s"));
            });
        } else {
             ui.label("Enter text (e.g., 'AAABBC') and click Calculate.");
        }
    }

    fn render_nist_visualizer(&mut self, ui: &mut egui::Ui) {
        use crate::learn::nist_visual::NistStepType;

         // Update animation state if playing
        let time = ui.input(|i| i.time);
        self.nist_process.update(time);
        if self.nist_process.is_playing {
            ui.ctx().request_repaint();
        }

        ui.heading("Learn NIST Frequency Test");
        ui.label("Checks if the number of 1s and 0s are approximately equal (like a fair coin flip).");
        ui.add_space(10.0);

        // Input Section
        ui.horizontal(|ui| {
            ui.label("Input:");
            ui.text_edit_singleline(&mut self.nist_process.input_text);
            if ui.button("Analyze").clicked() {
                self.nist_process.start(&self.nist_process.input_text.clone());
            }
            if ui.button("Generate Random").clicked() {
                self.nist_process.generate_random(16);
            }
        });

        ui.add_space(20.0);

        if let Some(step) = self.nist_process.current_step() {
             let total_steps = self.nist_process.steps.len();
             let current_idx = self.nist_process.current_step_index + 1;

             let step_title = match step.step_type {
                 NistStepType::ConvertToBits => "Convert to Bits",
                 NistStepType::CountOnesZeros => "Count Ones and Zeros",
                 NistStepType::CalculateStatistic => "Calculate Statistic",
                 NistStepType::CalculatePValue => "Calculate P-Value",
                 NistStepType::Interpret => "Interpretation",
             };

             ui.heading(format!("Step {} of {}: {}", current_idx, total_steps, step_title));
             ui.add_space(10.0);

             egui::Frame::canvas(ui.style()).show(ui, |ui| {
                 ui.set_min_width(600.0);
                 ui.vertical(|ui| {
                     ui.add_space(10.0);

                     // Bit visualization
                     ui.label(egui::RichText::new("Bit Sequence:").strong());
                     ui.horizontal_wrapped(|ui| {
                         for (i, &bit) in step.bits.iter().enumerate() {
                             if i > 0 && i % 8 == 0 {
                                 ui.add_space(5.0);
                             }
                             let color = if bit == 1 { egui::Color32::GREEN } else { egui::Color32::LIGHT_GRAY };
                             let text = egui::RichText::new(format!("{}", bit)).color(color).monospace().strong();
                             ui.label(text);
                         }
                     });

                     ui.add_space(10.0);
                     ui.separator();
                     ui.add_space(10.0);

                     if step.step_type != NistStepType::ConvertToBits {
                         ui.horizontal(|ui| {
                             ui.vertical(|ui| {
                                 ui.label("Ones:");
                                 ui.heading(format!("{}", step.ones_count));
                             });
                             ui.add_space(40.0);
                             ui.vertical(|ui| {
                                 ui.label("Zeros:");
                                 ui.heading(format!("{}", step.zeros_count));
                             });
                         });

                         // Visual bar
                         let total = step.bits.len() as f32;
                         if total > 0.0 {
                             let ones_pct = step.ones_count as f32 / total;
                             ui.add_space(5.0);
                             let (rect, _) = ui.allocate_exact_size(egui::vec2(300.0, 20.0), egui::Sense::hover());

                             // Draw zeros background
                             ui.painter().rect_filled(rect, 2.0, egui::Color32::LIGHT_GRAY);

                             // Draw ones foreground
                             let ones_width = rect.width() * ones_pct;
                             let ones_rect = egui::Rect::from_min_size(rect.min, egui::vec2(ones_width, rect.height()));
                             ui.painter().rect_filled(ones_rect, 2.0, egui::Color32::GREEN);

                             // Center line (ideal)
                             let center_x = rect.min.x + rect.width() * 0.5;
                             ui.painter().line_segment(
                                 [egui::pos2(center_x, rect.min.y), egui::pos2(center_x, rect.max.y)],
                                 egui::Stroke::new(2.0, egui::Color32::BLACK)
                             );

                             ui.label(format!("Ratio: {:.1}% Ones (Ideal: 50%)", ones_pct * 100.0));
                         }
                     }

                     ui.add_space(20.0);

                     // Stats and interpretation
                     if matches!(step.step_type, NistStepType::CalculateStatistic | NistStepType::CalculatePValue | NistStepType::Interpret) {
                         ui.group(|ui| {
                             ui.vertical(|ui| {
                                 ui.label(format!("Sum (+1 for 1, -1 for 0): {}", step.sum));
                                 ui.label(format!("S_obs (|Sum| / √n): {:.4}", step.s_obs));

                                 if matches!(step.step_type, NistStepType::CalculatePValue | NistStepType::Interpret) {
                                     ui.add_space(5.0);
                                     ui.label(format!("P-Value (erfc(S_obs/√2)): {:.4}", step.p_value));
                                 }

                                 if step.step_type == NistStepType::Interpret {
                                     ui.add_space(10.0);
                                     if step.passed {
                                         ui.colored_label(egui::Color32::GREEN, "✅ PASS: The sequence looks random.");
                                     } else {
                                         ui.colored_label(egui::Color32::RED, "❌ FAIL: The sequence has too many 1s or 0s.");
                                     }
                                     ui.label("(Threshold: P-value ≥ 0.01)");
                                 }
                             });
                         });
                     }

                     ui.add_space(10.0);
                 });
             });

             ui.add_space(20.0);

             // Controls
             ui.horizontal(|ui| {
                if ui.button("⬅ Previous").clicked() {
                    self.nist_process.prev_step();
                }

                let play_label = if self.nist_process.is_playing { "⏸ Pause" } else { "▶ Play" };
                if ui.button(play_label).clicked() {
                    self.nist_process.toggle_play();
                }

                if ui.button("Next ➡").clicked() {
                    self.nist_process.next_step();
                }

                ui.add_space(20.0);
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.nist_process.speed, 0.1..=5.0).text("steps/s"));
            });
        } else {
            ui.label("Enter text or generate random bytes to start.");
        }
    }
}
