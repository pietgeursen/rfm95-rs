use snafu::Snafu;
use defmt::Format;
use core::fmt::Debug;

#[derive(Debug, Snafu, Format)]
#[snafu(visibility = "pub(crate)")]
pub enum Error<SpiError: Debug> {
   Todo,
   SetPin,
   #[snafu(display("Could not read spi: {:?}", spi_err))]
   SpiRead{spi_err: SpiError},
   #[snafu(display("Could not complete spi transfer: {:?}", spi_err))]
   SpiTransfer{spi_err: SpiError}
}

