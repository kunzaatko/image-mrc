//! All IO functionality needed for MRC decoding

use crate::bytecast;
use std::io::{self, Read, Seek};

/// Byte order of the MRC file.
#[derive(Clone, Copy, Debug)]
pub enum ByteOrder {
    /// little endian byte order
    LittleEndian,
    /// big endian byte order
    BigEndian,
}

/// Reader that is aware of the byte order.
pub trait EndianReader: Read {
    /// Byte order that should be adhered to
    fn byte_order(&self) -> ByteOrder;

    /// Reads an u16
    #[inline(always)]
    fn read_u16(&mut self) -> Result<u16, io::Error> {
        let mut n = [0u8; 2];
        self.read_exact(&mut n)?;
        Ok(match self.byte_order() {
            ByteOrder::LittleEndian => u16::from_le_bytes(n),
            ByteOrder::BigEndian => u16::from_be_bytes(n),
        })
    }

    #[inline(always)]
    fn read_u16_into(&mut self, buffer: &mut [u16]) -> Result<(), io::Error> {
        self.read_exact(bytecast::u16_as_ne_mut_bytes(buffer))?;
        match self.byte_order() {
            ByteOrder::LittleEndian => {
                for n in buffer {
                    *n = u16::from_le(*n);
                }
            }
            ByteOrder::BigEndian => {
                for n in buffer {
                    *n = u16::from_be(*n);
                }
            }
        }
        Ok(())
    }

    /// Reads an i16
    #[inline(always)]
    fn read_i16(&mut self) -> Result<i16, io::Error> {
        let mut n = [0u8; 2];
        self.read_exact(&mut n)?;
        Ok(match self.byte_order() {
            ByteOrder::LittleEndian => i16::from_le_bytes(n),
            ByteOrder::BigEndian => i16::from_be_bytes(n),
        })
    }

    /// Reads an u32
    #[inline(always)]
    fn read_u32(&mut self) -> Result<u32, io::Error> {
        let mut n = [0u8; 4];
        self.read_exact(&mut n)?;
        Ok(match self.byte_order() {
            ByteOrder::LittleEndian => u32::from_le_bytes(n),
            ByteOrder::BigEndian => u32::from_be_bytes(n),
        })
    }

    #[inline(always)]
    fn read_u32_into(&mut self, buffer: &mut [u32]) -> Result<(), io::Error> {
        self.read_exact(bytecast::u32_as_ne_mut_bytes(buffer))?;
        match self.byte_order() {
            ByteOrder::LittleEndian => {
                for n in buffer {
                    *n = u32::from_le(*n);
                }
            }
            ByteOrder::BigEndian => {
                for n in buffer {
                    *n = u32::from_be(*n);
                }
            }
        }
        Ok(())
    }

    /// Reads an i32
    #[inline(always)]
    fn read_i32(&mut self) -> Result<i32, io::Error> {
        let mut n = [0u8; 4];
        self.read_exact(&mut n)?;
        Ok(match self.byte_order() {
            ByteOrder::LittleEndian => i32::from_le_bytes(n),
            ByteOrder::BigEndian => i32::from_be_bytes(n),
        })
    }

    /// Reads an u64
    #[inline(always)]
    fn read_u64(&mut self) -> Result<u64, io::Error> {
        let mut n = [0u8; 8];
        self.read_exact(&mut n)?;
        Ok(match self.byte_order() {
            ByteOrder::LittleEndian => u64::from_le_bytes(n),
            ByteOrder::BigEndian => u64::from_be_bytes(n),
        })
    }

    #[inline(always)]
    fn read_u64_into(&mut self, buffer: &mut [u64]) -> Result<(), io::Error> {
        self.read_exact(bytecast::u64_as_ne_mut_bytes(buffer))?;
        match self.byte_order() {
            ByteOrder::LittleEndian => {
                for n in buffer {
                    *n = u64::from_le(*n);
                }
            }
            ByteOrder::BigEndian => {
                for n in buffer {
                    *n = u64::from_be(*n);
                }
            }
        }
        Ok(())
    }

    /// Reads an f32
    #[inline(always)]
    fn read_f32(&mut self) -> Result<f32, io::Error> {
        let mut n = [0u8; 4];
        self.read_exact(&mut n)?;
        Ok(f32::from_bits(match self.byte_order() {
            ByteOrder::LittleEndian => u32::from_le_bytes(n),
            ByteOrder::BigEndian => u32::from_be_bytes(n),
        }))
    }

    #[inline(always)]
    fn read_f32_into(&mut self, buffer: &mut [f32]) -> Result<(), io::Error> {
        self.read_exact(bytecast::f32_as_ne_mut_bytes(buffer))?;
        match self.byte_order() {
            ByteOrder::LittleEndian => {
                for n in buffer {
                    *n = f32::from_bits(u32::from_le(n.to_bits()));
                }
            }
            ByteOrder::BigEndian => {
                for n in buffer {
                    *n = f32::from_bits(u32::from_be(n.to_bits()));
                }
            }
        }
        Ok(())
    }

    /// Reads an f64
    #[inline(always)]
    fn read_f64(&mut self) -> Result<f64, io::Error> {
        let mut n = [0u8; 8];
        self.read_exact(&mut n)?;
        Ok(f64::from_bits(match self.byte_order() {
            ByteOrder::LittleEndian => u64::from_le_bytes(n),
            ByteOrder::BigEndian => u64::from_be_bytes(n),
        }))
    }

    #[inline(always)]
    fn read_f64_into(&mut self, buffer: &mut [f64]) -> Result<(), io::Error> {
        self.read_exact(bytecast::f64_as_ne_mut_bytes(buffer))?;
        match self.byte_order() {
            ByteOrder::LittleEndian => {
                for n in buffer {
                    *n = f64::from_bits(u64::from_le(n.to_bits()));
                }
            }
            ByteOrder::BigEndian => {
                for n in buffer {
                    *n = f64::from_bits(u64::from_be(n.to_bits()));
                }
            }
        }
        Ok(())
    }
}

///
/// ## SmartReader Reader
///

/// Reader that is aware of the byte order.
#[derive(Debug)]
pub struct SmartReader<R>
where
    R: Read + Seek,
{
    reader: R,
    pub byte_order: ByteOrder,
}

impl<R> SmartReader<R>
where
    R: Read + Seek,
{
    /// Wraps a reader
    pub fn wrap(reader: R, byte_order: ByteOrder) -> SmartReader<R> {
        SmartReader { reader, byte_order }
    }
}

impl<R> EndianReader for SmartReader<R>
where
    R: Read + Seek,
{
    #[inline(always)]
    fn byte_order(&self) -> ByteOrder {
        self.byte_order
    }
}

impl<R: Read + Seek> Read for SmartReader<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Read + Seek> Seek for SmartReader<R> {
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.reader.seek(pos)
    }
}
