use core::fmt::Debug;
use defmt::Format;
use snafu::Snafu;
use packed_struct::PackingError;

#[derive(Debug)]
pub struct FormattedPackingError(pub PackingError);

impl defmt::Format for FormattedPackingError {
    fn format(&self, f: &mut defmt::Formatter) {
        // format the bitfields of the register as struct fields
        defmt::write!(
           f,
           "Packing Error",
        )
    }
}

#[derive(Debug, Snafu, Format)]
#[snafu(visibility = "pub(crate)")]
pub enum Error<SpiError: Debug> {
    Todo,
    #[snafu(display("Could not unpack bits: {:?}", unpack_error))]
    UnpackError{
        unpack_error: FormattedPackingError,
    },
    SetPin,
    #[snafu(display("Could not read spi: {:?}", spi_err))]
    SpiRead {
        spi_err: SpiError,
    },
    #[snafu(display("Could not complete spi transfer: {:?}", spi_err))]
    SpiTransfer {
        spi_err: SpiError,
    },
}
