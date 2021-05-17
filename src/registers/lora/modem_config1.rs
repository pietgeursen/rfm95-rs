use defmt::Format;
use packed_struct::prelude::*;

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Bandwidth {
    K7_5 = 0b000,
    K10_4 = 0b001,
    K15_6 = 0b010,
    K20_8 = 0b011,
    K31_25 = 0b100,
    K41_7 = 0b101,
    K62_5 = 0b110,
    K125 = 0b111,
    K250 = 0b1000,
    K500 = 0b1001,
}

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum CodingRate {
    Rate4_5 = 0b001,
    Rate4_6 = 0b010,
    Rate4_7 = 0b011,
    Rate4_8 = 0b100,
}

#[derive(Debug, PackedStruct)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "1")]
pub struct ModemConfig1 {
    #[packed_field(bits = "7:4", ty = "enum")]
    pub bandwidth: Bandwidth,
    #[packed_field(bits = "3:1", ty = "enum")]
    pub coding_rate: CodingRate,
    #[packed_field(bits = "0")]
    pub implicit_header_mode_on: bool,
}
