//! NIST SP 800-22 statistical tests.
//!
//! Every test scans the input in **MSB-first** order through the
//! zero-allocation helpers in [`super::bits`]. Tests that need a logical bit
//! length (the official NIST vectors are not always a whole number of bytes)
//! come in two flavours:
//!
//! * `*_test(data)` — uses all `data.len() * 8` bits.
//! * `*_test_bits(data, bit_len)` — uses only the first `bit_len` bits.
//!
//! The decision threshold is `p_value >= 0.01` (NIST alpha).

use statrs::distribution::{ChiSquared, ContinuousCDF, Normal};

use super::bits::{bit_at_msb, BitsMsb};

/// NIST SP 800-22 statistical tests.
pub struct NistTests;

impl NistTests {
    /// Frequency (monobit) test — NIST SP 800-22 §2.1.
    ///
    /// Tests whether the number of ones and zeros is approximately equal.
    pub fn frequency_test(data: &[u8]) -> f64 {
        Self::frequency_test_bits(data, data.len() * 8)
    }

    /// Frequency (monobit) test over the first `bit_len` bits.
    pub fn frequency_test_bits(data: &[u8], bit_len: usize) -> f64 {
        // NIST SP 800-22 requires n >= 100 bits.
        if bit_len < 100 || bit_len > data.len() * 8 {
            return 0.0;
        }

        let mut sum: i64 = 0;
        for bit in BitsMsb::with_len(data, bit_len) {
            sum += if bit == 1 { 1 } else { -1 };
        }

        let n = bit_len as f64;
        let s_obs = (sum as f64).abs() / n.sqrt();
        Self::erfc(s_obs / std::f64::consts::SQRT_2)
    }

    /// Frequency test within a block — NIST SP 800-22 §2.2.
    ///
    /// `m` is the block size (NIST recommends `m = 128`). The block size must
    /// be at least 20 bits and the sequence must be at least 100 bits long.
    pub fn block_frequency_test(data: &[u8], m: usize) -> f64 {
        Self::block_frequency_test_bits(data, data.len() * 8, m)
    }

    /// Block frequency test over the first `bit_len` bits.
    pub fn block_frequency_test_bits(data: &[u8], bit_len: usize, m: usize) -> f64 {
        if bit_len > data.len() * 8 || bit_len < 100 || m < 20 || m > bit_len {
            return 0.0;
        }

        let blocks = bit_len / m;
        if blocks == 0 {
            return 0.0;
        }

        let mut chi_sq = 0.0;
        for block in 0..blocks {
            let start = block * m;
            // Take a window of exactly `m` bits starting at `start`. Slicing
            // the byte array first would truncate blocks that do not start on
            // a byte boundary, and an iterator `skip(start)` would rescan the
            // sequence from bit zero for every block (quadratic runtime).
            let ones: usize = (start..start + m)
                .filter(|&index| bit_at_msb(data, index) == 1)
                .count();
            let pi = ones as f64 / m as f64;
            let diff = pi - 0.5;
            chi_sq += diff * diff;
        }
        chi_sq *= 4.0 * m as f64;

        chi_square_p_value(chi_sq, blocks as f64)
    }

    /// Runs test — NIST SP 800-22 §2.3.
    ///
    /// Counts uninterrupted runs of identical bits. Too few or too many runs
    /// indicate non-randomness.
    pub fn runs_test(data: &[u8]) -> f64 {
        Self::runs_test_bits(data, data.len() * 8)
    }

    /// Runs test over the first `bit_len` bits.
    pub fn runs_test_bits(data: &[u8], bit_len: usize) -> f64 {
        // NIST SP 800-22 requires n >= 100 bits.
        if bit_len < 100 || bit_len > data.len() * 8 {
            return 0.0;
        }

        let n = bit_len as f64;
        let ones = BitsMsb::with_len(data, bit_len)
            .filter(|&bit| bit == 1)
            .count() as f64;
        let pi = ones / n;

        // Pre-test: the sequence must be close to balanced.
        if (pi - 0.5).abs() >= 2.0 / n.sqrt() {
            return 0.0;
        }

        let mut runs = 1usize;
        let mut previous: Option<u8> = None;
        for bit in BitsMsb::with_len(data, bit_len) {
            if let Some(prev) = previous {
                if bit != prev {
                    runs += 1;
                }
            }
            previous = Some(bit);
        }

        // NIST §2.3.4: the sqrt(2) is already inside 2*sqrt(2n). Do not add
        // another SQRT_2 here.
        let numerator = (runs as f64 - 2.0 * n * pi * (1.0 - pi)).abs();
        let denominator = 2.0 * (2.0 * n).sqrt() * pi * (1.0 - pi);
        if denominator == 0.0 {
            return 0.0;
        }

        Self::erfc(numerator / denominator)
    }

