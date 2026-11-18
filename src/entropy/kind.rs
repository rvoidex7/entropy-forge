//! The selectable set of entropy sources shown in the RNG Arena.

use super::{
    BiasedSource, ChaCha20Source, CounterSource, EntropySource, LcgPreset, LcgSource, Lfsr16Source,
    SystemEntropy,
};

/// Seed used for every deterministic source in the arena.
///
/// A fixed seed makes a run reproducible: the same source always produces the
/// same bytes, so a screenshot or a bug report can be compared against a fresh
/// run. The system source ignores it.
pub const ARENA_SEED: u64 = 0x5EED_1234_5678_9ABC;

/// Which entropy source the app should use.
///
/// `SourceKind` is deliberately `Copy` and holds no generator state, so it can
/// be moved into a worker thread where the actual source is built.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SourceKind {
    /// Operating-system CSPRNG. The reference source.
    #[default]
    System,
    /// Seeded ChaCha20 CSPRNG. Deterministic, statistically sound.
    ChaCha20,
    /// glibc's LCG constants.
    Lcg,
    /// IBM RANDU, the classic bad LCG.
    Randu,
    /// 16-bit maximal-length LFSR.
    Lfsr16,
    /// Independent bits with an adjustable `P(1)`.
    Biased,
    /// `0, 1, 2, ..., 255`.
    Counter,
}

impl SourceKind {
    /// Every source, in the order the selector shows them.
    pub const ALL: [Self; 7] = [
        Self::System,
        Self::ChaCha20,
        Self::Lcg,
        Self::Randu,
        Self::Lfsr16,
        Self::Biased,
        Self::Counter,
    ];

    /// Short label for the selector.
    pub const fn label(self) -> &'static str {
        match self {
            Self::System => "System",
            Self::ChaCha20 => "ChaCha20",
            Self::Lcg => "LCG (glibc)",
            Self::Randu => "RANDU",
            Self::Lfsr16 => "LFSR-16",
            Self::Biased => "Biased coin",
            Self::Counter => "Counter",
        }
    }

    /// One-line explanation of what this source is and where it fails.
    pub const fn description(self) -> &'static str {
        match self {
            Self::System => "OS CSPRNG — the reference source. Should pass every test.",
            Self::ChaCha20 => {
                "Seeded ChaCha20 CSPRNG. Deterministic for a fixed seed, statistically sound."
            }
            Self::Lcg => "glibc LCG. Uniform-looking, structurally broken: lattices appear in 2D.",
            Self::Randu => "IBM RANDU LCG. Infamous for its 15 planes in three dimensions.",
            Self::Lfsr16 => "16-bit LFSR. Linear, with a 65,535-byte period.",
            Self::Biased => {
                "Biased coin. The frequency test fails as soon as P(1) drifts from 0.5."
            }
            Self::Counter => "0, 1, 2, … 255. A perfect histogram with zero predictability.",
        }
    }

    /// Whether the selector should show the bias slider for this source.
    pub const fn has_bias_control(self) -> bool {
        matches!(self, Self::Biased)
    }

    /// Build a fresh source. `bias` is only used by [`Self::Biased`].
    pub fn build(self, seed: u64, bias: f64) -> Box<dyn EntropySource> {
        match self {
            Self::System => Box::new(SystemEntropy::new()),
            Self::ChaCha20 => Box::new(ChaCha20Source::new(seed)),
            Self::Lcg => Box::new(LcgSource::new(LcgPreset::Glibc, seed)),
            Self::Randu => Box::new(LcgSource::new(LcgPreset::Randu, seed)),
            Self::Lfsr16 => Box::new(Lfsr16Source::new(seed as u16)),
            Self::Biased => Box::new(BiasedSource::new(bias)),
            Self::Counter => Box::new(CounterSource::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_kind_builds_and_fills_a_buffer() {
        for kind in SourceKind::ALL {
            let mut source = kind.build(ARENA_SEED, 0.75);
            let mut buffer = [0u8; 128];
            source.fill_bytes(&mut buffer);

            assert!(!source.name().is_empty(), "{kind:?} has no name");
        }
    }

    #[test]
    fn labels_are_unique() {
        let mut labels: Vec<&str> = SourceKind::ALL.iter().map(|k| k.label()).collect();
        labels.sort_unstable();
        let count = labels.len();
        labels.dedup();

        assert_eq!(labels.len(), count);
    }

    #[test]
    fn descriptions_are_present() {
        for kind in SourceKind::ALL {
            assert!(
                !kind.description().is_empty(),
                "{kind:?} has no description"
            );
        }
    }

    #[test]
    fn only_biased_has_a_bias_control() {
        for kind in SourceKind::ALL {
            assert_eq!(
                kind.has_bias_control(),
                kind == SourceKind::Biased,
                "{kind:?}"
            );
        }
    }

    #[test]
    fn counter_kind_counts_from_zero() {
        let mut source = SourceKind::Counter.build(ARENA_SEED, 0.75);
        let mut buffer = [0u8; 4];
        source.fill_bytes(&mut buffer);

        assert_eq!(buffer, [0, 1, 2, 3]);
    }

    #[test]
    fn deterministic_kinds_reproduce_after_reset() {
        for kind in [
            SourceKind::ChaCha20,
            SourceKind::Lcg,
            SourceKind::Randu,
            SourceKind::Lfsr16,
            SourceKind::Biased,
            SourceKind::Counter,
        ] {
            let mut source = kind.build(ARENA_SEED, 0.8);
            let mut first = [0u8; 64];
            let mut second = [0u8; 64];
            source.fill_bytes(&mut first);
            source.reset();
            source.fill_bytes(&mut second);

            assert_eq!(first, second, "{kind:?} is not reproducible");
        }
    }

    #[test]
    fn bias_parameter_reaches_the_biased_source() {
        let mut all_ones = SourceKind::Biased.build(ARENA_SEED, 1.0);
        let mut buffer = [0u8; 16];
        all_ones.fill_bytes(&mut buffer);
        assert!(buffer.iter().all(|&b| b == 0xFF));

        let mut all_zeros = SourceKind::Biased.build(ARENA_SEED, 0.0);
        all_zeros.fill_bytes(&mut buffer);
        assert!(buffer.iter().all(|&b| b == 0x00));
    }

    #[test]
    fn default_is_the_system_source() {
        assert_eq!(SourceKind::default(), SourceKind::System);
    }
}
