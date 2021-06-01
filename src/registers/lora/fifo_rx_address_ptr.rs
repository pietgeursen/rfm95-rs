use crate::size_bytes::SizeBytes;
use packed_struct::prelude::*;

impl SizeBytes for FifoRxAddressPtr {
    const SIZE: usize = 1;
}

#[derive(Debug, Clone, PackedStruct)]
#[packed_struct(size_bytes = "1", bit_numbering = "lsb0")]
pub struct FifoRxAddressPtr {
    #[packed_field(bits = "7:0", endian = "msb")]
    pub ptr: Integer<u8, packed_bits::Bits8>,
}