    /// Longest run of ones in a block — NIST SP 800-22 §2.4.
    pub fn longest_run_test(data: &[u8]) -> f64 {
        Self::longest_run_test_bits(data, data.len() * 8)
    }

    /// Longest run of ones over the first `bit_len` bits.
    ///
    /// Uses the fixed block counts from the NIST table (N = 16, 49, 75) and
    /// ignores any trailing bits that do not fill a complete block.
    pub fn longest_run_test_bits(data: &[u8], bit_len: usize) -> f64 {
        if bit_len > data.len() * 8 {
            return 0.0;
        }

        // (block size M, degrees of freedom K, fixed block count N,
        //  upper bounds for categories, category probabilities)
        let (block_size, degrees, blocks, bounds, probabilities): (
            usize,
            usize,
            usize,
            &[usize],
            &[f64],
        ) = if bit_len < 6_272 {
            if bit_len < 128 {
                return 0.0;
            }
            (8, 3, 16, &[1, 2, 3], &[0.2148, 0.3672, 0.2305, 0.1875])
        } else if bit_len < 750_000 {
            (
                128,
                5,
                49,
                &[4, 5, 6, 7, 8],
                &[0.1174, 0.2430, 0.2493, 0.1752, 0.1027, 0.1124],
            )
        } else {
            (
                10_000,
                6,
                75,
                &[10, 11, 12, 13, 14, 15],
                &[0.0882, 0.2092, 0.2483, 0.1933, 0.1208, 0.0675, 0.0727],
            )
        };

        debug_assert_eq!(bounds.len() + 1, probabilities.len());
        debug_assert!(blocks * block_size <= bit_len);

        let mut observed = vec![0usize; probabilities.len()];

        for block in 0..blocks {
            let start = block * block_size;
            let mut longest = 0usize;
            let mut current = 0usize;
            for offset in 0..block_size {
                if bit_at_msb(data, start + offset) == 1 {
                    current += 1;
                    longest = longest.max(current);
                } else {
                    current = 0;
                }
            }

            let category = bounds
                .iter()
                .position(|&upper| longest <= upper)
                .unwrap_or(bounds.len());
            observed[category] += 1;
        }

        let n = blocks as f64;
        let mut chi_sq = 0.0;
        for (count, probability) in observed.iter().zip(probabilities) {
            let expected = n * probability;
            let diff = *count as f64 - expected;
            chi_sq += diff * diff / expected;
        }

        chi_square_p_value(chi_sq, degrees as f64)
    }

    /// Chi-square test for byte distribution.
    ///
    /// Tests whether the byte values are uniformly distributed. Note that a
    /// p-value close to 1.0 means the distribution is *too* uniform and is
    /// itself suspicious for a real entropy source; see `QualityMetrics`.
    pub fn chi_square_test(data: &[u8]) -> f64 {
        let chi_sq = Self::chi_square_statistic(data);
        if chi_sq < 0.0 {
            return 0.0;
        }
        chi_square_p_value(chi_sq, 255.0)
    }

    /// Raw chi-square statistic for the byte distribution (df = 255).
    ///
    /// Returns a negative value for empty input.
    pub fn chi_square_statistic(data: &[u8]) -> f64 {
        if data.is_empty() {
            return -1.0;
        }

        let mut frequency = [0usize; 256];
        for &byte in data {
            frequency[byte as usize] += 1;
        }

        let expected = data.len() as f64 / 256.0;
        let mut chi_sq = 0.0;
        for count in frequency {
            let diff = count as f64 - expected;
            chi_sq += diff * diff / expected;
        }
        chi_sq
    }

    /// Serial test — NIST SP 800-22 §2.11.
    ///
    /// Returns `(p_value_1, p_value_2)` for the two difference statistics
    /// `∇ψ²_m` and `∇²ψ²_m`. `m` is the pattern length; NIST recommends 3.
    pub fn serial_test(data: &[u8], m: usize) -> (f64, f64) {
        Self::serial_test_bits(data, data.len() * 8, m)
    }

    /// Serial test over the first `bit_len` bits.
    pub fn serial_test_bits(data: &[u8], bit_len: usize, m: usize) -> (f64, f64) {
        // NIST SP 800-22 requires n >= 100 bits and m < n.
        if !(2..=16).contains(&m) || bit_len < 100 || bit_len > data.len() * 8 {
            return (0.0, 0.0);
        }

        let psi_m = psi_squared(data, bit_len, m);
        let psi_m1 = psi_squared(data, bit_len, m - 1);
        let psi_m2 = psi_squared(data, bit_len, m - 2);

        let nabla = psi_m - psi_m1;
        let nabla2 = psi_m - 2.0 * psi_m1 + psi_m2;

        let df1 = 2f64.powi(m as i32 - 1);
        let df2 = 2f64.powi(m as i32 - 2);

        (
            chi_square_p_value(nabla.max(0.0), df1),
            chi_square_p_value(nabla2.max(0.0), df2),
        )
    }

