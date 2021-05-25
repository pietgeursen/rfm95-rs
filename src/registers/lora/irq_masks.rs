use crate::size_bytes::SizeBytes;
use crate::start_address::StartAddress;
use crate::LoraRegisters;
use defmt::Format;
use packed_struct::prelude::*;

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum IrqMask {
    Enabled = 0b0,
    Masked = 0b1,
}

impl SizeBytes for IrqMasks {
    const SIZE: usize = 1;
}

#[derive(Debug, Format, PackedStruct)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "1")]
pub struct IrqMasks {
    #[packed_field(bits = "7", ty = "enum")]
    pub rx_timeout: IrqMask,
    #[packed_field(bits = "6", ty = "enum")]
    pub rx_done: IrqMask,
    #[packed_field(bits = "5", ty = "enum")]
    pub payload_crc_error: IrqMask,
    #[packed_field(bits = "4", ty = "enum")]
    pub valid_header: IrqMask,
    #[packed_field(bits = "3", ty = "enum")]
    pub tx_done: IrqMask,
    #[packed_field(bits = "2", ty = "enum")]
    pub cad_done: IrqMask,
    #[packed_field(bits = "1", ty = "enum")]
    pub frequency_hop_change_channel: IrqMask,
    #[packed_field(bits = "0", ty = "enum")]
    pub cad_detected: IrqMask,
}

impl StartAddress for IrqMasks {
    fn start_address() -> LoraRegisters {
        LoraRegisters::IrqFlagsMask
    }
}
