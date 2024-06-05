use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum MemoryError {
    #[error("Unable to read from given address.")]
    ReadError,
    #[error("Unable to write to given address.")]
    WriteError,
}

#[derive(Debug, Error, PartialEq)]
pub enum StackError {
    #[error("Stackoverflow!")]
    StackOverflow,
    #[error("Stackunderflow!")]
    StackUnderflow,
}

#[derive(Debug, Error, PartialEq)]
pub enum MachineError {
    #[error("{0}")]
    MemoryError(MemoryError),
    #[error("{0}")]
    StackError(StackError),
    #[error("Unknown opcode: {0}")]
    UnknownOpCode(u16),
}
