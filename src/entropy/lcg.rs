//! Linear congruential generators, on purpose weak.

use super::EntropySource;

/// The two LCG parameter sets worth studying.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LcgPreset {
    /// The classic glibc `rand()` constants: full period, still structurally
    /// broken.
    Glibc,
    /// IBM's RANDU: notorious for producing output that lies on 15 planes in
    /// three dimensions.
    Randu,
}

impl LcgPreset {
    /// Short name for display.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Glibc => "LCG (glibc)",
            Self::Randu => "RANDU",
        }
    }
}

/// A 31-bit LCG with a selectable preset.
///
/// Both presets use `m = 2^31` and return bits 16..23 of the state, which is
/// the part glibc and RANDU expose. Low bits of an LCG are far worse than high
/// bits, so they are discarded.
///
/// # Security Note
///
/// LCGs are predictable from a handful of outputs and show lattice structure.
/// They exist here to be measured, not used.
#[derive(Debug, Clone)]
pub struct LcgSource {
    preset: LcgPreset,
    state: u32,
    initial_state: u32,
}

impl LcgSource {
    /// Create a generator with the given preset and seed.
    pub fn new(preset: LcgPreset, seed: u64) -> Self {
        let mut state = (seed as u32) & 0x7FFF_FFFF;
        // RANDU maps zero to zero forever; nudge it off the fixed point.
        if state == 0 {
            state = 1;
        }

        Self {
            preset,
            state,
            initial_state: state,
        }
    }

    /// The preset this instance was built with.
    pub const fn preset(&self) -> LcgPreset {
        self.preset
    }

    fn next(&mut self) -> u8 {
        self.state = match self.preset {
            LcgPreset::Glibc => {
                self.state.wrapping_mul(1_103_515_245).wrapping_add(12_345) & 0x7FFF_FFFF
            }
            LcgPreset::Randu => self.state.wrapping_mul(65_539) & 0x7FFF_FFFF,
        };

        (self.state >> 16) as u8
    }
}

impl EntropySource for LcgSource {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            *byte = self.next();
        }
    }

    fn name(&self) -> &str {
        match self.preset {
            LcgPreset::Glibc => "LCG (glibc constants)",
            LcgPreset::Randu => "RANDU LCG (15 planes)",
        }
    }

    fn reset(&mut self) {
        self.state = self.initial_state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_preset_and_seed_reproduce() {
        for preset in [LcgPreset::Glibc, LcgPreset::Randu] {
            let mut first = LcgSource::new(preset, 1234);
            let mut second = LcgSource::new(preset, 1234);

            let mut a = [0u8; 64];
            let mut b = [0u8; 64];
            first.fill_bytes(&mut a);
            second.fill_bytes(&mut b);

            assert_eq!(a, b, "{preset:?}");
        }
    }

    #[test]
    fn presets_diverge_for_the_same_seed() {
        let mut glibc = LcgSource::new(LcgPreset::Glibc, 1234);
        let mut randu = LcgSource::new(LcgPreset::Randu, 1234);

        let mut a = [0u8; 64];
        let mut b = [0u8; 64];
        glibc.fill_bytes(&mut a);
        randu.fill_bytes(&mut b);

        assert_ne!(a, b);
    }

    #[test]
    fn reset_rewinds_the_sequence() {
        let mut source = LcgSource::new(LcgPreset::Glibc, 5);

        let mut first = [0u8; 32];
        let mut second = [0u8; 32];
        source.fill_bytes(&mut first);
        source.reset();
        source.fill_bytes(&mut second);

        assert_eq!(first, second);
    }

    #[test]
    fn randu_seed_zero_does_not_get_stuck() {
        let mut source = LcgSource::new(LcgPreset::Randu, 0);
        let mut buffer = [0u8; 32];
        source.fill_bytes(&mut buffer);

        assert!(buffer.iter().any(|&b| b != 0));
    }

    #[test]
    fn glibc_uses_many_byte_values() {
        let mut source = LcgSource::new(LcgPreset::Glibc, 2024);
        let mut buffer = [0u8; 10_000];
        source.fill_bytes(&mut buffer);

        let mut seen = [false; 256];
        for &byte in &buffer {
            seen[byte as usize] = true;
        }
        let distinct = seen.iter().filter(|&&s| s).count();

        assert!(distinct > 200, "only {distinct} distinct byte values");
    }

    #[test]
    fn preset_labels_and_names() {
        assert_eq!(LcgPreset::Glibc.label(), "LCG (glibc)");
        assert_eq!(LcgPreset::Randu.label(), "RANDU");
        assert!(LcgSource::new(LcgPreset::Glibc, 1).name().contains("glibc"));
        assert!(LcgSource::new(LcgPreset::Randu, 1).name().contains("RANDU"));
        assert_eq!(
            LcgSource::new(LcgPreset::Randu, 1).preset(),
            LcgPreset::Randu
        );
    }
}
