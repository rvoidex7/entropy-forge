use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use entropy_forge::learn::{EncryptionProcess, EntropyProcess, NistProcess};

#[derive(Serialize, Deserialize)]
pub struct SerializedBitOp {
    pub input_bit: bool,
    pub key_bit: bool,
    pub result_bit: bool,
    pub position: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedXorStep {
    pub character: String,
    pub input_byte: u8,
    pub keystream_byte: u8,
    pub result_byte: u8,
    pub input_binary: String,
    pub keystream_binary: String,
    pub result_binary: String,
    pub bit_ops: Vec<SerializedBitOp>,
}

#[derive(Serialize, Deserialize)]
pub struct XorStepsResponse {
    pub steps: Vec<SerializedXorStep>,
    pub total: usize,
}

#[tauri::command]
pub fn get_xor_steps(text: String) -> XorStepsResponse {
    let mut process = EncryptionProcess::new();
    process.start(&text);

    let mut serialized_steps = Vec::new();
    for step in process.steps {
        let mut bit_ops = Vec::new();
        for op in step.bit_ops {
            bit_ops.push(SerializedBitOp {
                input_bit: op.input_bit,
                key_bit: op.key_bit,
                result_bit: op.result_bit,
                position: op.position,
            });
        }

        let input_binary = format!("{:08b}", step.input_byte);
        let keystream_binary = format!("{:08b}", step.keystream_byte);
        let result_binary = format!("{:08b}", step.result_byte);

        serialized_steps.push(SerializedXorStep {
            character: step.character.to_string(),
            input_byte: step.input_byte,
            keystream_byte: step.keystream_byte,
            result_byte: step.result_byte,
            input_binary,
            keystream_binary,
            result_binary,
            bit_ops,
        });
    }

    XorStepsResponse {
        total: serialized_steps.len(),
        steps: serialized_steps,
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializedEntropyStep {
    pub step_type: String,
    pub byte_counts: HashMap<String, usize>,
    pub probabilities: HashMap<String, f64>,
    pub entropy_contributions: HashMap<String, f64>,
    pub current_entropy_sum: f64,
    pub total_entropy: f64,
    pub max_entropy: f64,
}

#[derive(Serialize, Deserialize)]
pub struct EntropyStepsResponse {
    pub steps: Vec<SerializedEntropyStep>,
    pub total: usize,
}

#[tauri::command]
pub fn get_entropy_steps(text: String) -> EntropyStepsResponse {
    let mut process = EntropyProcess::new();
    process.start(&text);

    let mut serialized_steps = Vec::new();
    for step in process.steps {
        let step_type = format!("{:?}", step.step_type);

        let byte_counts = step.byte_counts.into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        let probabilities = step.probabilities.into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        let entropy_contributions = step.entropy_contributions.into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        serialized_steps.push(SerializedEntropyStep {
            step_type,
            byte_counts,
            probabilities,
            entropy_contributions,
            current_entropy_sum: step.current_entropy_sum,
            total_entropy: step.total_entropy,
            max_entropy: step.max_entropy,
        });
    }

    EntropyStepsResponse {
        total: serialized_steps.len(),
        steps: serialized_steps,
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializedNistStep {
    pub step_type: String,
    pub bits: Vec<u8>,
    pub ones_count: usize,
    pub zeros_count: usize,
    pub sum: i64,
    pub s_obs: f64,
    pub p_value: f64,
    pub passed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NistStepsResponse {
    pub steps: Vec<SerializedNistStep>,
    pub total: usize,
    pub input_text: String,
}

#[tauri::command]
pub fn get_nist_steps(text: String) -> NistStepsResponse {
    let mut process = NistProcess::new();
    process.start(&text);

    let mut serialized_steps = Vec::new();
    for step in process.steps {
        serialized_steps.push(SerializedNistStep {
            step_type: format!("{:?}", step.step_type),
            bits: step.bits,
            ones_count: step.ones_count,
            zeros_count: step.zeros_count,
            sum: step.sum,
            s_obs: step.s_obs,
            p_value: step.p_value,
            passed: step.passed,
        });
    }

    NistStepsResponse {
        total: serialized_steps.len(),
        steps: serialized_steps,
        input_text: process.input_text,
    }
}

#[tauri::command]
pub fn get_nist_steps_random(count: usize) -> NistStepsResponse {
    let mut process = NistProcess::new();
    process.generate_random(count);

    let mut serialized_steps = Vec::new();
    for step in process.steps {
        serialized_steps.push(SerializedNistStep {
            step_type: format!("{:?}", step.step_type),
            bits: step.bits,
            ones_count: step.ones_count,
            zeros_count: step.zeros_count,
            sum: step.sum,
            s_obs: step.s_obs,
            p_value: step.p_value,
            passed: step.passed,
        });
    }

    NistStepsResponse {
        total: serialized_steps.len(),
        steps: serialized_steps,
        input_text: process.input_text,
    }
}
