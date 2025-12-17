use std::collections::HashMap;

/// Represents a step in the entropy calculation process
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
    pub max_entropy: f64,
}

/// Manages the state of the Shannon entropy visualization
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

        // --- Step 1: Count Bytes ---
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
            max_entropy: 0.0,
        });

        // --- Step 2: Calculate Probabilities ---
        let total_bytes = data.len() as f64;
        let mut probabilities = HashMap::new();
        for (&byte, &count) in &byte_counts {
            probabilities.insert(byte, count as f64 / total_bytes);
        }

        self.steps.push(EntropyStep {
            step_type: EntropyStepType::CalculateProbabilities,
            byte_counts: byte_counts.clone(),
            probabilities: probabilities.clone(),
            entropy_contributions: HashMap::new(),
            current_entropy_sum: 0.0,
            total_entropy: 0.0,
            max_entropy: 0.0,
        });

        // --- Step 3: Calculate Contributions ---
        let mut entropy_contributions = HashMap::new();
        for (&byte, &p) in &probabilities {
            if p > 0.0 {
                entropy_contributions.insert(byte, -p * p.log2());
            } else {
                entropy_contributions.insert(byte, 0.0);
            }
        }

        self.steps.push(EntropyStep {
            step_type: EntropyStepType::CalculateContributions,
            byte_counts: byte_counts.clone(),
            probabilities: probabilities.clone(),
            entropy_contributions: entropy_contributions.clone(),
            current_entropy_sum: 0.0,
            total_entropy: 0.0,
            max_entropy: 0.0,
        });

        // --- Step 4: Sum Entropy ---
        let total_entropy: f64 = entropy_contributions.values().sum();
        let max_possible = if !probabilities.is_empty() {
            (probabilities.len() as f64).log2()
        } else {
            0.0
        };

        self.steps.push(EntropyStep {
            step_type: EntropyStepType::SumEntropy,
            byte_counts: byte_counts.clone(),
            probabilities: probabilities.clone(),
            entropy_contributions: entropy_contributions.clone(),
            current_entropy_sum: total_entropy,
            total_entropy,
            max_entropy: max_possible,
        });

        // --- Step 5: Interpretation ---
        self.steps.push(EntropyStep {
            step_type: EntropyStepType::Interpret,
            byte_counts: byte_counts,
            probabilities: probabilities,
            entropy_contributions: entropy_contributions,
            current_entropy_sum: total_entropy,
            total_entropy,
            max_entropy: max_possible,
        });
    }

    pub fn current_step(&self) -> Option<&EntropyStep> {
        if self.steps.is_empty() {
            None
        } else {
            Some(&self.steps[self.current_step_index])
        }
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
            if time - self.last_update > (1.0 / self.speed as f64) {
                self.next_step();
                self.last_update = time;
            }
        } else {
            self.last_update = time;
        }
    }
}
