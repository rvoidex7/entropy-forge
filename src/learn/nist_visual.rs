use crate::quality::NistTests;

#[derive(Debug, Clone, PartialEq)]
pub enum NistStepType {
    ConvertToBits,
    CountOnesZeros,
    CalculateStatistic,
    CalculatePValue,
    Interpret,
}

#[derive(Debug, Clone)]
pub struct NistFrequencyStep {
    pub step_type: NistStepType,
    pub bits: Vec<u8>, // 0 or 1
    pub ones_count: usize,
    pub zeros_count: usize,
    pub sum: i64,
    pub s_obs: f64,
    pub p_value: f64,
    pub passed: bool,
}

pub struct NistProcess {
    pub input_text: String,
    pub steps: Vec<NistFrequencyStep>,
    pub current_step_index: usize,
    pub is_playing: bool,
    pub speed: f32,
    pub last_update: f64,
}

impl Default for NistProcess {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            steps: Vec::new(),
            current_step_index: 0,
            is_playing: false,
            speed: 1.0,
            last_update: 0.0,
        }
    }
}


impl NistProcess {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, text: &str) {
        self.input_text = text.to_string();
        self.steps.clear();
        self.current_step_index = 0;
        self.is_playing = false;

        let data = text.as_bytes();
        if data.is_empty() {
            return;
        }

        // --- Step 1: Convert to Bits ---
        let mut bits = Vec::new();
        for &byte in data {
            for i in (0..8).rev() {
                bits.push((byte >> i) & 1);
            }
        }

        self.steps.push(NistFrequencyStep {
            step_type: NistStepType::ConvertToBits,
            bits: bits.clone(),
            ones_count: 0,
            zeros_count: 0,
            sum: 0,
            s_obs: 0.0,
            p_value: 0.0,
            passed: false,
        });

        // --- Step 2: Count Ones and Zeros ---
        let ones_count = bits.iter().filter(|&&b| b == 1).count();
        let zeros_count = bits.len() - ones_count;

        self.steps.push(NistFrequencyStep {
            step_type: NistStepType::CountOnesZeros,
            bits: bits.clone(),
            ones_count,
            zeros_count,
            sum: 0,
            s_obs: 0.0,
            p_value: 0.0,
            passed: false,
        });

        // --- Step 3: Calculate Statistic ---
        // Sum = (+1 * ones) + (-1 * zeros)
        let sum = (ones_count as i64) - (zeros_count as i64);
        let n = bits.len() as f64;
        let s_obs = (sum as f64).abs() / n.sqrt();

        self.steps.push(NistFrequencyStep {
            step_type: NistStepType::CalculateStatistic,
            bits: bits.clone(),
            ones_count,
            zeros_count,
            sum,
            s_obs,
            p_value: 0.0,
            passed: false,
        });

        // --- Step 4: Calculate P-Value ---
        // p_value = erfc(S_obs / sqrt(2))
        let p_value = NistTests::erfc(s_obs / std::f64::consts::SQRT_2);

        self.steps.push(NistFrequencyStep {
            step_type: NistStepType::CalculatePValue,
            bits: bits.clone(),
            ones_count,
            zeros_count,
            sum,
            s_obs,
            p_value,
            passed: false, // Not decided yet visually, but mathematically yes
        });

        // --- Step 5: Interpretation ---
        let passed = p_value >= 0.01;
        self.steps.push(NistFrequencyStep {
            step_type: NistStepType::Interpret,
            bits: bits.clone(),
            ones_count,
            zeros_count,
            sum,
            s_obs,
            p_value,
            passed,
        });
    }

    pub fn generate_random(&mut self, count: usize) {
        use crate::entropy::{EntropySource, SystemEntropy};
        let mut entropy = SystemEntropy::new();
        let mut data = vec![0u8; count];
        entropy.fill_bytes(&mut data);

        // Compromise: Generate a random string of alphanumeric characters.
        // It's easy to understand and has high entropy.

        // Re-using SystemEntropy is better as I already have it.
        // SystemEntropy fills bytes. I can map them to chars.

        let chars: String = data.iter()
            .map(|&b| {
                // map to printable ascii range 33-126
                let c = 33 + (b % (126 - 33));
                c as char
            })
            .collect();

        self.start(&chars);
    }

    pub fn current_step(&self) -> Option<&NistFrequencyStep> {
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
