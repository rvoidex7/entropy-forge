//! Deterministic byte counter.

use super::EntropySource;

/// Produces `0, 1, 2, ..., 255, 0, 1, ...`.
///
/// This is the pedagogical centre of the lab: the histogram is perfect, the
/// chi-square statistic is zero, and the next byte is fully predictable.
///
/// # Security Note
///
/// A counter has zero entropy no matter how uniform it looks. Never use it as
/// a random source.
#[derive(Debug, Clone)]
pub struct CounterSource {
    next: u8,
    initial: u8,
}

impl CounterSource {
    /// Start counting at zero.
    pub fn new() -> Self {
        Self::starting_at(0)
    }

    /// Start counting at `start`, wrapping at 256.
    pub fn starting_at(start: u8) -> Self {
        Self {
            next: start,
            initial: start,
        }
    }
}

impl Default for CounterSource {
    fn default() -> Self {
        Self::new()
    }
}

impl EntropySource for CounterSource {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            *byte = self.next;
            self.next = self.next.wrapping_add(1);
        }
    }

    fn name(&self) -> &str {
        "Counter (incrementing byte)"
    }

    fn reset(&mut self) {
        self.next = self.initial;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_every_byte_in_order() {
        let mut source = CounterSource::new();
        let mut buffer = [0u8; 256];
        source.fill_bytes(&mut buffer);

        assert_eq!(buffer, std::array::from_fn(|i| i as u8));
    }

    #[test]
    fn wraps_after_255() {
        let mut source = CounterSource::starting_at(254);
        let mut buffer = [0u8; 4];
        source.fill_bytes(&mut buffer);

        assert_eq!(buffer, [254, 255, 0, 1]);
    }

    #[test]
    fn reset_rewinds_to_the_start_byte() {
        let mut source = CounterSource::starting_at(10);

        let mut first = [0u8; 8];
        let mut second = [0u8; 8];
        source.fill_bytes(&mut first);
        source.reset();
        source.fill_bytes(&mut second);

        assert_eq!(first, second);
        assert_eq!(first[0], 10);
    }

    #[test]
    fn default_starts_at_zero() {
        let mut source = CounterSource::default();
        assert_eq!(source.next_byte(), 0);
        assert_eq!(source.next_byte(), 1);
    }

    #[test]
    fn name_identifies_the_source() {
        assert!(CounterSource::new().name().contains("Counter"));
    }
}
