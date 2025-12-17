//! Main GUI application

use eframe::egui;
use crate::entropy::{EntropySource, SystemEntropy};
use crate::crypto::StreamCipher;
use crate::quality::{QualityMetrics, NistTests};
use crate::bench::{PerformanceBench, BenchmarkResult};

/// Main application state
pub struct EntropyWeaverApp {
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
}

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Use,
    Test,
    Benchmark,
}

impl Default for EntropyWeaverApp {
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
        }
    }
}

impl eframe::App for EntropyWeaverApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.heading("🔐 Entropy Weaver");
            ui.label(format!("Source: {}", self.entropy.name()));
            ui.add_space(5.0);
            ui.separator();
            
            // Tabs
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Use, "📝 Use");
                ui.selectable_value(&mut self.current_tab, Tab::Test, "🔬 Test");
                ui.selectable_value(&mut self.current_tab, Tab::Benchmark, "⚡ Benchmark");
            });
            
            ui.separator();
            ui.add_space(10.0);
            
            // Tab content
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.current_tab {
                    Tab::Use => self.render_use_tab(ui),
                    Tab::Test => self.render_test_tab(ui),
                    Tab::Benchmark => self.render_benchmark_tab(ui),
                }
            });
        });
    }
}

impl EntropyWeaverApp {
    /// Render the "Use" tab (stream cipher)
    fn render_use_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Stream Cipher");
        ui.label("Encrypt or decrypt data using the entropy source as keystream.");
        ui.add_space(10.0);
        
        // Input
        ui.horizontal(|ui| {
            ui.label("Input:");
            ui.text_edit_singleline(&mut self.cipher_input);
        });
        
        ui.checkbox(&mut self.cipher_hex, "Display output as hex");
        
        ui.add_space(10.0);
        
        // Encrypt button
        if ui.button("🔒 Encrypt / Decrypt").clicked() {
            let mut cipher = StreamCipher::new(&mut *self.entropy);
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
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(30.0, 30.0),
                            egui::Sense::hover()
                        );
                        ui.painter().rect_filled(rect, 2.0, color);
                        
                        // Tooltip
                        if ui.rect_contains_pointer(rect) {
                            egui::show_tooltip_at_pointer(ui.ctx(), egui::Id::new("state_tooltip"), |ui| {
                                ui.label(format!("Byte {}: 0x{:02X} ({})", i, byte, byte));
                            });
                        }
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
            ui.horizontal(|ui| {
                ui.label("Shannon Entropy:");
                ui.label(format!("{:.4} bits/byte", metrics.shannon_entropy));
            });
            let shannon_progress = (metrics.shannon_entropy / 8.0) as f32;
            ui.add(egui::ProgressBar::new(shannon_progress)
                .text(format!("{:.1}%", shannon_progress * 100.0)));
            
            ui.add_space(5.0);
            
            // Min-entropy
            ui.horizontal(|ui| {
                ui.label("Min-Entropy:");
                ui.label(format!("{:.4} bits/byte", metrics.min_entropy));
            });
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
            ui.heading("NIST SP 800-22 Tests");
            ui.label("P-values ≥ 0.01 indicate randomness");
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
                ui.label("Throughput:");
                ui.heading(format!("{:.2} MB/s", result.throughput_mbps));
            });
            
            ui.add_space(5.0);
            
            // Latency
            ui.horizontal(|ui| {
                ui.label("Latency per byte:");
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
}
