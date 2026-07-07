pub mod ops;
pub mod mem;
pub mod regs;
pub mod alu;
pub mod instrs;
pub mod error;

pub use ops::Opcode;
pub use error::Error;

use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub struct StepData {
    // this is a placeholder which will become a snapshot of some state for
    // stepping and error reporting.
    // includes:
    // - regfile
    // - instr
    // - side effects
    // - ptr to memory array
} impl fmt::Display for StepData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MEOW at 0xCAFE")
    }
}

/*
impl fmt::Display for StepData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at 0x{:X}", self.instr, self.regfile.get_pc())
    }
}
*/
