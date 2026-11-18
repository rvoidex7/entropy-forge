//! Biased coin source with an adjustable `P(1)`.

use super::EntropySource;

/// Independent bits with `P(1) = probability`.
///
/// The underlying uniform stream is a xorshift64 generator, which is fine here
/// because the point of this source is the bias, not the generator. A byte is
/// assembled MSB-first from eight independent biased bits.
///
/// # Security Note
///
/// A biased source leaks its bias immediately: the frequency test fails, and
/// so does anything that assumes a uniform distribution.
#[derive(Debug, Clone)]
pub struct BiasedSource {
    probability: f64,
    state: u64,
    initial_state: u64,
}

impl BiasedSource {
    /// Create a biased source. `probability` is clamped to `[0, 1]`.
    pub fn new(probability: f64) -> Self {
        Self::with_seed(probability, 0x5EED_0BAD_C0FF_EE01)
    }

    /// Create a biased source with an explicit internal seed.
    pub fn with_seed(probability: f64, seed: u64) -> Self {
        let state = if seed == 0 {
            0x9E37_79B9_7F4A_7C15
        } else {
            seed
        };

        Self {
            probability: probability.clamp(0.0, 1.0),
            state,
            initial_state: state,
        }
    }

    /// Current `P(1)`.
    pub const fn probability(&self) -> f64 {
        self.probability
    }

    /// Change `P(1)` without rewinding the stream.
    pub fn set_probability(&mut self, probability: f64) {
        self.probability = probability.clamp(0.0, 1.0);
    }

    /// Uniform `f64` in `[0, 1)` from the high 53 bits of a xorshift64 step.
    fn next_uniform(&mut self) -> f64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;

        let bits = x >> 11;
        bits as f64 / (1u64 << 53) as f64
    }

    fn next_bit(&mut self) -> u8 {
        u8::from(self.next_uniform() < self.probability)
    }
}

impl EntropySource for BiasedSource {
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
        "Biased coin (adjustable P(1))"
    }

    fn reset(&mut self) {
        self.state = self.initial_state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ones_fraction(source: &mut BiasedSource, bytes: usize) -> f64 {
        let mut buffer = vec![0u8; bytes];
        source.fill_bytes(&mut buffer);
        let ones: usize = buffer.iter().map(|b| b.count_ones() as usize).sum();
        ones as f64 / (bytes as f64 * 8.0)
    }

    #[test]
    fn probability_zero_yields_all_zeros() {
        let mut source = BiasedSource::new(0.0);
        let mut buffer = [0xFFu8; 256];
        source.fill_bytes(&mut buffer);

        assert!(buffer.iter().all(|&b| b == 0));
    }

    #[test]
    fn probability_one_yields_all_ones() {
        let mut source = BiasedSource::new(1.0);
        let mut buffer = [0u8; 256];
        source.fill_bytes(&mut buffer);

        assert!(buffer.iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn high_probability_produces_mostly_ones() {
        let mut source = BiasedSource::new(0.9);
        let fraction = ones_fraction(&mut source, 20_000);

        assert!(fraction > 0.88, "ones fraction was {fraction}");
    }

    #[test]
    fn low_probability_produces_mostly_zeros() {
        let mut source = BiasedSource::new(0.1);
        let fraction = ones_fraction(&mut source, 20_000);

        assert!(fraction < 0.12, "ones fraction was {fraction}");
    }

    #[test]
    fn fair_coin_stays_near_half() {
        let mut source = BiasedSource::new(0.5);
        let fraction = ones_fraction(&mut source, 50_000);

        assert!((0.48..0.52).contains(&fraction), "fraction was {fraction}");
    }

    #[test]
    fn probability_is_clamped() {
        assert_eq!(BiasedSource::new(-1.0).probability(), 0.0);
        assert_eq!(BiasedSource::new(2.0).probability(), 1.0);

        let mut source = BiasedSource::new(0.5);
        source.set_probability(5.0);
        assert_eq!(source.probability(), 1.0);
    }

    #[test]
    fn same_seed_reproduces() {
        let mut first = BiasedSource::with_seed(0.7, 42);
        let mut second = BiasedSource::with_seed(0.7, 42);

        let mut a = [0u8; 64];
        let mut b = [0u8; 64];
        first.fill_bytes(&mut a);
        second.fill_bytes(&mut b);

        assert_eq!(a, b);
    }

    #[test]
    fn reset_rewinds_the_stream() {
        let mut source = BiasedSource::new(0.6);

        let mut first = [0u8; 32];
        let mut second = [0u8; 32];
        source.fill_bytes(&mut first);
        source.reset();
        source.fill_bytes(&mut second);

        assert_eq!(first, second);
    }

    #[test]
    fn zero_seed_does_not_get_stuck() {
        let mut source = BiasedSource::with_seed(0.5, 0);
        let fraction = ones_fraction(&mut source, 1_000);

        assert!(fraction > 0.3 && fraction < 0.7, "fraction was {fraction}");
    }

    #[test]
    fn name_identifies_the_source() {
        assert!(BiasedSource::new(0.5).name().contains("Biased"));
    }
}
