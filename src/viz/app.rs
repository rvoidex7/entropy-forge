//! Main GUI application.

use std::sync::mpsc::{self, Receiver, TryRecvError};

use eframe::egui;

use crate::bench::{BenchmarkResult, PerformanceBench};
use crate::crypto::KeyedStreamCipher;
use crate::entropy::{EntropySource, SystemEntropy};
use crate::learn::entropy_visual::interpret_entropy;
use crate::learn::{EncryptionProcess, EntropyProcess, NistProcess};
use crate::quality::{NistTests, QualityMetrics};

use super::{FIELD_GAP, SECTION_GAP};

/// Result of a background quality-analysis run.
struct TestOutcome {
    metrics: QualityMetrics,
    nist: Vec<(String, f64)>,
}

/// Direction of the last cipher action.
///
/// The "Use output as input" button uses it to decide whether the last output
/// was ciphertext or plaintext.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CipherMode {
    Encrypt,
    Decrypt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LearnMode {
    XorCipher,
    ShannonEntropy,
    NistFrequency,
}

/// Top-level application tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Use,
    Test,
    Benchmark,
    Learn,
}

/// Main application state.
pub struct EntropyForgeApp {
    // Entropy source (used for the displayed name; analysis runs on a worker
    // thread with its own source so the UI never blocks).
    entropy: Box<dyn EntropySource>,

    // Current tab.
    current_tab: Tab,

    // Use tab state.
    passphrase: String,
    nonce_hex: String,
    cipher_mode: CipherMode,
    cipher_input: String,
    cipher_output: String,
    cipher_output_bytes: Vec<u8>,
    cipher_hex: bool,
    cipher_error: Option<String>,
    cipher_notice: Option<String>,

    // Test tab state.
    quality_metrics: Option<QualityMetrics>,
    nist_results: Vec<(String, f64)>,
    quality_sample_size: usize,
    is_testing: bool,
    test_rx: Option<Receiver<TestOutcome>>,
    test_error: Option<String>,

    // Benchmark tab state.
    bench_result: Option<BenchmarkResult>,
    bench_size: usize,
    is_benchmarking: bool,
    bench_rx: Option<Receiver<BenchmarkResult>>,

    // Learn tab state.
    learn_mode: LearnMode,
    learn_process: EncryptionProcess,
    learn_input: String,
    entropy_process: EntropyProcess,
    nist_process: NistProcess,
}

impl Default for EntropyForgeApp {
    fn default() -> Self {
        Self {
            entropy: Box::new(SystemEntropy::new()),
            current_tab: Tab::Use,
            passphrase: String::from("entropy-forge-demo"),
            nonce_hex: hex::encode(KeyedStreamCipher::random_nonce()),
            cipher_mode: CipherMode::Encrypt,
            cipher_input: String::from("Hello, World!"),
            cipher_output: String::new(),
            cipher_output_bytes: Vec::new(),
            cipher_hex: true,
            cipher_error: None,
            cipher_notice: None,
            quality_metrics: None,
            nist_results: Vec::new(),
            quality_sample_size: 100_000,
            is_testing: false,
            test_rx: None,
            test_error: None,
            bench_result: None,
            bench_size: 1_000_000,
            is_benchmarking: false,
            bench_rx: None,
            learn_mode: LearnMode::XorCipher,
            learn_process: EncryptionProcess::new(),
            learn_input: String::from("Hello"),
            entropy_process: EntropyProcess::new(),
            nist_process: NistProcess::new(),
        }
    }
}

impl EntropyForgeApp {
    /// Create the app with a specific tab selected.
    pub fn for_tab(tab: Tab) -> Self {
        Self {
            current_tab: tab,
            ..Self::default()
        }
    }

    /// Currently selected tab.
    pub fn current_tab(&self) -> Tab {
        self.current_tab
    }

    /// Select a tab programmatically.
    pub fn select_tab(&mut self, tab: Tab) {
        self.current_tab = tab;
    }
}

impl eframe::App for EntropyForgeApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Poll background jobs before rendering so results appear promptly.
        self.poll_test_job();
        self.poll_bench_job();

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("🔐 Entropy Forge");
            ui.label(format!("Source: {}", self.entropy.name()));
            ui.add_space(5.0);
            ui.separator();

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Use, "📝 Use");
                ui.selectable_value(&mut self.current_tab, Tab::Test, "🔬 Test");
                ui.selectable_value(&mut self.current_tab, Tab::Benchmark, "⚡ Benchmark");
                ui.selectable_value(&mut self.current_tab, Tab::Learn, "🎓 Learn");
            });

            ui.separator();
            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| match self.current_tab {
                Tab::Use => self.render_use_tab(ui),
                Tab::Test => self.render_test_tab(ui),
                Tab::Benchmark => self.render_benchmark_tab(ui),
                Tab::Learn => self.render_learn_tab(ui),
            });
        });
    }
}

impl EntropyForgeApp {
    /// Helper to render consistent educational tooltips.
    fn render_explanation_tooltip(ui: &mut egui::Ui, label: &str, text: &str) {
        ui.horizontal(|ui| {
            ui.label(label);
            ui.label("ℹ").on_hover_text(text);
        });
    }

    /// Render the "Use" tab (ChaCha20 stream cipher).
    fn render_use_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("ChaCha20 Stream Cipher");
        ui.label(
            "Encrypt or decrypt data with a real ChaCha20 keystream. \
             Encryption and decryption are exact inverses for the same passphrase and nonce.",
        );

        ui.add_space(SECTION_GAP);

