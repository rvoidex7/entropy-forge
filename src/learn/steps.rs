//! Byte-level XOR visualization steps.
//!
//! The cipher operates on **UTF-8 bytes**, never on `char as u8`. A single
//! Unicode character such as `ş` is two bytes; the visualization therefore
//! produces one step per byte and records which character that byte belongs
//! to.

/// A single bit XOR operation, displayed MSB-first.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitOperation {
    pub input_bit: bool,
    pub key_bit: bool,
    pub result_bit: bool,
    /// Bit position in the byte (7 = most significant, 0 = least significant).
    pub position: usize,
}

/// One UTF-8 byte being encrypted.
#[derive(Debug, Clone)]
pub struct EncryptionStep {
    /// The Unicode character this byte belongs to.
    pub character: char,
    /// Byte offset inside the character's UTF-8 encoding (0-based).
    pub byte_offset: usize,
    /// The input byte (a UTF-8 byte, not a truncated `char`).
    pub input_byte: u8,
    pub keystream_byte: u8,
    pub result_byte: u8,
    pub bit_ops: Vec<BitOperation>,
}

impl EncryptionStep {
    /// Create a step from a raw UTF-8 byte and its keystream byte.
    pub fn new(input_byte: u8, keystream_byte: u8, character: char, byte_offset: usize) -> Self {
        let result_byte = input_byte ^ keystream_byte;

        let mut bit_ops = Vec::with_capacity(8);
        for position in (0..8).rev() {
            let mask = 1 << position;
            bit_ops.push(BitOperation {
                input_bit: (input_byte & mask) != 0,
                key_bit: (keystream_byte & mask) != 0,
                result_bit: (result_byte & mask) != 0,
                position,
            });
        }

        Self {
            character,
            byte_offset,
            input_byte,
            keystream_byte,
            result_byte,
            bit_ops,
        }
    }

    /// Human-readable label, e.g. `'ş' byte 1/2`.
    pub fn character_label(&self) -> String {
        let width = self.character.len_utf8();
        if width > 1 {
            format!(
                "'{}' byte {}/{}",
                self.character,
                self.byte_offset + 1,
                width
            )
        } else {
            format!("'{}'", self.character)
        }
    }

    pub fn input_binary(&self) -> String {
        format!("{:08b}", self.input_byte)
    }

    pub fn keystream_binary(&self) -> String {
        format!("{:08b}", self.keystream_byte)
    }

    pub fn result_binary(&self) -> String {
        format!("{:08b}", self.result_byte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multibyte_character_is_not_truncated() {
        let step = EncryptionStep::new(0xC5, 0x00, 'ş', 0);
        assert_eq!(step.input_byte, 0xC5);
        assert_eq!(step.input_binary(), "11000101");
        assert_eq!(step.character_label(), "'ş' byte 1/2");
    }
}