    /// Cumulative sums (CUSUM) test — NIST SP 800-22 §2.13.
    ///
    /// Returns `(forward_p_value, reverse_p_value)`.
    pub fn cusum_test(data: &[u8]) -> (f64, f64) {
        Self::cusum_test_bits(data, data.len() * 8)
    }

    /// CUSUM test over the first `bit_len` bits.
    pub fn cusum_test_bits(data: &[u8], bit_len: usize) -> (f64, f64) {
        // NIST SP 800-22 requires n >= 100 bits.
        if bit_len < 100 || bit_len > data.len() * 8 {
            return (0.0, 0.0);
        }

        (
            cusum_p_value(data, bit_len, false),
            cusum_p_value(data, bit_len, true),
        )
    }

    /// Run all implemented tests and return `(name, p_value)` pairs.
    pub fn run_all_tests(data: &[u8]) -> Vec<(&'static str, f64)> {
        let serial = Self::serial_test(data, 3);
        let cusum = Self::cusum_test(data);

        vec![
            ("Frequency Test", Self::frequency_test(data)),
            (
                "Block Frequency Test",
                Self::block_frequency_test(data, 128),
            ),
            ("Runs Test", Self::runs_test(data)),
            ("Longest Run Test", Self::longest_run_test(data)),
            ("Chi-Square Test", Self::chi_square_test(data)),
            ("Serial Test (p1)", serial.0),
            ("Serial Test (p2)", serial.1),
            ("Cumulative Sums (forward)", cusum.0),
            ("Cumulative Sums (reverse)", cusum.1),
        ]
    }

    /// Complementary error function.
    ///
    /// Kept as a public helper for the educational visualization code.
    pub fn erfc(x: f64) -> f64 {
        statrs::function::erf::erfc(x)
    }
}

/// Compute `ψ²_k` with circular bit wrapping, without allocating a bit vector.
fn psi_squared(data: &[u8], bit_len: usize, k: usize) -> f64 {
    if k == 0 || bit_len < k {
        return 0.0;
    }

    let patterns = 1usize << k;
    let mask = patterns - 1;
    let mut counts = vec![0usize; patterns];

    let mut window = 0usize;
    for index in 0..k {
        window = (window << 1) | bit_at_msb(data, index) as usize;
    }
    counts[window] += 1;

    for index in 1..bit_len {
        let next = bit_at_msb(data, (index + k - 1) % bit_len) as usize;
        window = ((window << 1) | next) & mask;
        counts[window] += 1;
    }

    let sum_squares: f64 = counts.iter().map(|&count| (count * count) as f64).sum();
    (patterns as f64 / bit_len as f64) * sum_squares - bit_len as f64
}

/// CUSUM p-value for one direction of the sequence.
fn cusum_p_value(data: &[u8], bit_len: usize, reverse: bool) -> f64 {
    let mut sum: i64 = 0;
    let mut max_abs: i64 = 0;

    for index in 0..bit_len {
        let bit_index = if reverse { bit_len - 1 - index } else { index };
        sum += if bit_at_msb(data, bit_index) == 1 {
            1
        } else {
            -1
        };
        max_abs = max_abs.max(sum.abs());
    }

    let z = max_abs as f64;
    if z == 0.0 {
        return 1.0;
    }

    let n = bit_len as f64;
    let sqrt_n = n.sqrt();
    let normal = match Normal::new(0.0, 1.0) {
        Ok(distribution) => distribution,
        Err(_) => return 0.0,
    };

    // The NIST summation index `k` is an integer. Starting from a fractional
    // value silently shifts every term and produces p = 1.0.
    let upper = (n / z - 1.0) / 4.0;

    let mut sum1 = 0.0;
    let mut k = ((-n / z + 1.0) / 4.0).ceil() as i64;
    while (k as f64) <= upper {
        let kf = k as f64;
        sum1 +=
            normal.cdf((4.0 * kf + 1.0) * z / sqrt_n) - normal.cdf((4.0 * kf - 1.0) * z / sqrt_n);
        k += 1;
    }

    let mut sum2 = 0.0;
    let mut k = ((-n / z - 3.0) / 4.0).ceil() as i64;
    while (k as f64) <= upper {
        let kf = k as f64;
        sum2 +=
            normal.cdf((4.0 * kf + 3.0) * z / sqrt_n) - normal.cdf((4.0 * kf + 1.0) * z / sqrt_n);
        k += 1;
    }

    (1.0 - sum1 + sum2).clamp(0.0, 1.0)
}

