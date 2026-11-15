//! Step-by-step Shannon entropy visualization.
//!
//! Two different notions of "efficiency" are kept separate:
//!
//! * **Cryptographic byte entropy**: `H(X) / 8.0 bits per byte`.
//! * **Observed-alphabet efficiency**: `H(X) / log2(|observed alphabet|)`.
//!
//! A two-character input such as `"AB"` has 100% observed-alphabet
//! efficiency, but only `1.0 / 8.0 = 12.5%` cryptographic byte entropy. The
//! UI must not call `"AB"` "high entropy".

use std::collections::HashMap;

/// Represents a step in the Shannon entropy calculation.
#[derive(Debug, Clone, PartialEq)]
pub enum EntropyStepType {
    CountBytes,
    CalculateProbabilities,
    CalculateContributions,
    SumEntropy,
    Interpret,
}

#[derive(Debug, Clone)]
pub struct EntropyStep {
    pub step_type: EntropyStepType,
    pub byte_counts: HashMap<u8, usize>,
    pub probabilities: HashMap<u8, f64>,
    pub entropy_contributions: HashMap<u8, f64>,
    pub current_entropy_sum: f64,
    pub total_entropy: f64,
    /// Theoretical maximum in bits per byte: always 8.0 for a byte alphabet.
    pub max_entropy: f64,
    /// `log2(number of distinct bytes observed in this input)`.
    pub alphabet_max_entropy: f64,
}

/// Manages the state of the Shannon entropy visualization.
pub struct EntropyProcess {
    pub input: String,
    pub steps: Vec<EntropyStep>,
    pub current_step_index: usize,
    pub is_playing: bool,
    pub speed: f32,
    pub last_update: f64,
}

impl Default for EntropyProcess {
    fn default() -> Self {
        Self {
            input: String::new(),
            steps: Vec::new(),
            current_step_index: 0,
            is_playing: false,
            speed: 1.0,
            last_update: 0.0,
        }
    }
}

impl EntropyProcess {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, text: &str) {
        self.input = text.to_string();
        self.steps.clear();
        self.current_step_index = 0;
        self.is_playing = false;

        let data = text.as_bytes();
        if data.is_empty() {
            return;
        }

        // --- Step 1: Count bytes ---
        let mut byte_counts = HashMap::new();
        for &byte in data {
            *byte_counts.entry(byte).or_insert(0) += 1;
        }

        self.steps.push(EntropyStep {
            step_type: EntropyStepType::CountBytes,
            byte_counts: byte_counts.clone(),
            probabilities: HashMap::new(),
            entropy_contributions: HashMap::new(),
            current_entropy_sum: 0.0,
            total_entropy: 0.0,
            max_entropy: 8.0,
            alphabet_max_entropy: 0.0,
        });

        // --- Step 2: Probabilities ---
        let total_bytes = data.len() as f64;
        let mut probabilities = HashMap::new();
        for (&byte, &count) in &byte_counts {
            probabilities.insert(byte, count as f64 / total_bytes);
        }

        let alphabet_max_entropy = if probabilities.is_empty() {
            0.0
        } else {
            (probabilities.len() as f64).log2()
        };

        self.steps.push(EntropyStep {
            step_type: EntropyStepType::CalculateProbabilities,
            byte_counts: byte_counts.clone(),
            probabilities: probabilities.clone(),
            entropy_contributions: HashMap::new(),
            current_entropy_sum: 0.0,
            total_entropy: 0.0,
            max_entropy: 8.0,
            alphabet_max_entropy,
        });

        // --- Step 3: Contributions ---
        let mut entropy_contributions = HashMap::new();
        for (&byte, &p) in &probabilities {
            entropy_contributions.insert(byte, if p > 0.0 { -p * p.log2() } else { 0.0 });
        }

