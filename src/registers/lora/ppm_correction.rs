use crate::start_address::StartAddress;
use crate::LoraRegisters;
use packed_struct::prelude::*;
pub use uom::si::frequency::{hertz, kilohertz, megahertz};
pub use uom::si::u32::Frequency;
use crate::size_bytes::SizeBytes;

impl SizeBytes for PpmCorrection {
    const SIZE: usize = 1;
}

/// Modify only in Sleep mode.
#[derive(Debug, PackedStruct)]
#[packed_struct(size_bytes = "1", bit_numbering = "lsb0")]
pub struct PpmCorrection {
    #[packed_field(bits = "7:0", endian = "msb")]
    pub correction: Integer<i8, packed_bits::Bits8>,
}

impl StartAddress for PpmCorrection {
    fn start_address() -> LoraRegisters {
        LoraRegisters::PpmCorrection
    }
}