        // Key material: passphrase and nonce.
        let passphrase_label = ui.label(egui::RichText::new("Passphrase").strong());
        ui.add(
            egui::TextEdit::singleline(&mut self.passphrase).desired_width(ui.available_width()),
        )
        .labelled_by(passphrase_label.id);
        ui.label(egui::RichText::new("Hashed with SHA-256 to form the 256-bit key.").weak());

        ui.add_space(FIELD_GAP);

        let nonce_label =
            ui.label(egui::RichText::new("Nonce (96 bits, 24 hex characters)").strong());
        ui.horizontal_wrapped(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.nonce_hex)
                    .desired_width(230.0_f32.min(ui.available_width()))
                    .font(egui::TextStyle::Monospace),
            )
            .labelled_by(nonce_label.id);
            if ui
                .button("🎲 New")
                .on_hover_text("Generate a fresh random nonce from the operating system.")
                .clicked()
            {
                self.nonce_hex = hex::encode(KeyedStreamCipher::random_nonce());
            }
        });
        ui.label(egui::RichText::new("Never reuse a nonce with the same passphrase.").weak());

        section_break(ui);

        // Message and actions.
        let input_label = ui.label(egui::RichText::new("Input").strong());
        ui.add(
            egui::TextEdit::multiline(&mut self.cipher_input)
                .desired_width(ui.available_width())
                .desired_rows(3),
        )
        .labelled_by(input_label.id);

        ui.add_space(FIELD_GAP);

        ui.horizontal_wrapped(|ui| {
            if ui.button("🔒 Encrypt").clicked() {
                self.run_cipher(CipherMode::Encrypt);
            }
            if ui.button("🔓 Decrypt").clicked() {
                self.run_cipher(CipherMode::Decrypt);
            }
            if ui
                .checkbox(&mut self.cipher_hex, "Ciphertext as hex")
                .on_hover_text(
                    "Encrypt: show the ciphertext as hex.\n\
                     Decrypt: read the ciphertext from the input as hex.",
                )
                .changed()
            {
                self.refresh_cipher_output();
            }
        });
        ui.label(
            egui::RichText::new(
                "With 'Ciphertext as hex' checked, Encrypt shows hex output and Decrypt reads hex input.",
            )
            .weak(),
        );

        ui.add_space(FIELD_GAP);

        ui.horizontal_wrapped(|ui| {
            let has_output = !self.cipher_output_bytes.is_empty();
            if ui
                .add_enabled(has_output, egui::Button::new("↩ Use output as input"))
                .on_hover_text(
                    "Load the ciphertext as hex and switch to Decrypt, or load the \
                     decrypted text and switch to Encrypt.",
                )
                .clicked()
            {
                self.use_output_as_input();
            }
            if ui
                .button("Learn how this works")
                .on_hover_text(
                    "Opens the XOR visualizer. ChaCha20 adds key mixing on top of the \
                     same XOR-with-keystream idea.",
                )
                .clicked()
            {
                self.current_tab = Tab::Learn;
                self.learn_mode = LearnMode::XorCipher;
                self.learn_input = self.cipher_input.clone();
                self.learn_process.start(&self.learn_input);
            }
        });

        ui.add_space(SECTION_GAP);

        // Output.
        let output_label_text = if self.cipher_output_bytes.is_empty() {
            "Output"
        } else {
            match self.cipher_mode {
                CipherMode::Encrypt if self.cipher_hex => "Ciphertext (hex)",
                CipherMode::Encrypt => "Ciphertext (binary, shown as lossy text)",
                CipherMode::Decrypt => "Plaintext",
            }
        };
        let output_label = ui.label(egui::RichText::new(output_label_text).strong());
        ui.add(
            egui::TextEdit::multiline(&mut self.cipher_output)
                .desired_width(ui.available_width())
                .desired_rows(3)
                .font(egui::TextStyle::Monospace),
        )
        .labelled_by(output_label.id);

        if let Some(error) = &self.cipher_error {
            ui.add_space(FIELD_GAP);
            ui.colored_label(egui::Color32::RED, error);
        }

        if let Some(notice) = &self.cipher_notice {
            ui.add_space(FIELD_GAP);
            ui.colored_label(egui::Color32::from_rgb(220, 160, 30), notice);
        }

        if !self.cipher_output_bytes.is_empty() {
            ui.add_space(FIELD_GAP);
            ui.label(format!(
                "Output length: {} bytes",
                self.cipher_output_bytes.len()
            ));
        }

        ui.add_space(SECTION_GAP);
        ui.collapsing("How ChaCha20 differs from plain XOR", |ui| {
            ui.label(
                "• A plain XOR one-time pad is malleable: flipping one ciphertext \
                 bit flips exactly one plaintext bit. It has no avalanche effect.",
            );
            ui.label(
                "• ChaCha20 mixes the key and nonce through 20 rounds before XORing, \
                 so the keystream looks random and the same key/nonce pair is reusable \
                 only once.",
            );
            ui.label(
                "• Never reuse a nonce with the same key: XORing two ciphertexts then \
                 cancels the keystream (the two-time pad attack). Regenerate the nonce \
                 for every message.",
            );
            ui.label(
                "• This demo derives the key with SHA-256 for simplicity. Production \
                 software should use Argon2 or PBKDF2 with a random salt.",
            );
        });
    }

    /// Run the cipher in `mode` with the current passphrase and nonce.
    fn run_cipher(&mut self, mode: CipherMode) {
        self.cipher_mode = mode;
        self.cipher_notice = None;

        let nonce = match parse_nonce(&self.nonce_hex) {
            Ok(nonce) => nonce,
            Err(error) => {
                self.cipher_error = Some(error);
                self.cipher_output.clear();
                self.cipher_output_bytes.clear();
                return;
            }
        };

        let input_bytes: Vec<u8> = match mode {
            // In decrypt mode the hex toggle promises hex input: reject bad hex
            // instead of silently treating the characters as raw bytes.
            CipherMode::Decrypt if self.cipher_hex => {
                // Allow pasted hex with spaces or line breaks.
                let cleaned: String = self
                    .cipher_input
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                match hex::decode(&cleaned) {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        self.cipher_error = Some(format!(
                            "Invalid hex ciphertext: {error}. Uncheck 'Ciphertext as hex' \
                             to decrypt raw text bytes."
                        ));
                        self.cipher_output.clear();
                        self.cipher_output_bytes.clear();
                        return;
                    }
                }
            }
            _ => self.cipher_input.as_bytes().to_vec(),
        };

        self.cipher_output_bytes =
            KeyedStreamCipher::process_with_passphrase(&self.passphrase, &nonce, &input_bytes);
        self.apply_output_display();
        self.cipher_error = None;
    }

    /// Format the raw output bytes for the current mode and hex setting.
    ///
    /// The hex toggle describes the *ciphertext*, so decrypted plaintext is
    /// always shown as UTF-8 text. Otherwise a successful decrypt would look
    /// like a hex dump and seem to have failed.
    fn format_cipher_output(
        mode: CipherMode,
        hex_output: bool,
        bytes: &[u8],
    ) -> (String, Option<String>) {
        if bytes.is_empty() {
            return (String::new(), None);
        }

        if mode == CipherMode::Encrypt && hex_output {
            return (hex::encode(bytes), None);
        }

        match std::str::from_utf8(bytes) {
            Ok(text) => (text.to_string(), None),
            Err(_) => {
                let notice = match mode {
                    CipherMode::Encrypt => {
                        "Ciphertext is binary and not valid UTF-8; showing lossy text. \
                         Enable 'Ciphertext as hex' for the exact value."
                    }
                    CipherMode::Decrypt => {
                        "Decrypted bytes are not valid UTF-8; showing lossy text. \
                         Check the passphrase, nonce, and hex input."
                    }
                };
                (
                    String::from_utf8_lossy(bytes).to_string(),
                    Some(notice.to_string()),
                )
            }
        }
    }

    /// Re-render the output text from the stored bytes without re-running the
    /// cipher. Used when the hex toggle changes.
    fn refresh_cipher_output(&mut self) {
        if self.cipher_output_bytes.is_empty() {
            return;
        }
        self.apply_output_display();
    }

    /// Update the displayed text and warning from `cipher_output_bytes`.
    fn apply_output_display(&mut self) {
        let (display, notice) = Self::format_cipher_output(
            self.cipher_mode,
            self.cipher_hex,
            &self.cipher_output_bytes,
        );
        self.cipher_output = display;
        self.cipher_notice = notice;
    }

    /// Move the last output into the input box and flip the direction.
    fn use_output_as_input(&mut self) {
        if self.cipher_output_bytes.is_empty() {
            return;
        }

        match self.cipher_mode {
            CipherMode::Encrypt => {
                // Ciphertext is binary: load it as hex so Decrypt can parse it.
                self.cipher_input = hex::encode(&self.cipher_output_bytes);
                self.cipher_hex = true;
                self.cipher_mode = CipherMode::Decrypt;
                self.cipher_notice = Some(
                    "Ciphertext loaded as hex. Keep the same passphrase and nonce.".to_string(),
                );
            }
            CipherMode::Decrypt => {
                self.cipher_input = String::from_utf8_lossy(&self.cipher_output_bytes).to_string();
                self.cipher_mode = CipherMode::Encrypt;
                self.cipher_notice = Some("Plaintext loaded as input.".to_string());
            }
        }

        self.cipher_output.clear();
        self.cipher_output_bytes.clear();
        self.cipher_error = None;
    }

    /// Poll the background quality-analysis job.
    fn poll_test_job(&mut self) {
        let mut outcome = None;
        let mut disconnected = false;

        if let Some(rx) = self.test_rx.as_ref() {
            match rx.try_recv() {
                Ok(result) => outcome = Some(result),
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => disconnected = true,
            }
        }

        if let Some(outcome) = outcome {
            self.quality_metrics = Some(outcome.metrics);
            self.nist_results = outcome.nist;
            self.is_testing = false;
            self.test_rx = None;
        } else if disconnected {
            self.is_testing = false;
            self.test_rx = None;
            self.test_error = Some("The analysis worker stopped unexpectedly.".to_string());
        }
    }

    /// Poll the background benchmark job.
    fn poll_bench_job(&mut self) {
        let mut result = None;
        let mut disconnected = false;

        if let Some(rx) = self.bench_rx.as_ref() {
            match rx.try_recv() {
                Ok(benchmark) => result = Some(benchmark),
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => disconnected = true,
            }
        }

        if let Some(benchmark) = result {
            self.bench_result = Some(benchmark);
            self.is_benchmarking = false;
            self.bench_rx = None;
        } else if disconnected {
            self.is_benchmarking = false;
            self.bench_rx = None;
        }
    }

    /// Render the "Test" tab (quality metrics and NIST tests).
    fn render_test_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Quality Analysis");
        ui.label("Analyze entropy quality with statistical tests.");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            let label = ui.label("Sample size:");
            ui.add(
                egui::Slider::new(&mut self.quality_sample_size, 1_000..=1_000_000)
                    .logarithmic(true)
                    .suffix(" bytes"),
            )
            .labelled_by(label.id);
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("🔬 Run All Tests").clicked() && !self.is_testing {
                self.start_test_job();
            }

            if self.is_testing {
                ui.spinner();
                ui.label("Testing in background...");
                ui.ctx().request_repaint();
            }
        });

        if let Some(error) = &self.test_error {
            ui.colored_label(egui::Color32::RED, error);
        }

        ui.add_space(20.0);

        if self.quality_metrics.is_some() {
            self.render_metrics_section(ui);
            self.render_nist_section(ui);
        } else {
            ui.label("Click 'Run All Tests' to analyze entropy quality.");
        }
    }

    fn start_test_job(&mut self) {
        self.is_testing = true;
        self.test_error = None;
        let sample_size = self.quality_sample_size;
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            let mut source = SystemEntropy::new();
            let mut data = vec![0u8; sample_size];
            source.fill_bytes(&mut data);

            // Both analyses share one sample so the displayed metrics and NIST
            // results describe exactly the same bytes.
            let metrics = QualityMetrics::analyze_slice(&data);
            let nist = NistTests::run_all_tests(&data)
                .into_iter()
                .map(|(name, p_value)| (name.to_string(), p_value))
                .collect();

            let _ = tx.send(TestOutcome { metrics, nist });
        });

        self.test_rx = Some(rx);
    }

    fn render_metrics_section(&mut self, ui: &mut egui::Ui) {
        let Some(metrics) = self.quality_metrics.as_ref() else {
            return;
        };

        ui.heading("Entropy Metrics");
        ui.add_space(5.0);

        Self::render_explanation_tooltip(
            ui,
            "Shannon Entropy:",
            "Average unpredictability of the bytes.\n8.0 bits/byte is the maximum.\n\
             High Shannon entropy alone does NOT prove cryptographic security.",
        );
        ui.label(format!("{:.4} bits/byte", metrics.shannon_entropy));
        let shannon_progress = (metrics.shannon_entropy / 8.0) as f32;
        ui.add(egui::ProgressBar::new(shannon_progress).text(format!(
            "{:.1}% of the 8 bits/byte maximum",
            shannon_progress * 100.0
        )));

        ui.add_space(5.0);

        Self::render_explanation_tooltip(
            ui,
            "Min-Entropy (MCV, 99% bound):",
            "NIST SP 800-90B most-common-value estimate with a 99% upper \
             confidence bound. This is the conservative worst-case guessability \
             used for security arguments.",
        );
        ui.label(format!("{:.4} bits/byte", metrics.min_entropy));
        let min_progress = (metrics.min_entropy / 8.0) as f32;
        ui.add(egui::ProgressBar::new(min_progress).text(format!(
            "{:.1}% of the 8 bits/byte maximum",
            min_progress * 100.0
        )));

        ui.add_space(5.0);

        Self::render_explanation_tooltip(
            ui,
            "Markov-1 Min-Entropy:",
            "Worst-case conditional entropy of the next byte given the previous \
             one. A deterministic counter has 8.0 bits of Shannon entropy but \
             almost 0.0 bits here.",
        );
        ui.label(format!("{:.4} bits/byte", metrics.markov_min_entropy));

        ui.add_space(5.0);

        Self::render_explanation_tooltip(
            ui,
            "Lag-1 Serial Correlation:",
            "Correlation between adjacent bytes. Random data stays near 0.0; \
             a value larger than 3/sqrt(sample size) is suspicious.",
        );
        ui.label(format!("{:.6}", metrics.serial_correlation));

        ui.add_space(10.0);

        ui.label(format!(
            "Mean byte value: {:.2} (ideal: 127.5)",
            metrics.mean
        ));
        ui.label(format!(
            "Chi-square: {:.2} (p = {:.6})",
            metrics.chi_square, metrics.chi_square_p_value
        ));
        ui.label(format!(
            "Longest run of identical bits: {} bits",
            metrics.longest_run
        ));
        ui.label(format!(
            "NIST tests failed: {} of {}",
            metrics.nist_failures, metrics.nist_tests_run
        ));

        if metrics.is_suspiciously_uniform {
            ui.add_space(5.0);
            ui.colored_label(
                egui::Color32::from_rgb(220, 160, 30),
                "⚠ Suspiciously uniform byte distribution (chi-square p > 0.9999). \
                 Deterministic counters and synthetic data often land here.",
            );
        }

        ui.add_space(10.0);

        let score = metrics.overall_score();
        ui.horizontal(|ui| {
            ui.label("Overall Quality Score:");
            ui.label(format!("{:.1}/100", score));
        });

        if metrics.is_capped() {
            ui.colored_label(
                egui::Color32::from_rgb(220, 160, 30),
                "⚠ Score capped at 30: the sample shows deterministic structure, \
                 strong serial correlation, or a majority of failing NIST tests.",
            );
        }
    }

    fn render_nist_section(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.heading("NIST SP 800-22 Tests");
            ui.label("ℹ").on_hover_text(
                "Statistical tests used to check whether a sequence is \
                 indistinguishable from a fair coin flip. P-values ≥ 0.01 pass.",
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
                    ui.label(format!("{:.6}", p_value));

                    if *p_value >= 0.01 {
                        ui.colored_label(egui::Color32::GREEN, "✓ Pass");
                    } else {
                        ui.colored_label(egui::Color32::RED, "✗ Fail");
                    }
                    ui.end_row();
                }
            });
    }

    /// Render the "Benchmark" tab.
    fn render_benchmark_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Performance Benchmark");
        ui.label("Measure entropy generation speed.");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            let label = ui.label("Benchmark size:");
            ui.add(
                egui::Slider::new(&mut self.bench_size, 10_000..=10_000_000)
                    .logarithmic(true)
                    .suffix(" bytes"),
            )
            .labelled_by(label.id);
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("⚡ Run Benchmark").clicked() && !self.is_benchmarking {
                self.start_bench_job();
            }

            if self.is_benchmarking {
                ui.spinner();
                ui.label("Benchmarking in background...");
                ui.ctx().request_repaint();
            }
        });

        ui.add_space(20.0);

        if let Some(result) = &self.bench_result {
            ui.heading("Results");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                Self::render_explanation_tooltip(
                    ui,
                    "Throughput:",
                    "How fast entropy can be generated. Important for bulk \
                     encryption such as file encryption or streaming.",
                );
                ui.heading(format!("{:.2} MB/s", result.throughput_mbps));
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                Self::render_explanation_tooltip(
                    ui,
                    "Latency per byte:",
                    "Time to generate a single byte. Important for real-time \
                     operations such as key generation and handshakes.",
                );
                ui.label(format!("{:.2} µs", result.latency_us));
            });

            ui.add_space(5.0);
            ui.label(format!(
                "Generated {} bytes in {:.3} seconds",
                result.bytes_generated,
                result.duration.as_secs_f64()
            ));
        } else {
            ui.label("Click 'Run Benchmark' to measure performance.");
        }
    }

    fn start_bench_job(&mut self) {
        self.is_benchmarking = true;
        let size = self.bench_size;
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            let mut source = SystemEntropy::new();
            let result = PerformanceBench::benchmark(&mut source, size);
            let _ = tx.send(result);
        });

        self.bench_rx = Some(rx);
    }

    /// Render the "Learn" tab.
    fn render_learn_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.learn_mode, LearnMode::XorCipher, "XOR Cipher");
            ui.selectable_value(
                &mut self.learn_mode,
                LearnMode::ShannonEntropy,
                "Shannon Entropy",
            );
            ui.selectable_value(
                &mut self.learn_mode,
                LearnMode::NistFrequency,
                "NIST Frequency",
            );
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
        let time = ui.input(|i| i.time);
        self.learn_process.update(time);
        if self.learn_process.is_playing {
            ui.ctx().request_repaint();
        }

        ui.heading("Learn Cryptography: XOR Cipher");
        ui.label("Visualize how a stream cipher XORs keystream bytes with UTF-8 bytes.");
        ui.add_space(10.0);

        ui.horizontal_wrapped(|ui| {
            let label = ui.label("Input:");
            ui.add(
                egui::TextEdit::singleline(&mut self.learn_input)
                    .desired_width((ui.available_width() - 160.0).max(120.0)),
            )
            .labelled_by(label.id);
            if ui.button("Start Visualization").clicked() {
                self.learn_process.start(&self.learn_input);
            }
        });

        ui.add_space(20.0);

        if let Some(step) = self.learn_process.current_step() {
            let total_steps = self.learn_process.steps.len();
            let current_idx = self.learn_process.current_step_index + 1;

            ui.heading(format!(
                "Step {} of {}: {}",
                current_idx,
                total_steps,
                step.character_label()
            ));
            ui.add_space(10.0);

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);

                    let input_color = egui::Color32::from_rgb(100, 200, 255);
                    let key_color = egui::Color32::from_rgb(255, 200, 100);
                    let result_color = egui::Color32::from_rgb(100, 255, 100);

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Input:").strong());
                        ui.add_space(20.0);
                        ui.label(step.character_label());
                        ui.label("=");
                        ui.label(step.input_byte.to_string());
                        ui.label("=");
                        ui.label(
                            egui::RichText::new(step.input_binary())
                                .color(input_color)
                                .font(egui::FontId::monospace(20.0)),
                        );
                    });

                    ui.add_space(5.0);
                    ui.label(egui::RichText::new("↓").strong());
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Key:  ").strong());
                        ui.add_space(20.0);
                        ui.label("keystream byte");
                        ui.label("=");
                        ui.label(step.keystream_byte.to_string());
                        ui.label("=");
                        ui.label(
                            egui::RichText::new(step.keystream_binary())
                                .color(key_color)
                                .font(egui::FontId::monospace(20.0)),
                        );
                    });

                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("XOR:  ").strong());
                        ui.add_space(20.0);
                        ui.label("result byte");
                        ui.label("=");
                        ui.label(step.result_byte.to_string());
                        ui.label("=");
                        ui.label(
                            egui::RichText::new(step.result_binary())
                                .color(result_color)
                                .font(egui::FontId::monospace(20.0)),
                        );
                    });

                    ui.add_space(10.0);
                    ui.label("0⊕1=1, 1⊕0=1, 0⊕0=0, 1⊕1=0");
                    ui.label("Different bits produce 1; identical bits produce 0.");
                    ui.add_space(10.0);
                });
            });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.button("⬅ Previous").clicked() {
                    self.learn_process.prev_step();
                }

                let play_label = if self.learn_process.is_playing {
                    "⏸ Pause"
                } else {
                    "▶ Play"
                };
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

            ui.group(|ui| {
                ui.heading("Encryption Progress");
                ui.horizontal_wrapped(|ui| {
                    for (i, step) in self.learn_process.steps.iter().enumerate() {
                        let text = format!("{:02X}", step.result_byte);
                        let mut rich_text = egui::RichText::new(text).monospace();

                        if i == self.learn_process.current_step_index {
                            rich_text = rich_text
                                .strong()
                                .color(egui::Color32::GREEN)
                                .background_color(egui::Color32::from_gray(60));
                        } else if i < self.learn_process.current_step_index {
                            rich_text = rich_text.color(egui::Color32::LIGHT_GRAY);
                        } else {
                            rich_text = rich_text.color(egui::Color32::DARK_GRAY);
                        }

                        ui.label(rich_text);
                    }
                });
            });

            ui.add_space(20.0);
            ui.collapsing("How it works", |ui| {
                ui.label("1. Take the UTF-8 bytes of the text, not the characters.");
                ui.label(
                    "2. Example: 'H' is one byte (0x48), but 'ş' is two bytes \
                     (0xC5 0x9F) and 🔐 is four bytes.",
                );
                ui.label("3. Draw one random keystream byte per input byte.");
                ui.label("4. XOR each input byte with its keystream byte.");
                ui.label(
                    "5. XOR is reversible only when the same keystream is reused. \
                     Never reuse a keystream in real encryption.",
                );
            });
        } else {
            ui.label("Enter text and click 'Start Visualization' to begin.");
        }
    }

    fn render_entropy_visualizer(&mut self, ui: &mut egui::Ui) {
        use crate::learn::entropy_visual::EntropyStepType;

        let time = ui.input(|i| i.time);
        self.entropy_process.update(time);
        if self.entropy_process.is_playing {
            ui.ctx().request_repaint();
        }

        ui.heading("Learn Shannon Entropy");
        ui.label("Measures the unpredictability of the UTF-8 bytes in the input.");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            let label = ui.label("Input:");
            ui.text_edit_singleline(&mut self.entropy_process.input)
                .labelled_by(label.id);
            if ui.button("Calculate").clicked() {
                let input = self.entropy_process.input.clone();
                self.entropy_process.start(&input);
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

            ui.heading(format!(
                "Step {} of {}: {}",
                current_idx, total_steps, step_title
            ));

            ui.add_space(10.0);

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(10.0);

                    if !step.byte_counts.is_empty() {
                        egui::Grid::new("entropy_grid")
                            .striped(true)
                            .spacing([20.0, 5.0])
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("Byte").strong());
                                ui.label(egui::RichText::new("Count").strong());
                                if step.step_type != EntropyStepType::CountBytes {
                                    ui.label(egui::RichText::new("Probability").strong());
                                    ui.label(egui::RichText::new("Contribution").strong());
                                }
                                ui.label(egui::RichText::new("Visual").strong());
                                ui.end_row();

                                let mut bytes: Vec<_> = step.byte_counts.keys().collect();
                                bytes.sort();

                                let max_count =
                                    step.byte_counts.values().max().copied().unwrap_or(1).max(1)
                                        as f32;

                                for &byte in bytes {
                                    let count = step.byte_counts[&byte];
                                    let char_repr = if (32..=126).contains(&byte) {
                                        (byte as char).to_string()
                                    } else {
                                        format!("0x{:02X}", byte)
                                    };

                                    ui.label(format!("'{}'", char_repr));
                                    ui.label(count.to_string());

                                    if step.step_type != EntropyStepType::CountBytes {
                                        let p = step.probabilities.get(&byte).unwrap_or(&0.0);
                                        let contribution =
                                            step.entropy_contributions.get(&byte).unwrap_or(&0.0);

                                        ui.label(format!("{:.3}", p));

                                        if matches!(
                                            step.step_type,
                                            EntropyStepType::CalculateContributions
                                                | EntropyStepType::SumEntropy
                                                | EntropyStepType::Interpret
                                        ) {
                                            ui.label(format!("{:.3} bits", contribution));
                                        } else {
                                            ui.label("-");
                                        }
                                    }

                                    let width = (count as f32 / max_count) * 100.0;
                                    let color = if count == 1 {
                                        egui::Color32::GREEN
                                    } else {
                                        egui::Color32::from_rgb(
                                            255,
                                            100 + (155 - (count * 20).min(155)) as u8,
                                            100,
                                        )
                                    };
                                    let (rect, _) = ui.allocate_exact_size(
                                        egui::vec2(width, 15.0),
                                        egui::Sense::hover(),
                                    );
                                    ui.painter().rect_filled(rect, 2.0, color);
                                    ui.end_row();
                                }
                            });
                    }

                    ui.add_space(20.0);

                    match step.step_type {
                        EntropyStepType::CountBytes => {
                            ui.label(
                                "We count how many times each unique byte appears in the input.",
                            );
                        }
                        EntropyStepType::CalculateProbabilities => {
                            ui.label("Probability P(x) = Count(x) / Total Bytes");
                            ui.label("This tells us how likely each byte is to appear.");
                        }
                        EntropyStepType::CalculateContributions => {
                            ui.label("Contribution = -P(x) * log2(P(x))");
                            ui.label(
                                "Rare events provide more information per occurrence, but \
                                 they also mean the distribution is less uniform.",
                            );
                        }
                        EntropyStepType::SumEntropy | EntropyStepType::Interpret => {
                            let absolute_efficiency = if step.max_entropy > 0.0 {
                                (step.current_entropy_sum / step.max_entropy) * 100.0
                            } else {
                                0.0
                            };
                            let alphabet_efficiency = if step.alphabet_max_entropy > 0.0 {
                                (step.current_entropy_sum / step.alphabet_max_entropy) * 100.0
                            } else {
                                0.0
                            };

                            ui.heading(format!(
                                "Total Entropy: {:.4} bits/byte",
                                step.current_entropy_sum
                            ));
                            ui.label(format!(
                                "Absolute maximum for bytes: {:.2} bits/byte",
                                step.max_entropy
                            ));
                            ui.label(format!("Absolute efficiency: {:.1}%", absolute_efficiency));
                            ui.label(format!(
                                "Observed alphabet maximum: {:.3} bits/byte ({} distinct bytes)",
                                step.alphabet_max_entropy,
                                step.byte_counts.len()
                            ));
                            ui.label(format!(
                                "Observed-alphabet efficiency: {:.1}%",
                                alphabet_efficiency
                            ));

                            if step.step_type == EntropyStepType::Interpret {
                                ui.separator();
                                match interpret_entropy(step.current_entropy_sum) {
                                    crate::learn::entropy_visual::EntropyVerdict::Low => {
                                        ui.label("Low entropy: the input is very predictable.");
                                    }
                                    crate::learn::entropy_visual::EntropyVerdict::Moderate => {
                                        ui.label("Moderate entropy.");
                                    }
                                    crate::learn::entropy_visual::EntropyVerdict::High => {
                                        ui.label("High entropy on the absolute 8 bits/byte scale.");
                                    }
                                }
                                ui.label(
                                    "Note: a two-character input such as 'AB' is 100% \
                                     efficient for its tiny alphabet, but only 1 bit/byte \
                                     on the absolute scale.",
                                );
                            }
                        }
                    }
                    ui.add_space(10.0);
                });
            });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.button("⬅ Previous").clicked() {
                    self.entropy_process.prev_step();
                }

                let play_label = if self.entropy_process.is_playing {
                    "⏸ Pause"
                } else {
                    "▶ Play"
                };
                if ui.button(play_label).clicked() {
                    self.entropy_process.toggle_play();
                }

                if ui.button("Next ➡").clicked() {
                    self.entropy_process.next_step();
                }

                ui.add_space(20.0);
                ui.label("Speed:");
                ui.add(
                    egui::Slider::new(&mut self.entropy_process.speed, 0.1..=5.0).text("steps/s"),
                );
            });
        } else {
            ui.label("Enter text (e.g., 'AAABBC') and click Calculate.");
        }
    }

    fn render_nist_visualizer(&mut self, ui: &mut egui::Ui) {
        use crate::learn::nist_visual::NistStepType;

        let time = ui.input(|i| i.time);
        self.nist_process.update(time);
        if self.nist_process.is_playing {
            ui.ctx().request_repaint();
        }

        ui.heading("Learn NIST Frequency Test");
        ui.label("Checks whether the number of 1s and 0s is approximately equal.");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            let label = ui.label("Input:");
            ui.text_edit_singleline(&mut self.nist_process.input_text)
                .labelled_by(label.id);
            if ui.button("Analyze").clicked() {
                let input = self.nist_process.input_text.clone();
                self.nist_process.start(&input);
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

            ui.heading(format!(
                "Step {} of {}: {}",
                current_idx, total_steps, step_title
            ));
            ui.add_space(10.0);

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(10.0);

                    ui.label(egui::RichText::new("Bit Sequence:").strong());
                    ui.horizontal_wrapped(|ui| {
                        for (i, &bit) in step.bits.iter().enumerate() {
                            if i > 0 && i % 8 == 0 {
                                ui.add_space(5.0);
                            }
                            let color = if bit == 1 {
                                egui::Color32::GREEN
                            } else {
                                egui::Color32::LIGHT_GRAY
                            };
                            let text = egui::RichText::new(bit.to_string())
                                .color(color)
                                .monospace()
                                .strong();
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
                                ui.heading(step.ones_count.to_string());
                            });
                            ui.add_space(40.0);
                            ui.vertical(|ui| {
                                ui.label("Zeros:");
                                ui.heading(step.zeros_count.to_string());
                            });
                        });

                        let total = step.bits.len() as f32;
                        if total > 0.0 {
                            let ones_pct = step.ones_count as f32 / total;
                            ui.add_space(5.0);
                            let (rect, _) = ui
                                .allocate_exact_size(egui::vec2(300.0, 20.0), egui::Sense::hover());

                            ui.painter()
                                .rect_filled(rect, 2.0, egui::Color32::LIGHT_GRAY);

                            let ones_width = rect.width() * ones_pct;
                            let ones_rect = egui::Rect::from_min_size(
                                rect.min,
                                egui::vec2(ones_width, rect.height()),
                            );
                            ui.painter()
                                .rect_filled(ones_rect, 2.0, egui::Color32::GREEN);

                            let center_x = rect.min.x + rect.width() * 0.5;
                            ui.painter().line_segment(
                                [
                                    egui::pos2(center_x, rect.min.y),
                                    egui::pos2(center_x, rect.max.y),
                                ],
                                egui::Stroke::new(2.0, egui::Color32::BLACK),
                            );

                            ui.label(format!("Ratio: {:.1}% Ones (Ideal: 50%)", ones_pct * 100.0));
                        }
                    }

                    ui.add_space(20.0);

                    if matches!(
                        step.step_type,
                        NistStepType::CalculateStatistic
                            | NistStepType::CalculatePValue
                            | NistStepType::Interpret
                    ) {
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.label(format!("Sum (+1 for 1, -1 for 0): {}", step.sum));
                                ui.label(format!("S_obs (|Sum| / √n): {:.4}", step.s_obs));

                                if matches!(
                                    step.step_type,
                                    NistStepType::CalculatePValue | NistStepType::Interpret
                                ) {
                                    ui.add_space(5.0);
                                    ui.label(format!(
                                        "P-Value (erfc(S_obs/√2)): {:.4}",
                                        step.p_value
                                    ));
                                }

                                if step.step_type == NistStepType::Interpret {
                                    ui.add_space(10.0);
                                    if step.passed {
                                        ui.colored_label(
                                            egui::Color32::GREEN,
                                            "✅ PASS: the sequence looks random.",
                                        );
                                    } else {
                                        ui.colored_label(
                                            egui::Color32::RED,
                                            "❌ FAIL: the sequence has too many 1s or 0s.",
                                        );
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

            ui.horizontal(|ui| {
                if ui.button("⬅ Previous").clicked() {
                    self.nist_process.prev_step();
                }

                let play_label = if self.nist_process.is_playing {
                    "⏸ Pause"
                } else {
                    "▶ Play"
                };
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

/// Consistent separator between logical sections of a screen.
fn section_break(ui: &mut egui::Ui) {
    ui.add_space(SECTION_GAP);
    ui.separator();
    ui.add_space(SECTION_GAP);
}

/// Parse a 96-bit nonce from a hex string.
fn parse_nonce(input: &str) -> Result<[u8; 12], String> {
    let trimmed = input.trim();
    let bytes = hex::decode(trimmed)
        .map_err(|_| "Nonce must be a valid hexadecimal string.".to_string())?;

    if bytes.len() != 12 {
        return Err(format!(
            "Nonce must be exactly 24 hex characters (96 bits); got {} bytes.",
            bytes.len()
        ));
    }

    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&bytes);
    Ok(nonce)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonce_parser_accepts_96_bits() {
        let hex = "000102030405060708090a0b";
        let nonce = parse_nonce(hex).unwrap();
        assert_eq!(nonce, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    }

    #[test]
    fn nonce_parser_rejects_wrong_length_and_bad_hex() {
        assert!(parse_nonce("00").is_err());
        assert!(parse_nonce("zz").is_err());
    }

    #[test]
    fn decrypt_displays_plaintext_even_when_hex_toggle_is_on() {
        // Regression: the hex toggle used to hex-encode decrypted plaintext,
        // which made a successful round-trip look like it had failed.
        let nonce = [9u8; 12];
        let plaintext = "Merhaba dünya! 🔐";
        let ciphertext =
            KeyedStreamCipher::process_with_passphrase("pw", &nonce, plaintext.as_bytes());
        let decrypted = KeyedStreamCipher::process_with_passphrase("pw", &nonce, &ciphertext);

        let (display, notice) =
            EntropyForgeApp::format_cipher_output(CipherMode::Decrypt, true, &decrypted);

        assert_eq!(display, plaintext);
        assert!(notice.is_none());
    }

    #[test]
    fn encrypt_with_hex_toggle_shows_hex_ciphertext() {
        let ciphertext = [0xDE, 0xAD, 0xBE, 0xEF];
        let (display, notice) =
            EntropyForgeApp::format_cipher_output(CipherMode::Encrypt, true, &ciphertext);

        assert_eq!(display, hex::encode(ciphertext));
        assert!(notice.is_none());
    }

    #[test]
    fn encrypt_decrypt_round_trip_through_displayed_output() {
        let nonce = [3u8; 12];
        let plaintext = "round trip";
        let ciphertext =
            KeyedStreamCipher::process_with_passphrase("pw", &nonce, plaintext.as_bytes());

        let (hex_display, _) =
            EntropyForgeApp::format_cipher_output(CipherMode::Encrypt, true, &ciphertext);
        let decoded = hex::decode(hex_display).unwrap();
        let decrypted = KeyedStreamCipher::process_with_passphrase("pw", &nonce, &decoded);
        let (plain_display, _) =
            EntropyForgeApp::format_cipher_output(CipherMode::Decrypt, true, &decrypted);

        assert_eq!(plain_display, plaintext);
    }

    #[test]
    fn invalid_utf8_output_produces_a_warning() {
        let (display, notice) =
            EntropyForgeApp::format_cipher_output(CipherMode::Decrypt, true, &[0xFF, 0xFE]);

        assert_eq!(display, "\u{FFFD}\u{FFFD}");
        assert!(
            notice.unwrap().contains("not valid UTF-8"),
            "wrong key/nonce or hex input should be mentioned"
        );
    }

    #[test]
    fn binary_ciphertext_without_hex_warns_about_lossy_text() {
        let (_, notice) =
            EntropyForgeApp::format_cipher_output(CipherMode::Encrypt, false, &[0xFF, 0xFE]);

        assert!(notice.unwrap().contains("Ciphertext is binary"));
    }

    #[test]
    fn ui_round_trip_encrypt_then_decrypt_restores_plaintext() {
        let mut app = EntropyForgeApp {
            passphrase: "test-passphrase".to_string(),
            nonce_hex: "000102030405060708090a0b".to_string(),
            cipher_input: "Hello, World!".to_string(),
            cipher_hex: true,
            ..EntropyForgeApp::default()
        };

        app.run_cipher(CipherMode::Encrypt);
        assert!(app.cipher_error.is_none());
        let ciphertext_hex = app.cipher_output.clone();
        assert_eq!(ciphertext_hex, hex::encode(&app.cipher_output_bytes));

        app.use_output_as_input();
        assert_eq!(app.cipher_mode, CipherMode::Decrypt);
        assert_eq!(app.cipher_input, ciphertext_hex);
        assert!(app.cipher_hex);

        app.run_cipher(CipherMode::Decrypt);
        assert!(app.cipher_error.is_none());
        assert_eq!(app.cipher_output, "Hello, World!");
        assert_eq!(app.cipher_output_bytes, b"Hello, World!");
    }

    #[test]
    fn decrypt_rejects_invalid_hex_instead_of_falling_back() {
        let mut app = EntropyForgeApp {
            nonce_hex: "000102030405060708090a0b".to_string(),
            cipher_hex: true,
            cipher_input: "not hex".to_string(),
            ..EntropyForgeApp::default()
        };

        app.run_cipher(CipherMode::Decrypt);

        assert!(app.cipher_error.is_some());
        assert!(app.cipher_output_bytes.is_empty());
    }

    #[test]
    fn toggling_hex_reformats_existing_output() {
        let mut app = EntropyForgeApp {
            passphrase: "pw".to_string(),
            nonce_hex: "000102030405060708090a0b".to_string(),
            cipher_input: "abc".to_string(),
            cipher_hex: true,
            ..EntropyForgeApp::default()
        };

        app.run_cipher(CipherMode::Encrypt);
        let hex_output = app.cipher_output.clone();

        app.cipher_hex = false;
        app.refresh_cipher_output();
        assert_ne!(app.cipher_output, hex_output);

        app.cipher_hex = true;
        app.refresh_cipher_output();
        assert_eq!(app.cipher_output, hex_output);
    }
}
