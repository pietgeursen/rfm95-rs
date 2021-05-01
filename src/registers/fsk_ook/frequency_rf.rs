use packed_struct::prelude::*;

#[derive(Debug, PackedStruct)]
#[packed_struct(size_bytes="3", bit_numbering="lsb0")]
pub struct FrequencyRf{
    #[packed_field(bits="0:23", endian="msb")]
    pub rate: Integer<u32, packed_bits::Bits24>
} 

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bits() {
        let packed = [0xAB, 0xCD, 0xEF];
        let frequency_deviation = FrequencyRf::unpack(&packed).unwrap();
        let rate: u32 = frequency_deviation.rate.into();
        assert_eq!(rate, 0xABCDEF);
    }

}