/// Upper-tail p-value of a chi-square distribution.
fn chi_square_p_value(statistic: f64, degrees_of_freedom: f64) -> f64 {
    if !statistic.is_finite() || degrees_of_freedom <= 0.0 {
        return 0.0;
    }
    if statistic <= 0.0 {
        return 1.0;
    }

    match ChiSquared::new(degrees_of_freedom) {
        Ok(distribution) => (1.0 - distribution.cdf(statistic)).clamp(0.0, 1.0),
        Err(_) => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bitstring_to_bytes_msb(bits: &str) -> (Vec<u8>, usize) {
        let clean: Vec<u8> = bits
            .chars()
            .filter(|c| *c == '0' || *c == '1')
            .map(|c| if c == '1' { 1u8 } else { 0u8 })
            .collect();
        let bit_len = clean.len();
        let mut bytes = vec![0u8; bit_len.div_ceil(8)];
        for (index, bit) in clean.into_iter().enumerate() {
            bytes[index / 8] |= bit << (7 - (index % 8));
        }
        (bytes, bit_len)
    }

    const NIST_PI_100: &str = "11001001000011111101101010100010001000010110100011\
                               00001000110100110001001100011001100010100010111000";

    #[test]
    fn nist_official_frequency_vector_2_1_8() {
        let (bytes, bit_len) = bitstring_to_bytes_msb(NIST_PI_100);
        let p = NistTests::frequency_test_bits(&bytes, bit_len);
        assert!((p - 0.109599).abs() < 1e-5, "frequency p = {p:.6}");
    }

    #[test]
    fn nist_official_runs_vector_2_3_8() {
        let (bytes, bit_len) = bitstring_to_bytes_msb(NIST_PI_100);
        let p = NistTests::runs_test_bits(&bytes, bit_len);
        assert!((p - 0.500798).abs() < 1e-5, "runs p = {p:.6}");
    }

    #[test]
    fn nist_official_longest_run_vector_2_4_8() {
        let bits = concat!(
            "11001100", "00010101", "01101100", "01001100", "11100000", "00000010", "01001101",
            "01010001", "00010011", "11010110", "10000000", "11010111", "11001100", "11100110",
            "11011000", "10110010",
        );
        let (bytes, bit_len) = bitstring_to_bytes_msb(bits);
        assert_eq!(bit_len, 128);
        let p = NistTests::longest_run_test_bits(&bytes, bit_len);
        assert!((p - 0.180609).abs() < 1e-4, "longest run p = {p:.6}");
    }

    #[test]
    fn nist_official_cusum_vector_2_13_8() {
        let (bytes, bit_len) = bitstring_to_bytes_msb(NIST_PI_100);
        let (forward, reverse) = NistTests::cusum_test_bits(&bytes, bit_len);
        assert!(
            (forward - 0.219194).abs() < 1e-5,
            "cusum forward p = {forward:.6}"
        );
        assert!(
            (reverse - 0.114866).abs() < 1e-5,
            "cusum reverse p = {reverse:.6}"
        );
    }

    #[test]
    fn block_frequency_counts_full_non_aligned_blocks() {
        // Every 20-bit window of the alternating pattern holds exactly ten
        // ones, so chi-square must be zero and p must be one. A truncated
        // window (the old bug) would count fewer bits and lower the p-value.
        let data = vec![0b1010_1010u8; 1_000];
        let p = NistTests::block_frequency_test_bits(&data, data.len() * 8, 20);
        assert!(p > 0.9999, "block frequency p = {p:.6}");
    }

    #[test]
    fn alternating_sequence_passes_monobit_but_fails_runs_and_serial() {
        let alternating = vec![0xAAu8; 2_000];
        assert!(NistTests::frequency_test(&alternating) > 0.99);
        assert!(NistTests::runs_test(&alternating) < 0.01);

        let (p1, p2) = NistTests::serial_test(&alternating, 3);
        assert!(p1 < 0.01, "serial p1 = {p1}");
        assert!(p2 < 0.01, "serial p2 = {p2}");
    }

    #[test]
    fn counter_passes_monobit_despite_being_deterministic() {
        let counter: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
        assert!(NistTests::frequency_test(&counter) > 0.5);
        // A byte counter is perfectly balanced, so bit-level tests alone can
        // miss it. QualityMetrics::markov_min_entropy is what catches it.
    }

    #[test]
    fn block_frequency_and_cusum_are_bounded() {
        let mut data = vec![0u8; 10_000];
        getrandom::getrandom(&mut data).unwrap();

        let block = NistTests::block_frequency_test(&data, 128);
        assert!((0.0..=1.0).contains(&block));

        let (forward, reverse) = NistTests::cusum_test(&data);
        assert!((0.0..=1.0).contains(&forward));
        assert!((0.0..=1.0).contains(&reverse));
    }
}
