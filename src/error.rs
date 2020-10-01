use std::error::Error;
use std::fmt;
use std::io;
use std::string;

use super::decoder::ifd::Value;
use super::Mode;

/// Mrc error kinds.
#[derive(Debug)]
pub enum MrcError {
    /// The Image is not formatted properly.
    FormatError(MrcFormatError),

    /// The Decoder does not support features required by the image.
    UnsupportedError(MrcUnsupportedError),

    /// An I/O Error occurred while decoding the image.
    IoError(io::Error),

    /// The Limits of the Decoder is exceeded.
    LimitsExceeded,
}

/// The image is not formatted properly.
///
/// This indicates that the encoder producing the image might behave incorrectly or that the input
/// file has been corrupted.
#[derive(Debug, Clone, PartialEq)]
pub enum MrcFormatError {
    ByteExpected(Value),
    UnsignedIntegerExpected(Value),
    SignedIntegerExpected(Value),
    Format(String),
}

impl fmt::Display for MrcFormatError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::MrcFormatError::*;
        match *self {
            ByteExpected(ref val) => write!(fmt, "Expected byte, {:?} found.", val),
            UnsignedIntegerExpected(ref val) => {
                write!(fmt, "Expected unsigned integer, {:?} found.", val)
            }
            SignedIntegerExpected(ref val) => {
                write!(fmt, "Expected signed integer, {:?} found.", val)
            }
            Format(ref val) => write!(fmt, "Invalid format: {:?}.", val),
        }
    }
}

/// The Decoder does not support features required by the image.
///
/// This only captures known failures for which the standard either does not require support or an
/// implementation has been planned but not yet completed. Some variants may become unused over
/// time and will then get deprecated before being removed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MrcUnsupportedError {
    UnsupportedMode(Mode),
    UnsupportedDataType,
}

/// Result of an image decoding/encoding process
pub type MrcResult<T> = Result<T, MrcError>;
