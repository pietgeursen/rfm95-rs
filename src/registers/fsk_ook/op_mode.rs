use packed_struct::prelude::*;
use defmt::Format;

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Mode{
    Sleep = 0b000,
    Standby = 0b001,
    FSTx = 0b010,
    Transmitter = 0b011,
    FSRx = 0b100,
    Receiver = 0b101,
} 

#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum ModulationType {
    Fsk = 0b00,
    Ook = 0b01
}

// TODO: this can only be changed in Mode::Sleep
#[derive(Format, PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum ModemMode{
    FskOok = 0b0,
    LoRa = 0b1,
}

#[derive(Debug, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes = "1")]
pub struct OpMode{
    #[packed_field(bits="7", ty="enum")]
    pub modem_mode: ModemMode,
    #[packed_field(bits="5:6", ty="enum")]
    pub modulation_type: ModulationType,
    #[packed_field(bits="4")]
    pub _reserved: ReservedZero<packed_bits::Bits1>,
    #[packed_field(bits="3")]
    pub low_frequency_mode: bool,
    #[packed_field(bits="0:2", ty="enum")]
    pub mode: Mode
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bits() {
        let op_mode = OpMode{
            modem_mode: ModemMode::LoRa,
            modulation_type: ModulationType::Fsk,
            _reserved: Default::default(),
            low_frequency_mode: true,
            mode: Mode::Transmitter
        };

        let packed: u8 = op_mode.pack().unwrap()[0];
        assert_eq!(packed, 0b10001011);
    }

}
