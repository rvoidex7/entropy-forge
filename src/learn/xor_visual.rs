use super::steps::EncryptionStep;
use crate::entropy::{EntropySource, SystemEntropy};

/// Manages the state of the learning visualization
pub struct EncryptionProcess {
    pub input_text: String,
    pub steps: Vec<EncryptionStep>,
    pub current_step_index: usize,
    pub is_playing: bool,
    pub speed: f32, // Steps per second (approximate)
    pub last_update: f64, // Timestamp for animation timing
}

impl Default for EncryptionProcess {
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

impl EncryptionProcess {
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes the visualization with the given text
    pub fn start(&mut self, text: &str) {
        self.input_text = text.to_string();
        self.steps.clear();
        self.current_step_index = 0;
        self.is_playing = false;

        let mut entropy = SystemEntropy::new();
        let mut keystream = vec![0u8; text.len()];
        entropy.fill_bytes(&mut keystream);

        for (i, char) in text.chars().enumerate() {
            if i < keystream.len() {
                self.steps.push(EncryptionStep::new(char, keystream[i]));
            }
        }
    }

    pub fn current_step(&self) -> Option<&EncryptionStep> {
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
            self.is_playing = false; // Stop at end
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

    /// Update animation state based on time delta
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
