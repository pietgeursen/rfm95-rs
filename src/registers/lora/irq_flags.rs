use crate::start_address::StartAddress;
use crate::LoraRegisters;
use defmt::Format;
use packed_struct::prelude::*;

use crate::size_bytes::SizeBytes;

impl SizeBytes for IrqFlags {
    const SIZE: usize = 1;
}

#[derive(Debug, Format, PackedStruct)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "1")]
pub struct IrqFlags {
    #[packed_field(bits = "7")]
    pub rx_timeout: bool,
    #[packed_field(bits = "6")]
    pub rx_done: bool,
    #[packed_field(bits = "5")]
    pub payload_crc_error: bool,
    #[packed_field(bits = "4")]
    pub valid_header: bool,
    #[packed_field(bits = "3")]
    pub tx_done: bool,
    #[packed_field(bits = "2")]
    pub cad_done: bool,
    #[packed_field(bits = "1")]
    pub frequency_hop_change_channel: bool,
    #[packed_field(bits = "0")]
    pub cad_detected: bool,
}

impl StartAddress for IrqFlags {
    fn start_address() -> LoraRegisters {
        LoraRegisters::IrqFlags
    }
}
