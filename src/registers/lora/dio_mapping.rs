use crate::start_address::StartAddress;
use crate::size_bytes::SizeBytes;
use crate::LoraRegisters;
use defmt::Format;
use packed_struct::prelude::*;

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Dio0Mapping {
    RxDone = 0b00,
    TxDone = 0b01,
    CadDone = 0b10,
}

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Dio1Mapping {
    RxTimeout = 0b00,
    FhssChannelChange = 0b01,
    CadDetected = 0b10,
}

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Dio2Mapping {
    FhssChannelChange = 0b00,
}

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Dio3Mapping {
    CadDone = 0b00,
    ValidHeader = 0b01,
    PayloadCrcError = 0b10,
}

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Dio4Mapping {
    CadDone = 0b00,
    PllLock = 0b01,
}

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Dio5Mapping {
    ModeReady = 0b00,
    ClkOut = 0b01,
}

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum MapPreambleDetect {
    Rssi = 0b0,
    PreambleDetect = 0b1,
}

impl SizeBytes for DioMapping {
    const SIZE: usize = 2;
}

#[derive(Debug, PackedStruct)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "2")]
pub struct DioMapping {
    #[packed_field(bits = "15:14", ty = "enum")]
    pub dio_0_mapping: Dio0Mapping,
    #[packed_field(bits = "13:12", ty = "enum")]
    pub dio_1_mapping: Dio1Mapping,
    #[packed_field(bits = "11:10", ty = "enum")]
    pub dio_2_mapping: Dio2Mapping,
    #[packed_field(bits = "9:8", ty = "enum")]
    pub dio_3_mapping: Dio3Mapping,
    #[packed_field(bits = "7:6", ty = "enum")]
    pub dio_4_mapping: Dio4Mapping,
    #[packed_field(bits = "5:4", ty = "enum")]
    pub dio_5_mapping: Dio5Mapping,
    #[packed_field(bits = "3:1")]
    pub _reserved: ReservedZero<packed_bits::Bits3>,
    #[packed_field(bits = "0", ty = "enum")]
    map_preamble_detect: MapPreambleDetect,
}

impl StartAddress for DioMapping {
    fn start_address() -> LoraRegisters {
        LoraRegisters::DioMapping1
    }
}

