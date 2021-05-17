use packed_struct::prelude::*;

#[derive(Debug, PackedStruct)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
pub struct FrequencyDeviation {
    #[packed_field(bits = "0:13", endian = "msb")]
    pub rate: Integer<u16, packed_bits::Bits14>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bits() {
        let packed = [0xAB, 0xCD];
        let frequency_deviation = FrequencyDeviation::unpack(&packed).unwrap();
        let rate: u16 = frequency_deviation.rate.into();
        assert_eq!(rate, 0x2BCD);
    }
}
