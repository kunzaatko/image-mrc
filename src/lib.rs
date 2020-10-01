//! Decoding and Encoding of MRC Images
//!
//! The MRC binary file format is widely used in the three-dimensional electron microscopy field for storing image and volume data.
//!
//! # Related Links
//! * <https://www.sciencedirect.com/science/article/pii/S104784771500074X> - The MRC specification

mod bytecast;
pub mod decoder;
mod error;
pub use self::error::{MrcError, MrcResult};

/// An enumeration over supported modes
#[derive(Copy, PartialEq, Eq, Debug, Clone, Hash)]
pub enum Mode {
    /// Represents a 1-byte singed integer
    Mode0,

    /// Represents a 2-byte signed integer
    Mode1,

    /// Represents 4-byte reals
    Mode2,

    /// Represents a complex number consisting of a pair of 2-byte singed integers
    Mode3,

    /// Represents a complex number consisting of a pair of 4-byte reals
    Mode4,

    /// Represents a 2-byte unsigned integer [implemented by UCSFtomo (Zheng et al., 2007)]
    Mode6,

    /// [(Kremer et al., 1996)]
    IMOD,

    /// [(FEI)]
    EPU,

    /// [(Image Visualization Environment, Chen et al., 1996)]
    IVE,

    /// Represents RGB data in 3 1-byte unsigned integers [(IMOD)]
    Mode16,
}
