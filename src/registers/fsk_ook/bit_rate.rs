use packed_struct::prelude::*;

#[derive(PackedStruct)]
#[packed_struct(endian="msb")]
pub struct BitRate{
    rate: Integer<u16, packed_bits::Bits16>
} 

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bits() {
        let packed = [0xAB, 0xCD];
        let bit_rate = BitRate::unpack(&packed).unwrap();
        let rate: u16 = bit_rate.rate.into();
        assert_eq!(rate, 0xABCD);
    }

}
