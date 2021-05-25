use defmt::Format;
use packed_struct::prelude::*;

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum PaSelect {
    Rfo = 0b0,
    PaBoost = 0b1,
}

#[derive(Debug, PackedStruct)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "1")]
pub struct PaConfig {
    #[packed_field(bits = "7", ty = "enum")]
    pub pa_select: PaSelect,
    #[packed_field(bits = "6:4", endian = "msb")]
    max_power: Integer<u8, packed_bits::Bits3>,
    #[packed_field(bits = "3:0", endian = "msb")]
    output_power: Integer<u8, packed_bits::Bits4>,
}
