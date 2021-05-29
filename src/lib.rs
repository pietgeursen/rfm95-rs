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
use embedded_hal::blocking::delay::DelayMs;
use hal::blocking::spi::{Transfer, Write};
use hal::digital::v2::OutputPin;
use packed_struct::{prelude::*, types::bits::ByteArray};
use registers::{lora::frequency_rf::*, start_address::StartAddress};
use registers::{lora::modem_config1::*, size_bytes::SizeBytes};

pub mod error;
mod internal;
pub mod registers;

pub use error::*;
pub use registers::lora::config::Config;
pub use registers::lora::modem_config2::*;
pub use registers::lora::op_mode::*;
use registers::lora::*;
pub use registers::*;

pub struct FskOokMode;
pub struct LoRaMode;

pub struct RFM95<SPI, Mode, ChipSelectPin, ResetPin, DelayMs> {
    config: Config,
    chip_select: ChipSelectPin,
    reset: ResetPin,
    mode: PhantomData<Mode>,
    spi: PhantomData<SPI>,
    delay_ms: DelayMs, //wait_for_irq: IrqNb,
}

impl<SPI, SpiError, Mode, ChipSelectPin, PinError, ResetPin, DelayMs>
    RFM95<SPI, Mode, ChipSelectPin, ResetPin, DelayMs>
