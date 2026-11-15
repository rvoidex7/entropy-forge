//! Golden and adversarial tests for the cryptographic and statistical core.
//!
//! Expected values come from NIST SP 800-22 Rev. 1a (Sections 2.1.8, 2.3.8,
//! 2.4.8 and 2.13.8). These values are fixed by the specification and must not
//! be relaxed.

use entropy_forge::crypto::{AvalancheAnalyzer, KeyedStreamCipher};
use entropy_forge::learn::EncryptionProcess;
use entropy_forge::quality::{NistTests, QualityReport};

/// Convert a `"110010..."` string to an MSB-first byte vector.
fn bitstring_to_bytes_msb(bits: &str) -> (Vec<u8>, usize) {
    let clean: Vec<u8> = bits
        .chars()
        .filter(|c| *c == '0' || *c == '1')
        .map(|c| u8::from(c == '1'))
        .collect();
    let bit_len = clean.len();
    let mut bytes = vec![0u8; bit_len.div_ceil(8)];
    for (index, bit) in clean.into_iter().enumerate() {
        bytes[index / 8] |= bit << (7 - (index % 8));
    }
    (bytes, bit_len)
}

/// The official 100-bit NIST test sequence: the first 100 fractional bits of pi.
const NIST_PI_100: &str = "11001001000011111101101010100010001000010110100011\
                            00001000110100110001001100011001100010100010111000";

#[test]
fn nist_official_vector_frequency_monobit_2_1_8() {
    // Ref: NIST SP 800-22 Rev 1a, Section 2.1.8: expected P-value = 0.109599.
    let (bytes, bit_len) = bitstring_to_bytes_msb(NIST_PI_100);
    let p = NistTests::frequency_test_bits(&bytes, bit_len);
    assert!(
        (p - 0.109599).abs() < 1e-5,
        "NIST 2.1.8 Frequency P-value: expected 0.109599, got {p:.6}"
    );
}

#[test]
fn nist_official_vector_runs_test_2_3_8() {
    // Ref: NIST SP 800-22 Rev 1a, Section 2.3.8: expected P-value = 0.500798.
    let (bytes, bit_len) = bitstring_to_bytes_msb(NIST_PI_100);
    let p = NistTests::runs_test_bits(&bytes, bit_len);
    assert!(
        (p - 0.500798).abs() < 1e-5,
        "NIST 2.3.8 Runs P-value: expected 0.500798, got {p:.6}"
    );
}

#[test]
fn nist_official_vector_longest_run_2_4_8() {
    // Ref: NIST SP 800-22 Rev 1a, Section 2.4.8 (16 blocks of 8 bits):
    // v0=4, v1=9, v2=3, v3=0 -> chi_sq = 4.882457 -> expected P-value = 0.180609.
    let bits = concat!(
        "11001100", "00010101", "01101100", "01001100", "11100000", "00000010", "01001101",
        "01010001", "00010011", "11010110", "10000000", "11010111", "11001100", "11100110",
        "11011000", "10110010",
    );
    let (bytes, bit_len) = bitstring_to_bytes_msb(bits);
    assert_eq!(bit_len, 128);
    let p = NistTests::longest_run_test_bits(&bytes, bit_len);
    assert!(
        (p - 0.180609).abs() < 1e-4,
        "NIST 2.4.8 Longest Run P-value: expected 0.180609, got {p:.6}"
    );
}

#[test]
fn nist_official_vector_cusum_2_13_8() {
    // Ref: NIST SP 800-22 Rev 1a, Section 2.13.8:
    // forward P-value = 0.219194, reverse P-value = 0.114866.
    let (bytes, bit_len) = bitstring_to_bytes_msb(NIST_PI_100);
    let (forward, reverse) = NistTests::cusum_test_bits(&bytes, bit_len);
    assert!(
        (forward - 0.219194).abs() < 1e-5,
        "NIST 2.13.8 CUSUM forward: expected 0.219194, got {forward:.6}"
    );
    assert!(
        (reverse - 0.114866).abs() < 1e-5,
        "NIST 2.13.8 CUSUM reverse: expected 0.114866, got {reverse:.6}"
    );
}

#[test]
fn adversarial_counter_must_fail_quality_and_nist() {
    // A repeating 0, 1, ..., 255 counter has Shannon entropy 8.0 and mean
    // 127.5, yet it has zero cryptographic security.
    let counter_data: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
    let report = QualityReport::analyze_slice(&counter_data);

    assert!((report.shannon_entropy - 8.0).abs() < 0.01);
    assert!(
        report.markov_min_entropy < 0.05,
        "Counter conditional entropy must be near zero, got {}",
        report.markov_min_entropy
    );
    assert!(
        report.overall_score() <= 30.0,
        "A deterministic counter cannot score above 30, got {}",
        report.overall_score()
    );
    assert!(report.is_suspiciously_uniform);
}

#[test]
fn adversarial_alternating_0xaa_passes_monobit_but_fails_runs_and_serial() {
    // 0xAA = 10101010... : 50% ones, but maximum oscillation.
    let alternating = vec![0xAAu8; 2_000];
    assert!(NistTests::frequency_test(&alternating) > 0.99);
    assert!(
        NistTests::runs_test(&alternating) < 0.01,
        "0xAA must fail the Runs test"
    );
    let (p1, _p2) = NistTests::serial_test(&alternating, 3);
    assert!(p1 < 0.01, "0xAA must fail the Serial test");
}

#[test]
fn stream_cipher_turkish_utf8_roundtrip() {
    let plaintext = "Şifreleme testi: ğüşıöç ĞÜŞİÖÇ 🔐 12345!";
    let key = [0x42u8; 32];
    let nonce = [0x24u8; 12];

    let ciphertext = KeyedStreamCipher::process_chacha20(&key, &nonce, plaintext.as_bytes());
    assert_ne!(ciphertext, plaintext.as_bytes());

    let decrypted_bytes = KeyedStreamCipher::process_chacha20(&key, &nonce, &ciphertext);
    let decrypted = String::from_utf8(decrypted_bytes).expect("output must be valid UTF-8");
    assert_eq!(decrypted, plaintext);
}

#[test]
fn learn_xor_utf8_preserves_multibyte_characters() {
    let text = "şğİ"; // 6 UTF-8 bytes, not 3 truncated `char as u8` values.
    let mut process = EncryptionProcess::new();
    process.start(text);

    assert_eq!(process.steps.len(), text.len());
    for (step, &expected_byte) in process.steps.iter().zip(text.as_bytes()) {
        assert_eq!(step.input_byte, expected_byte);
        assert_eq!(step.input_byte ^ step.keystream_byte, step.result_byte);
    }
}

#[test]
fn sha256_strict_avalanche_criterion_is_near_50_percent() {
    // Mean over 200 single-bit flips follows Binomial(256, 0.5).
    let stats = AvalancheAnalyzer::analyze_sha256(b"Entropy Forge Avalanche Benchmark");
    assert!(
        (stats.mean_flip_percentage - 50.0).abs() < 3.0,
        "SHA-256 mean avalanche should be ~50%, got {:.2}%",
        stats.mean_flip_percentage
    );
}
