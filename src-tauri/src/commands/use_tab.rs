use serde::{Serialize, Deserialize};
use entropy_forge::crypto::StreamCipher;
use entropy_forge::entropy::SystemEntropy;

#[derive(Serialize, Deserialize)]
pub struct EncryptResponse {
    pub ciphertext: String,
    pub keystream_bytes: Vec<u8>,
}

#[tauri::command]
pub fn encrypt_decrypt(plaintext: String, hex_output: bool) -> EncryptResponse {
    let temp_entropy = SystemEntropy::new();
    let mut cipher = StreamCipher::new(temp_entropy);
    let output = cipher.process(plaintext.as_bytes());

    let ciphertext = if hex_output {
        hex::encode(&output)
    } else {
        String::from_utf8_lossy(&output).to_string()
    };

    let mut keystream_bytes = cipher.state().to_vec();

    // Pad to 64 bytes if needed for visualization grid
    if keystream_bytes.len() < 64 {
        keystream_bytes.resize(64, 0);
    } else {
        keystream_bytes.truncate(64);
    }

    EncryptResponse {
        ciphertext,
        keystream_bytes,
    }
}
