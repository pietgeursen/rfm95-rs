use super::lora_registers::LoraRegisters;

pub trait StartAddress{
    fn start_address() -> LoraRegisters;
}
