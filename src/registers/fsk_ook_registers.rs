pub enum FskOokRegisters {
    Fifo = 0x00,

    // Common registers
    OpMode = 0x01,
    BitrateMsb = 0x02,
    BitrateLsb = 0x03,
    FdevMsb = 0x04,
    FdevLsb = 0x05,
    FrfMsb = 0x06,
    FrfMid = 0x07,
    FrfLsb = 0x08,

    // Transmitter Registers
    PaConfig = 0x09,
    PaRamp = 0x0A,
    Ocp = 0x0B,

    // Receiver Registers
    Lna = 0x0C,
    RxConfig = 0x0D,
    RssiConfig = 0x0E,
    RssiCollision = 0x0F,
    RssiThres = 0x10,
    RssiValue = 0x11,
    RxBw = 0x12,
    AfcBw = 0x13,
    OokPeak = 0x14,
    OokFix = 0x15,
    OokAvg = 0x16,

    // 0x17 - 0x19 not valid for OOK. 
    // They're RW but you have to leave the default values. 

    AfcFei = 0x1A,
    AfcMsb = 0x1B,
    AfcLsb = 0x1C,
    FeiMsb = 0x1D,
    FeiLsb = 0x1E,
    PreambleDetect = 0x1F,
    RxTimeout1 = 0x20,
    RxTimeout2 = 0x21,
    RxTimeout3 = 0x22,
    RxDelay = 0x23,

    // Rc oscillator registers
    Osc = 0x24,

    // Packet handling registers
    PreambleMsb = 0x25,
    PreambleLsb = 0x26,
    SyncConfig = 0x27,
    SyncValue1 = 0x28,
    SyncValue2 = 0x29,
    SyncValue3 = 0x2A,
    SyncValue4 = 0x2B,
    SyncValue5 = 0x2C,
    SyncValue6 = 0x2D,
    SyncValue7 = 0x2E,
    SyncValue8 = 0x2F,
    PacketConfig1 = 0x30,
    PacketConfig2 = 0x31,
    PayloadLength = 0x32,
    NodeAdress = 0x33,
    BroadcastAddress = 0x34,
    FifoThreshold = 0x35,
    SeqConfig1 = 0x36,
    SeqConfig2 = 0x37,
    TimerResolution = 0x38,
    Timer1Coefficient = 0x39,
    Timer2Coefficient = 0x3A,

    // Service Registers
    ImageCalibration = 0x3B,
    Temp = 0x3C,
    LowBat = 0x3D,

    // Status Registers
    IrqFlags1 = 0x3E,
    IrqFlags2 = 0x3F,

    // IO Control Registers
    DioMapping1 = 0x40,
    DioMapping2 = 0x41,

    // Version Registers
    Version = 0x42,

    // Additional Registers
    PllHop = 0x44,
    Tcxo = 0x4B,
    PaDac = 0x4D,
    FormerTemp = 0x5B,
    BitrateFractionalPart = 0x5D,
    AgcRef = 0x61,
    AgcThreshold1 = 0x62,
    AgcThreshold2 = 0x63,
    AgcThreshold3 = 0x64,

}

impl FskOokRegisters {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

