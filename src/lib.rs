//! rfm95-rs
//!
//! `no_std` compatible spi driver for the
//! [rfm95c](https://www.hoperf.com/modules/lora/RFM95.html) LoRa radio.
//!
//! ## Status
//!
#![no_std]
#![forbid(unsafe_code)]

use core::fmt::Debug;
use core::marker::PhantomData;
use embedded_hal as hal;
use hal::blocking::spi::{Transfer, Write};
use hal::digital::v2::OutputPin;
use packed_struct::prelude::*;
use snafu::{ensure, OptionExt, ResultExt};

pub mod config;
pub mod error;
mod internal;
pub mod registers;

pub use config::*;
pub use error::*;
pub use registers::fsk_ook::op_mode::*;
pub use registers::*;

use internal::*;

pub struct FskOokMode;
pub struct LoRaMode;

pub struct RFM95<SPI, Mode, OutputPin> {
    config: Config,
    chip_select: OutputPin,
    mode: PhantomData<Mode>,
    spi: PhantomData<SPI>,
}

// Magic number that must be written to the reset register to actually trigger a reset.
const RESET_WORD: u8 = 0xB6;
const DEVICE_ID: u8 = 0x58;

impl<SPI, SpiError, Mode, ChipSelectPin, ChipSelectPinError> RFM95<SPI, Mode, ChipSelectPin>
where
    SPI: Transfer<u8, Error = SpiError> + Write<u8, Error = SpiError>,
    SpiError: Debug,
    ChipSelectPin: OutputPin<Error = ChipSelectPinError>,
{
    fn _configure(spi: &mut SPI, config: &Config) -> Result<(), Error<SpiError>> {
        unimplemented!();
    }

    fn write_to_address(
        spi: &mut SPI,
        chip_select: &mut ChipSelectPin,
        address: u8,
        byte: u8,
    ) -> Result<(), Error<SpiError>> {
        let buffer = [address | 0b10000000, byte];

        chip_select.set_low()
            .map_err(|_| Error::SetPin)?;
        spi.write(&buffer)
            .map_err(|spi_err| Error::SpiRead { spi_err })?;
        chip_select.set_high()
            .map_err(|_| Error::SetPin)?;
        Ok(())
    }

    fn read_register(
        spi: &mut SPI,
        chip_select: &mut ChipSelectPin,
        address: u8,
    ) -> Result<u8, Error<SpiError>> {
        let mut buffer = [address, 0];
        chip_select.set_low().map_err(|_| Error::SetPin)?;

        spi.transfer(&mut buffer)
            .map_err(|spi_err| Error::SpiTransfer { spi_err })?;

        chip_select.set_high().map_err(|_| Error::SetPin)?;

        Ok(buffer[1])
    }
}

impl<SPI, SpiError, ChipSelectPin> RFM95<SPI, FskOokMode, ChipSelectPin>
where
    SPI: Transfer<u8, Error = SpiError> + Write<u8, Error = SpiError>,
    SpiError: Debug,
    ChipSelectPin: OutputPin,
{
    pub fn new(
        spi: &mut SPI,
        mut chip_select: ChipSelectPin,
        config: Config,
    ) -> Result<Self, Error<SpiError>> {

        let internal_op_mode = Self::read_register(spi, &mut chip_select, FskOokRegisters::OpMode.addr()).unwrap();
        let internal_op_mode = OpMode::unpack(&[internal_op_mode]).unwrap();
        defmt::info!("internal_op_mode modem_mode: {:?}, modulation_type: {:?}, mode{:?}, low_frequency_mode: {:?}", internal_op_mode.modem_mode, internal_op_mode.modulation_type, internal_op_mode.mode, internal_op_mode.low_frequency_mode);

        let op_mode = OpMode {
            mode: Mode::Transmitter,
            _reserved: ReservedZero::default(),
            low_frequency_mode: false,
            modulation_type: ModulationType::Fsk,
            modem_mode: ModemMode::FskOok,
        };

        let packed_op_mode = op_mode.pack().unwrap()[0];

        Self::write_to_address(
            spi,
            &mut chip_select,
            FskOokRegisters::OpMode.addr(),
            packed_op_mode,
        )
        .map_err(|_| Error::Todo)?;

        Self::write_to_address(spi, &mut chip_select, FskOokRegisters::Fifo.addr(), 0xAA)
            .map_err(|_| Error::Todo)?;

        let rfm95 = RFM95 {
            spi: PhantomData,
            chip_select,
            config,
            mode: PhantomData,
        };

        Ok(rfm95)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use embedded_hal_mock::i2c::Mock as SPIMock;

    #[test]
    fn temperature_calibration() {}

    #[test]
    fn pressure_calibration() {}
}
