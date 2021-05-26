use super::frequency_rf::{FrequencyRf, F_OSC};
pub use super::modem_config1::Bandwidth;
pub use super::ppm_correction::PpmCorrection;
use crate::size_bytes::SizeBytes;
use crate::start_address::StartAddress;
use crate::LoraRegisters;
use packed_struct::prelude::*;
pub use uom::si::frequency::{hertz, kilohertz, megahertz};
pub use uom::si::i32::Frequency as FrequencyI32;
pub use uom::si::u32::Frequency as FrequencyU32;

impl SizeBytes for Fei {
    const SIZE: usize = 3;
}

/// Modify only in Sleep mode.
#[derive(Debug, PackedStruct)]
#[packed_struct(size_bytes = "3", bit_numbering = "lsb0")]
pub struct Fei {
    #[packed_field(bits = "0:19", endian = "msb")]
    pub error: Integer<i32, packed_bits::Bits20>,
}

impl Fei {
    pub fn err_hz(&self, bandwidth: &Bandwidth) -> FrequencyI32 {
        let bandwidth_freq: FrequencyU32 = (*bandwidth).into();
        let bandwidth_hz = bandwidth_freq.get::<kilohertz>() as f64;
        let error: i32 = self.error.into();
        let error_hz = ((((error as i64) << 24)as f64) / (F_OSC as f64)) * (bandwidth_hz / 500f64);

        FrequencyI32::new::<hertz>(error_hz as i32)
    }

    pub fn frf_new(&self, frf_current: &FrequencyRf, bandwidth: &Bandwidth) -> FrequencyRf {
        let err_hz = self.err_hz(bandwidth).get::<hertz>();
        defmt::trace!("ErrorHz was {:?}", err_hz);

        let frf_current: FrequencyU32 = frf_current.clone().into();
        let frf_current_hz: i32 = frf_current.get::<hertz>() as i32;

        let frf_new = frf_current_hz - err_hz;
        defmt::trace!("frf_current was {:?}", frf_current_hz);
        defmt::trace!("frf_new was {:?}", frf_new);

        let frf_new = FrequencyU32::new::<hertz>(frf_new as u32);

        frf_new.into()
    }

    pub fn ppm_correction(&self, frf_current: &FrequencyRf, bandwidth: &Bandwidth) -> PpmCorrection{
        let err_hz = self.err_hz(bandwidth).get::<hertz>() as f64;
        let frf: FrequencyU32 = (*frf_current).clone().into();
        let err_ppm: f64 = err_hz * (1_000_000f64 / frf.get::<hertz>() as f64);
        defmt::trace!("err_ppm : {:?}", err_ppm as f32);

        let correction = (0.95f64 * err_ppm) as i8;
        defmt::trace!("ppm correction: {:?}", correction);

        PpmCorrection{
            correction: correction.into()
        }
    }
}

impl StartAddress for Fei {
    fn start_address() -> LoraRegisters {
        LoraRegisters::FeiMsb
    }
}