where
    SPI: Transfer<u8, Error = SpiError> + Write<u8, Error = SpiError>,
    SpiError: Debug,
    ChipSelectPin: OutputPin<Error = PinError>,
    ResetPin: OutputPin<Error = PinError>,
{
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

impl<SPI, SpiError, ChipSelectPin, ResetPin, PinError, Delay>
    RFM95<SPI, LoRaMode, ChipSelectPin, ResetPin, Delay>
where
    SPI: Transfer<u8, Error = SpiError> + Write<u8, Error = SpiError>,
    SpiError: Debug,
    ChipSelectPin: OutputPin<Error = PinError>,
    ResetPin: OutputPin<Error = PinError>,
    Delay: DelayMs<u32>,
{
    pub fn new(
        _spi: &mut SPI,
        chip_select: ChipSelectPin,
        reset: ResetPin,
        config: Config,
        delay_ms: Delay, //wait_for_irq: IrqNb,
    ) -> Result<Self, Error<SpiError>> {
        let rfm95 = RFM95 {
            spi: PhantomData,
            chip_select,
            reset,
            config: config.clone(),
            mode: PhantomData,
            delay_ms, //wait_for_irq,
        };

        Ok(rfm95)
    }

    pub fn init(&mut self, spi: &mut SPI) -> Result<(), Error<SpiError>> {
        self.reset()?;
        self.set_mode(spi, Mode::Sleep)?;
        self.set_modem_mode(spi, ModemMode::LoRa)?;
        self.set_mode(spi, Mode::Standby)?;
        self.set_low_frequency_mode(spi, self.config.low_frequency_mode)?;
        self.set_bandwidth(spi, self.config.bandwidth)?;
        self.set_coding_rate(spi, self.config.coding_rate)?;
        self.set_implicit_header_mode(spi, self.config.implicit_header_mode_on)?;
        self.set_spreading_factor(spi, self.config.spreading_factor)?;
        self.set_rx_payload_crc_on(spi, self.config.rx_payload_crc_on)?;

        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), Error<SpiError>> {
        self.reset.set_low().map_err(|_| Error::SetPin)?;
        self.delay_ms.delay_ms(1);
        self.reset.set_high().map_err(|_| Error::SetPin)?;
        self.delay_ms.delay_ms(10);
        Ok(())
    }

    pub fn set_mode(&mut self, spi: &mut SPI, mode: Mode) -> Result<(), Error<SpiError>> {
        self.read_update_write_packed_struct::<_, _, { OpMode::SIZE }>(
            spi,
            |op_mode: &mut OpMode| {
                op_mode.mode = mode;
            },
        )
    }
    pub fn get_frequency(&mut self, spi: &mut SPI) -> Result<Frequency, Error<SpiError>> {
        let mut buffer = [0u8; 3];

        Self::read_registers(
            spi,
            &mut self.chip_select,
            LoraRegisters::FrfMsb.addr(),
            &mut buffer,
        )?;
        let frequency_rf = frequency_rf::FrequencyRf::unpack(&buffer).unwrap();

        Ok(frequency_rf.into())
    }

    pub fn set_frequency(
        &mut self,
        spi: &mut SPI,
        frequency: Frequency,
    ) -> Result<(), Error<SpiError>> {
        let register: FrequencyRf = frequency.into();
        let mut packed = register.pack().unwrap();

        Self::write_registers(
            spi,
            &mut self.chip_select,
            LoraRegisters::FrfMsb.addr(),
            &mut packed,
        )
    }

    pub fn set_carrier_test_on(&mut self, spi: &mut SPI) -> Result<(), Error<SpiError>> {
        let mut data = [1, 2, 3, 5];

        self.set_continuous_transmission(spi, true)?;
        self.transmit_data(spi, &mut data)?;
        Ok(())
    }

    pub fn transmit_data(&mut self, spi: &mut SPI, data: &mut [u8]) -> Result<(), Error<SpiError>> {
        self.set_mode(spi, Mode::Standby)?;
        self.write_tx_fifo(spi, data)?;
        self.set_mode(spi, Mode::Transmitter)?;
        Ok(())
    }

    pub fn receive_data(&mut self, spi: &mut SPI) -> Result<(), Error<SpiError>> {
        self.set_mode(spi, Mode::Standby)?;
        self.set_mode(spi, Mode::RxContinuous)?;
        Ok(())
    }

    /// Only do this in Sleep Mode
    // TODO read, set correct mode and restore mode.
    pub fn set_modem_mode(
        &mut self,
        spi: &mut SPI,
        modem_mode: ModemMode,
    ) -> Result<(), Error<SpiError>> {
        self.read_update_write_packed_struct::<_, _, { OpMode::SIZE }>(
            spi,
            |op_mode: &mut OpMode| {
                op_mode.modem_mode = modem_mode;
            },
        )
    }

    pub fn set_continuous_transmission(
        &mut self,
        spi: &mut SPI,
        continuous_transmission_enabled: bool,
    ) -> Result<(), Error<SpiError>> {
        self.read_update_write_packed_struct::<_, _, { ModemConfig2::SIZE }>(
            spi,
            |config: &mut ModemConfig2| {
                config.tx_continuous_mode = continuous_transmission_enabled;
            },
        )
    }

    pub fn write_tx_fifo(&mut self, spi: &mut SPI, data: &mut [u8]) -> Result<(), Error<SpiError>> {
        // get the tx_fifo base address
        let tx_fifo_base = Self::read_register(
            spi,
            &mut self.chip_select,
            LoraRegisters::FifoTxBaseAddress.addr(),
        )?;

        // set the fifo pointer to the tx base address
        Self::write_registers(
            spi,
            &mut self.chip_select,
            LoraRegisters::FifoAddrPointer.addr(),
            &mut [tx_fifo_base],
        )?;

        // write the buffer
        Self::write_registers(spi, &mut self.chip_select, LoraRegisters::Fifo.addr(), data)
    }

    pub fn set_bandwidth(
        &mut self,
        spi: &mut SPI,
        bandwidth: Bandwidth,
    ) -> Result<(), Error<SpiError>> {
        self.read_update_write_packed_struct::<_, _, { ModemConfig1::SIZE }>(
            spi,
            |config: &mut ModemConfig1| config.bandwidth = bandwidth,
        )
    }

    pub fn set_coding_rate(
        &mut self,
        spi: &mut SPI,
        coding_rate: CodingRate,
    ) -> Result<(), Error<SpiError>> {
        self.read_update_write_packed_struct::<_, _, { ModemConfig1::SIZE }>(
            spi,
            |config: &mut ModemConfig1| config.coding_rate = coding_rate,
        )
    }

    pub fn set_spreading_factor(
        &mut self,
        spi: &mut SPI,
        spreading_factor: SpreadingFactor,
    ) -> Result<(), Error<SpiError>> {
        self.read_update_write_packed_struct::<_, _, { ModemConfig2::SIZE }>(
            spi,
            |config: &mut ModemConfig2| config.spreading_factor = spreading_factor,
        )
    }

    pub fn set_rx_payload_crc_on(
        &mut self,
        spi: &mut SPI,
        enabled: bool,
    ) -> Result<(), Error<SpiError>> {
        self.read_update_write_packed_struct::<_, _, { ModemConfig2::SIZE }>(
            spi,
            |config: &mut ModemConfig2| config.rx_payload_crc_on = enabled,
        )
    }

    pub fn set_low_frequency_mode(
        &mut self,
        spi: &mut SPI,
        enabled: bool,
    ) -> Result<(), Error<SpiError>> {
        self.read_update_write_packed_struct::<_, _, { OpMode::SIZE }>(
            spi,
            |config: &mut OpMode| config.low_frequency_mode = enabled,
        )
    }

    pub fn set_implicit_header_mode(
        &mut self,
        spi: &mut SPI,
        enabled: bool,
    ) -> Result<(), Error<SpiError>> {
        self.read_update_write_packed_struct::<_, _, { ModemConfig1::SIZE }>(
            spi,
            |config: &mut ModemConfig1| config.implicit_header_mode_on = enabled,
        )
    }

    pub fn read_packed_struct<S, const NUM_BYTES: usize>(
        &mut self,
        spi: &mut SPI,
    ) -> Result<S, Error<SpiError>>
    where
        S: PackedStruct + StartAddress + SizeBytes,
    {
        let addr = S::start_address().addr();
        let mut buffer = [0; NUM_BYTES];
        Self::read_registers(spi, &mut self.chip_select, addr, &mut buffer)?;

        S::unpack_from_slice(&buffer)
            .map_err(|err| FormattedPackingError(err))
            .map_err(|err| Error::UnpackError { unpack_error: err })
    }

    pub fn write_packed_struct<S>(&mut self, spi: &mut SPI, s: &S) -> Result<(), Error<SpiError>>
    where
        S: PackedStruct + StartAddress + SizeBytes,
    {
        let addr = S::start_address().addr();
        let mut packed = s.pack().unwrap();

        Self::write_registers(
            spi,
            &mut self.chip_select,
            addr,
            packed.as_mut_bytes_slice(),
        )
    }

    pub fn read_update_write_packed_struct<
        S: PackedStruct,
        Updater: FnMut(&mut S),
        const NUM_BYTES: usize,
    >(
        &mut self,
        spi: &mut SPI,
        mut updater: Updater,
    ) -> Result<(), Error<SpiError>>
    where
        S: PackedStruct + StartAddress + SizeBytes,
    {
        // When const generics gets better we can remove NUM_BYTES as a template param and just set
        // the array size with S::SIZE
        debug_assert_eq!(S::SIZE, NUM_BYTES, "NUM_BYTES does not match S::SIZE!");

        let addr = S::start_address().addr();
        let mut buffer = [0; NUM_BYTES];
        Self::read_registers(spi, &mut self.chip_select, addr, &mut buffer)?;

        let mut unpacked = S::unpack_from_slice(&buffer).unwrap();

        updater(&mut unpacked);

        let mut packed = unpacked.pack().unwrap();

        Self::write_registers(
            spi,
            &mut self.chip_select,
            addr,
            packed.as_mut_bytes_slice(),
        )
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
