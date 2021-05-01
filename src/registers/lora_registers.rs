pub enum LoraRegisters {
    Fifo = 0x00,

    // Common registers
    OpMode = 0x01,
    FrfMsb = 0x06,
    FrfMid = 0x07,
    FrfLsb = 0x08,

    // Transmitter Registers
    PaConfig = 0x09,
    PaRamp = 0x0A,
    Ocp = 0x0B,

    // Receiver Registers
    Lna = 0x0C,

    // Lora Page Registers
    FifoAddrPointer = 0x0D,
    FifoTxBaseAddress = 0x0E,
    FifoRxBaseAddress= 0x0F,
    FifoRxCurrentAddress= 0x10,
    IrqFlagsMask = 0x11,
    IrqFlags = 0x12,

    RxNumberOfBytesReceived = 0x13,
    RxHeaderCountMsb = 0x14,
    RxHeaderCountLsb = 0x15,
    RxPacketCountMsb = 0x16,
    RxPacketCountLsb = 0x17,
    ModemStatus = 0x18,
    PacketSNR = 0x19,
    PacketRssi = 0x1A,
    Rssi = 0x1B,
    HopChannel = 0x1C,

    ModemConfig1 = 0x1D,
    ModemConfig2 = 0x1E,

    SymbolTimeoutLsb = 0x1F,

    PreambleMsb = 0x20,
    PreambleLsb = 0x21,
    PayloadLength = 0x22,
    MaxPayloadLength = 0x23,
    HopPeriod = 0x24,
    FifoRxByteAddrPtr = 0x25,
    ModemConfig3 = 0x26,
    PpmCorrection = 0x27,
    FeiMsb = 0x28,
    FeiMid = 0x29,
    FeiLsb = 0x2A,
    RssiWideband = 0x2C,
    IfFreq = 0x2F,

}

impl LoraRegisters {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

