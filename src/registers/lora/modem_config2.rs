use defmt::Format;
use packed_struct::prelude::*;

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum SpreadingFactor {
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
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
pub struct ModemConfig2 {
    #[packed_field(bits = "7:4", ty = "enum")]
    pub spreading_factor: SpreadingFactor,
    #[packed_field(bits = "3")]
    pub tx_continuous_mode: bool,
    #[packed_field(bits = "2")]
    pub rx_payload_crc_on: bool,
    #[packed_field(bits = "1:0", endian = "msb")]
    symb_timeout_msb: Integer<u8, packed_bits::Bits2>,
}
