use defmt::Format;
use packed_struct::prelude::*;

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum AGC {
    Off = 0b0,
    On = 0b1
}

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum LowDataRateOptimize {
    Off = 0b0,
    On = 0b1
}


#[derive(Debug, PackedStruct)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "1")]
pub struct ModemConfig3 {
    #[packed_field(bits = "3", ty = "enum")]
    pub low_data_rate_optimize: LowDataRateOptimize,
    #[packed_field(bits = "2", ty = "enum")]
    pub agc: AGC,
}