        self.steps.push(EntropyStep {
            step_type: EntropyStepType::CalculateContributions,
            byte_counts: byte_counts.clone(),
            probabilities: probabilities.clone(),
            entropy_contributions: entropy_contributions.clone(),
            current_entropy_sum: 0.0,
            total_entropy: 0.0,
            max_entropy: 8.0,
            alphabet_max_entropy,
        });

        // --- Step 4: Sum ---
        let total_entropy: f64 = entropy_contributions.values().sum();

        self.steps.push(EntropyStep {
            step_type: EntropyStepType::SumEntropy,
            byte_counts: byte_counts.clone(),
            probabilities: probabilities.clone(),
            entropy_contributions: entropy_contributions.clone(),
            current_entropy_sum: total_entropy,
            total_entropy,
            max_entropy: 8.0,
            alphabet_max_entropy,
        });

        // --- Step 5: Interpret ---
        self.steps.push(EntropyStep {
            step_type: EntropyStepType::Interpret,
            byte_counts,
            probabilities,
            entropy_contributions,
            current_entropy_sum: total_entropy,
            total_entropy,
            max_entropy: 8.0,
            alphabet_max_entropy,
        });
    }

    pub fn current_step(&self) -> Option<&EntropyStep> {
        self.steps.get(self.current_step_index)
    }

    pub fn next_step(&mut self) {
        if self.current_step_index + 1 < self.steps.len() {
            self.current_step_index += 1;
        } else {
            self.is_playing = false;
        }
    }

    pub fn prev_step(&mut self) {
        if self.current_step_index > 0 {
            self.current_step_index -= 1;
        }
    }

    pub fn toggle_play(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn update(&mut self, time: f64) {
        if self.is_playing {
            let interval = 1.0 / self.speed.max(0.1) as f64;
            if time - self.last_update > interval {
                self.next_step();
                self.last_update = time;
            }
        } else {
            self.last_update = time;
        }
    }
}

/// Interpretation of an entropy result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntropyVerdict {
    Low,
    Moderate,
    High,
}

/// Classify entropy using the **absolute** 8 bits/byte scale.
pub fn interpret_entropy(entropy_bits_per_byte: f64) -> EntropyVerdict {
    if entropy_bits_per_byte < 2.0 {
        EntropyVerdict::Low
    } else if entropy_bits_per_byte < 6.0 {
        EntropyVerdict::Moderate
    } else {
        EntropyVerdict::High
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_input_has_low_absolute_entropy() {
        let mut process = EntropyProcess::new();
        process.start("AB");

        let last = process.steps.last().unwrap();
        assert!(last.max_entropy == 8.0);
        assert!((last.alphabet_max_entropy - 1.0).abs() < 1e-9);
        assert!((last.current_entropy_sum - 1.0).abs() < 1e-9);
        assert_eq!(
            interpret_entropy(last.current_entropy_sum),
            EntropyVerdict::Low
        );
    }

    #[test]
    fn utf8_multibyte_alphabet_is_measured_by_bytes() {
        let text: String = (0u8..=255).map(char::from).collect();
        let mut process = EntropyProcess::new();
        process.start(&text);

        let last = process.steps.last().unwrap();

        let mut counts = HashMap::new();
        for &byte in text.as_bytes() {
            *counts.entry(byte).or_insert(0usize) += 1;
        }
        let total = text.len() as f64;
        let expected: f64 = counts
            .values()
            .map(|&count| {
                let p = count as f64 / total;
                -p * p.log2()
            })
            .sum();

        assert_eq!(last.max_entropy, 8.0);
        assert!((last.current_entropy_sum - expected).abs() < 1e-9);
        assert!((last.alphabet_max_entropy - (counts.len() as f64).log2()).abs() < 1e-9);
    }

    #[test]
    fn multibyte_input_is_measured_in_utf8_bytes() {
        let mut process = EntropyProcess::new();
        process.start("şğİ");
        let first = &process.steps[0];
        assert_eq!(first.byte_counts.values().sum::<usize>(), 6);
    }
}
