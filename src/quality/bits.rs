//! MSB-first bit access helpers.
//!
//! NIST SP 800-22 treats a byte sequence as a bit sequence in **MSB-first**
//! order: the most significant bit (bit 7) of each byte comes first. All
//! quality tests in this crate must use these helpers instead of building
//! `Vec<u8>` bit vectors, which would inflate memory usage by 8x.

/// Number of bits in a byte slice.
#[inline]
pub fn bit_len(data: &[u8]) -> usize {
    data.len() * 8
}

/// Read a single bit in MSB-first order without allocating.
///
/// Bit index `0` is the most significant bit of `data[0]`.
///
/// # Panics
///
/// Panics if `bit_idx` is out of bounds. Callers that work with a logical
/// bit length shorter than `bit_len(data)` must check the bound first.
#[inline]
pub fn bit_at_msb(data: &[u8], bit_idx: usize) -> u8 {
    (data[bit_idx >> 3] >> (7 - (bit_idx & 7))) & 1
}

/// Read a bit in MSB-first order, returning `0` when out of bounds.
///
/// This is the safe variant used by circular/wrapping NIST tests.
#[inline]
pub fn bit_at_msb_or_zero(data: &[u8], bit_idx: usize) -> u8 {
    let byte = bit_idx >> 3;
    if byte >= data.len() {
        0
    } else {
        (data[byte] >> (7 - (bit_idx & 7))) & 1
    }
}

/// Iterator over the bits of a byte slice in MSB-first order.
///
/// The iterator is zero-allocation: it only keeps a reference to the slice and
/// the current bit index.
#[derive(Debug, Clone)]
pub struct BitsMsb<'a> {
    data: &'a [u8],
    bit: usize,
    len: usize,
}

impl<'a> BitsMsb<'a> {
    /// Iterate over every bit of `data`.
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            bit: 0,
            len: bit_len(data),
        }
    }

    /// Iterate over the first `len` bits of `data`.
    ///
    /// `len` is clamped to `bit_len(data)`.
    pub fn with_len(data: &'a [u8], len: usize) -> Self {
        Self {
            data,
            bit: 0,
            len: len.min(bit_len(data)),
        }
    }
}

impl Iterator for BitsMsb<'_> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.bit >= self.len {
            return None;
        }
        let value = bit_at_msb(self.data, self.bit);
        self.bit += 1;
        Some(value)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len - self.bit;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for BitsMsb<'_> {}

/// Convert bytes to an MSB-first bit vector.
///
/// This allocates and is intended for educational/UI code only. Quality
/// analysis should use [`bit_at_msb`] or [`BitsMsb`] instead.
pub fn bytes_to_bits_msb(data: &[u8]) -> Vec<u8> {
    BitsMsb::new(data).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn msb_first_reads_high_bit_first() {
        let data = [0b1000_0001u8];
        assert_eq!(bit_at_msb(&data, 0), 1);
        assert_eq!(bit_at_msb(&data, 1), 0);
        assert_eq!(bit_at_msb(&data, 7), 1);
        assert_eq!(bytes_to_bits_msb(&data), vec![1, 0, 0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn iterator_respects_logical_length() {
        let data = [0xFFu8, 0x00];
        let bits: Vec<u8> = BitsMsb::with_len(&data, 10).collect();
        assert_eq!(bits.len(), 10);
        assert_eq!(&bits[..8], &[1; 8]);
        assert_eq!(&bits[8..], &[0, 0]);
    }

    #[test]
    fn out_of_bounds_is_zero() {
        let data = [0xFFu8];
        assert_eq!(bit_at_msb_or_zero(&data, 8), 0);
    }
}
