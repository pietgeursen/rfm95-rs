use core::convert::From;
use packed_struct::prelude::*;
pub use uom::si::frequency::{hertz, kilohertz, megahertz};
pub use uom::si::u32::Frequency;

/// Modify only in Sleep mode.
#[derive(Debug, PackedStruct)]
#[packed_struct(size_bytes = "3", bit_numbering = "lsb0")]
pub struct FrequencyRf {
    #[packed_field(bits = "0:23", endian = "msb")]
    pub rate: Integer<u32, packed_bits::Bits24>,
}

const F_OSC: u64 = 32_000_000;

impl From<Frequency> for FrequencyRf {
    fn from(frequency: Frequency) -> Self {
        let frequency_hz: u64 = frequency.get::<hertz>() as u64;
        let rate = ((frequency_hz << 19) / F_OSC) as u32;

        FrequencyRf { rate: rate.into() }
    }
}

impl From<FrequencyRf> for Frequency {
    fn from(frequency_rf: FrequencyRf) -> Self {
        let rate: u32 = frequency_rf.rate.into();
        let frequency_hz = (rate as u64 * F_OSC) >> 19; 

        Frequency::new::<hertz>(frequency_hz as u32)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_uom_frequency() {
        let expected = 0x6c8000; // From the datasheet
        let freq = Frequency::new::<kilohertz>(434_000);
        let register: FrequencyRf = freq.into();

        let actual_rate: u32 = register.rate.into();

        assert_eq!(actual_rate, expected);
    }

    #[test]
    fn to_uom_frequency() {
        let expected = Frequency::new::<megahertz>(434);
        let frequency_rf = FrequencyRf {
            rate: 0x6c8000.into(),
        };
        let freq: Frequency = frequency_rf.into();

        assert_eq!(freq, expected);
    }

    #[test]
    fn bits() {
        let packed = [0xAB, 0xCD, 0xEF];
        let frequency = FrequencyRf::unpack(&packed).unwrap();
        let rate: u32 = frequency.rate.into();
        assert_eq!(rate, 0xABCDEF);
    }
}
