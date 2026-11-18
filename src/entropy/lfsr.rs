//! 16-bit linear feedback shift register.

use super::EntropySource;

/// Tap mask for `x^16 + x^14 + x^13 + x^11 + 1`.
///
/// This polynomial is maximal, so the register cycles through every non-zero
/// 16-bit state exactly once: a period of 65,535 bits, or 65,535 bytes when
/// one byte is produced per eight shifts.
const TAPS: u16 = 0xB400;

/// A 16-bit LFSR, the textbook example of a linear generator.
///
/// The output passes a frequency check and fails anything that looks at
/// structure: it is linear, so a short output window predicts the rest.
///
/// # Security Note
///
/// Linear feedback is trivially invertible. Never use an LFSR as a
/// cryptographic random source.
#[derive(Debug, Clone)]
pub struct Lfsr16Source {
    state: u16,
    initial_state: u16,
}

impl Lfsr16Source {
    /// Create a register from a seed. Zero is replaced because an all-zero
    /// register is a fixed point.
    pub fn new(seed: u16) -> Self {
        let state = if seed == 0 { 1 } else { seed };

        Self {
            state,
            initial_state: state,
        }
    }

    fn next_bit(&mut self) -> u8 {
        let bit = (self.state & 1) as u8;
        self.state >>= 1;
        if bit == 1 {
            self.state ^= TAPS;
        }
        bit
    }
}

impl EntropySource for Lfsr16Source {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            let mut value = 0u8;
            for _ in 0..8 {
                value = (value << 1) | self.next_bit();
            }
            *byte = value;
        }
    }

    fn name(&self) -> &str {
        "LFSR-16 (linear feedback)"
    }

    fn reset(&mut self) {
        self.state = self.initial_state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Bytes in one full cycle of the 16-bit register.
    const PERIOD_BYTES: usize = 65_535;

    #[test]
    fn same_seed_reproduces() {
        let mut first = Lfsr16Source::new(0xACE1);
        let mut second = Lfsr16Source::new(0xACE1);

        let mut a = [0u8; 64];
        let mut b = [0u8; 64];
        first.fill_bytes(&mut a);
        second.fill_bytes(&mut b);

        assert_eq!(a, b);
    }

    #[test]
    fn reset_rewinds_the_register() {
        let mut source = Lfsr16Source::new(0x1234);

        let mut first = [0u8; 32];
        let mut second = [0u8; 32];
        source.fill_bytes(&mut first);
        source.reset();
        source.fill_bytes(&mut second);

        assert_eq!(first, second);
    }

    #[test]
    fn period_is_65535_bytes() {
        let mut source = Lfsr16Source::new(0xACE1);
        let start = source.state;

        let mut buffer = vec![0u8; PERIOD_BYTES];
        source.fill_bytes(&mut buffer);
        assert_eq!(source.state, start, "register must return to its start");

        // The period is not shorter: one byte earlier the state differs.
        let mut short = Lfsr16Source::new(0xACE1);
        let mut buffer = vec![0u8; PERIOD_BYTES - 1];
        short.fill_bytes(&mut buffer);
        assert_ne!(short.state, start, "period must be exactly 65,535 bytes");
    }

    #[test]
    fn register_never_becomes_zero() {
        let mut source = Lfsr16Source::new(0xACE1);
        let mut buffer = vec![0u8; PERIOD_BYTES];
        source.fill_bytes(&mut buffer);

        assert_ne!(source.state, 0, "maximal LFSR must avoid the zero state");
    }

    #[test]
    fn zero_seed_is_replaced() {
        let mut source = Lfsr16Source::new(0);
        let mut buffer = [0u8; 8];
        source.fill_bytes(&mut buffer);

        assert!(buffer.iter().any(|&b| b != 0));
    }

    #[test]
    fn bits_are_roughly_balanced_over_a_full_period() {
        let mut source = Lfsr16Source::new(0xACE1);
        let mut buffer = vec![0u8; PERIOD_BYTES];
        source.fill_bytes(&mut buffer);

        let ones: usize = buffer.iter().map(|b| b.count_ones() as usize).sum();
        let ratio = ones as f64 / (PERIOD_BYTES as f64 * 8.0);

        assert!((0.45..0.55).contains(&ratio), "ones ratio was {ratio}");
    }

    #[test]
    fn name_identifies_the_source() {
        assert!(Lfsr16Source::new(1).name().contains("LFSR"));
    }
}
