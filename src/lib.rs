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
use registers::lora::frequency_rf::*;
use snafu::{ensure, OptionExt, ResultExt};

pub mod config;
pub mod error;
mod internal;
pub mod registers;

use arrayvec::*;
pub use config::*;
pub use error::*;
pub use registers::lora::modem_config2::*;
pub use registers::lora::op_mode::*;
use registers::lora::*;
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

        chip_select.set_low().map_err(|_| Error::SetPin)?;
        spi.write(&buffer)
            .map_err(|spi_err| Error::SpiRead { spi_err })?;
        chip_select.set_high().map_err(|_| Error::SetPin)?;
        Ok(())
    }

    fn read_register(
        spi: &mut SPI,
        chip_select: &mut ChipSelectPin,
        address: u8,
    ) -> Result<u8, Error<SpiError>> {
        let mut buffer = [0];

        Self::read_registers(spi, chip_select, address, &mut buffer)?;

        Ok(buffer[0])
    }

    fn write_registers(
        spi: &mut SPI,
        chip_select: &mut ChipSelectPin,
        address: u8,
        buffer: &mut [u8],
    ) -> Result<(), Error<SpiError>> {
        chip_select.set_low().map_err(|_| Error::SetPin)?;

        let mut write_command = [address | 0b10000000];

        spi.transfer(&mut write_command)
            .map_err(|spi_err| Error::SpiTransfer { spi_err })?;

        spi.transfer(buffer)
            .map_err(|spi_err| Error::SpiTransfer { spi_err })?;

        chip_select.set_high().map_err(|_| Error::SetPin)?;

        Ok(())
    }

    fn read_registers(
        spi: &mut SPI,
        chip_select: &mut ChipSelectPin,
        address: u8,
        buffer: &mut [u8],
    ) -> Result<(), Error<SpiError>> {
        chip_select.set_low().map_err(|_| Error::SetPin)?;

        let mut read_command = [address];

        spi.transfer(&mut read_command)
            .map_err(|spi_err| Error::SpiTransfer { spi_err })?;

        spi.transfer(buffer)
            .map_err(|spi_err| Error::SpiTransfer { spi_err })?;

        chip_select.set_high().map_err(|_| Error::SetPin)?;

        Ok(())
    }
}

impl<SPI, SpiError, ChipSelectPin> RFM95<SPI, LoRaMode, ChipSelectPin>
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
        let op_mode = OpMode {
            mode: Mode::Sleep,
            access_shared_registers: AccessSharedRegisters::AccessLora,
            _reserved: ReservedZero::default(),
            low_frequency_mode: false,
            modem_mode: ModemMode::LoRa,
        };

        let packed_op_mode = op_mode.pack().unwrap()[0];

        Self::write_to_address(
            spi,
            &mut chip_select,
            LoraRegisters::OpMode.addr(),
            packed_op_mode,
        )
        .map_err(|_| Error::Todo)?;

        let internal_op_mode =
            Self::read_register(spi, &mut chip_select, LoraRegisters::OpMode.addr()).unwrap();
        let internal_op_mode = OpMode::unpack(&[internal_op_mode]).unwrap();

        defmt::info!(
            "internal_op_mode modem_mode: {:?}",
            internal_op_mode.modem_mode
        );

        let frequency = Self::get_frequency(spi, &mut chip_select)?;
        defmt::info!("default frequency: {:?}", frequency.get::<hertz>());

        Self::set_frequency(spi, &mut chip_select, Frequency::new::<kilohertz>(915000))?;

        let frequency = Self::get_frequency(spi, &mut chip_select)?;
        defmt::info!("updated frequency: {:?}", frequency.get::<hertz>());

        Self::set_carrier_test_on(spi, &mut chip_select)?;

        let rfm95 = RFM95 {
            spi: PhantomData,
            chip_select,
            config,
            mode: PhantomData,
        };

        Ok(rfm95)
    }

    pub fn get_frequency(
        spi: &mut SPI,
        chip_select: &mut ChipSelectPin,
    ) -> Result<Frequency, Error<SpiError>> {
        let mut buffer = [0u8; 3];

        Self::read_registers(spi, chip_select, LoraRegisters::FrfMsb.addr(), &mut buffer)?;
        let frequency_rf = frequency_rf::FrequencyRf::unpack(&buffer).unwrap();

        Ok(frequency_rf.into())
    }

    pub fn set_frequency(
        spi: &mut SPI,
        chip_select: &mut ChipSelectPin,
        frequency: Frequency,
    ) -> Result<(), Error<SpiError>> {
        let register: FrequencyRf = frequency.into();
        let mut packed = register.pack().unwrap();

        Self::write_registers(spi, chip_select, LoraRegisters::FrfMsb.addr(), &mut packed)
    }

    pub fn set_carrier_test_on(
        spi: &mut SPI,
        chip_select: &mut ChipSelectPin,
    ) -> Result<(), Error<SpiError>> {
        let mut data = [1,2,3,5];

        Self::set_continuous_transmission(spi, chip_select, true)?;
        Self::set_opmode_mode(spi, chip_select, Mode::Standby)?;
        Self::write_tx_fifo(spi, chip_select, &mut data)?;
        Self::set_opmode_mode(spi, chip_select, Mode::Transmitter)?;

        Ok(())
    }

    pub fn set_opmode_mode(
        spi: &mut SPI,
        chip_select: &mut ChipSelectPin,
        mode: Mode,
    ) -> Result<(), Error<SpiError>> {
        let op_mode_byte =
            Self::read_register(spi, chip_select, LoraRegisters::OpMode.addr()).unwrap();
        let mut op_mode = OpMode::unpack(&[op_mode_byte]).unwrap();

        op_mode.mode = mode;

        let mut packed = op_mode.pack().unwrap();
        Self::write_registers(spi, chip_select, LoraRegisters::OpMode.addr(), &mut packed)
    }

    pub fn set_continuous_transmission(
        spi: &mut SPI,
        chip_select: &mut ChipSelectPin,
        continuous_transmission_enabled: bool,
    ) -> Result<(), Error<SpiError>> {
        let op_mode_byte =
            Self::read_register(spi, chip_select, LoraRegisters::ModemConfig2.addr()).unwrap();
        let mut config = ModemConfig2::unpack(&[op_mode_byte]).unwrap();

        config.tx_continuous_mode = continuous_transmission_enabled;

        let mut packed = config.pack().unwrap();
        Self::write_registers(
            spi,
            chip_select,
            LoraRegisters::ModemConfig2.addr(),
            &mut packed,
        )
    }

    pub fn write_tx_fifo(
        spi: &mut SPI,
        chip_select: &mut ChipSelectPin,
        data: &mut [u8],
    ) -> Result<(), Error<SpiError>> {
        // get the tx_fifo base address
        let tx_fifo_base =
            Self::read_register(spi, chip_select, LoraRegisters::FifoTxBaseAddress.addr())?;

        // set the fifo pointer to the tx base address
        Self::write_registers(
            spi,
            chip_select,
            LoraRegisters::FifoAddrPointer.addr(),
            &mut [tx_fifo_base],
        )?;

        // write the buffer
        Self::write_registers(spi, chip_select, LoraRegisters::Fifo.addr(), data)
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
