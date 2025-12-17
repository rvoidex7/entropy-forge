/// Represents a single bit XOR operation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitOperation {
    pub input_bit: bool,
    pub key_bit: bool,
    pub result_bit: bool,
    pub position: usize, // 0-7, where 0 is MSB or LSB depending on preference (usually MSB 7..0)
}

/// Represents one byte being encrypted
#[derive(Debug, Clone)]
pub struct EncryptionStep {
    pub character: char,
    pub input_byte: u8,
    pub keystream_byte: u8,
    pub result_byte: u8,
    pub bit_ops: Vec<BitOperation>,
}

impl EncryptionStep {
    pub fn new(input_char: char, keystream_byte: u8) -> Self {
        let input_byte = input_char as u8;
        let result_byte = input_byte ^ keystream_byte;

        let mut bit_ops = Vec::with_capacity(8);

        // Process bits from MSB (7) to LSB (0) for display
        for i in (0..8).rev() {
            let mask = 1 << i;
            let input_bit = (input_byte & mask) != 0;
            let key_bit = (keystream_byte & mask) != 0;
            let result_bit = (result_byte & mask) != 0;

            bit_ops.push(BitOperation {
                input_bit,
                key_bit,
                result_bit,
                position: i,
            });
        }

        Self {
            character: input_char,
            input_byte,
            keystream_byte,
            result_byte,
            bit_ops,
        }
    }

    /// Returns the binary string representation of the input byte
    pub fn input_binary(&self) -> String {
        format!("{:08b}", self.input_byte)
    }

    /// Returns the binary string representation of the keystream byte
    pub fn keystream_binary(&self) -> String {
        format!("{:08b}", self.keystream_byte)
    }

    /// Returns the binary string representation of the result byte
    pub fn result_binary(&self) -> String {
        format!("{:08b}", self.result_byte)
    }
}
