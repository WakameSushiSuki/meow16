use crate::vm;
use strum::EnumCount;

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    Error,
    ErrorMessage(String),
    OutOfRangeOpcode(u8),
    OutOfRangeMemory(u16),
    PinInactive(u16),
} impl Error {
    pub fn error_code(&self) -> usize {
        match self {
            Self::Error => 1,
            Self::ErrorMessage(_) => 2,
            Self::OutOfRangeOpcode(_) => 3,
            Self::OutOfRangeMemory(_) => 4,
            Self::PinInactive(_) => 5,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::Error => format!("an error occured"),
            Self::ErrorMessage(msg) => format!("{}", msg),
            Self::OutOfRangeOpcode(op) => format!(
                "{} is out of bounds of the maximum opcode {}",
                op, vm::Opcode::COUNT
            ),
            Self::OutOfRangeMemory(addr) => format!(
                "{} is out of bounds of the maximum memory address {}",
                addr, "0xCAFE" // placeholder
            ),
            Self::PinInactive(pin) => format!("pin {} has no active signal", pin)
        }
    }
}
