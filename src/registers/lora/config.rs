pub use super::modem_config1::{Bandwidth, CodingRate};
pub use super::modem_config2::SpreadingFactor;

#[derive(Clone)]
pub struct Config {
    pub bandwidth: Bandwidth,
    pub coding_rate: CodingRate,
    pub implicit_header_mode_on: bool,
    pub spreading_factor: SpreadingFactor,
    pub rx_payload_crc_on: bool,
    pub low_frequency_mode: bool,
}
