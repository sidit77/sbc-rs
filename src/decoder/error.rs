use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Reason {
    InvalidSyncWord,
    InvalidSubbands,
    InvalidBlockLength,
    InvalidBitpoolValue,
    InvalidCrc,
    UnexpectedData
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    BadData(Reason),
    NotEnoughData {
        expected: usize,
        actual: usize
    },
    OutputBufferTooSmall {
        expected: usize,
        actual: usize
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadData(reason) => write!(f, "Failed to decode frame: {:?}", reason),
            Error::NotEnoughData { expected, actual } => write!(f, "Not enough data: expected {} bytes, got {}", expected, actual),
            Error::OutputBufferTooSmall { expected, actual } => write!(f, "Output buffer too small: expected {} samples, got {}", expected, actual)
        }
    }
}

impl std::error::Error for Error {}