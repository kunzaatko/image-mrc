use super::{Mode, MrcError, MrcResult};
use std::cmp;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{self, Read, Seek};

pub mod header;
pub mod ifd;
mod stream;

use self::stream::{ByteOrder, SmartReader};
use header::Header;

/// Result of a decoding process
#[derive(Debug)]
pub enum DecodingResult {
    /// A vector of unsigned bytes
    U8(Vec<u8>),
    /// A vector of unsigned words
    U16(Vec<u16>),
    /// A vector of 32 bit unsigned ints
    U32(Vec<u32>),
    /// A vector of 64 bit unsigned ints
    U64(Vec<u64>),
    /// A vector of 32 bit IEEE floats
    F32(Vec<f32>),
    /// A vector of 64 bit IEEE floats
    F64(Vec<f64>),
}

impl DecodingResult {
    fn new_u8(size: usize, limits: &Limits) -> MrcResult<DecodingResult> {
        if size > limits.decoding_buffer_size {
            Err(MrcError::LimitsExceeded)
        } else {
            Ok(DecodingResult::U8(vec![0; size]))
        }
    }

    fn new_u16(size: usize, limits: &Limits) -> MrcResult<DecodingResult> {
        if size > limits.decoding_buffer_size / 2 {
            Err(MrcError::LimitsExceeded)
        } else {
            Ok(DecodingResult::U16(vec![0; size]))
        }
    }

    fn new_u32(size: usize, limits: &Limits) -> MrcResult<DecodingResult> {
        if size > limits.decoding_buffer_size / 4 {
            Err(MrcError::LimitsExceeded)
        } else {
            Ok(DecodingResult::U32(vec![0; size]))
        }
    }

    fn new_u64(size: usize, limits: &Limits) -> MrcResult<DecodingResult> {
        if size > limits.decoding_buffer_size / 8 {
            Err(MrcError::LimitsExceeded)
        } else {
            Ok(DecodingResult::U64(vec![0; size]))
        }
    }

    fn new_f32(size: usize, limits: &Limits) -> MrcResult<DecodingResult> {
        if size > limits.decoding_buffer_size / std::mem::size_of::<f32>() {
            Err(MrcError::LimitsExceeded)
        } else {
            Ok(DecodingResult::F32(vec![0.0; size]))
        }
    }

    fn new_f64(size: usize, limits: &Limits) -> MrcResult<DecodingResult> {
        if size > limits.decoding_buffer_size / std::mem::size_of::<f64>() {
            Err(MrcError::LimitsExceeded)
        } else {
            Ok(DecodingResult::F64(vec![0.0; size]))
        }
    }

    pub fn as_buffer(&mut self, start: usize) -> DecodingBuffer {
        match *self {
            DecodingResult::U8(ref mut buf) => DecodingBuffer::U8(&mut buf[start..]),
            DecodingResult::U16(ref mut buf) => DecodingBuffer::U16(&mut buf[start..]),
            DecodingResult::U32(ref mut buf) => DecodingBuffer::U32(&mut buf[start..]),
            DecodingResult::U64(ref mut buf) => DecodingBuffer::U64(&mut buf[start..]),
            DecodingResult::F32(ref mut buf) => DecodingBuffer::F32(&mut buf[start..]),
            DecodingResult::F64(ref mut buf) => DecodingBuffer::F64(&mut buf[start..]),
        }
    }
}

// A buffer for image decoding
pub enum DecodingBuffer<'a> {
    /// A slice of unsigned bytes
    U8(&'a mut [u8]),
    /// A slice of unsigned words
    U16(&'a mut [u16]),
    /// A slice of 32 bit unsigned ints
    U32(&'a mut [u32]),
    /// A slice of 64 bit unsigned ints
    U64(&'a mut [u64]),
    /// A slice of 32 bit IEEE floats
    F32(&'a mut [f32]),
    /// A slice of 64 bit IEEE floats
    F64(&'a mut [f64]),
}

impl<'a> DecodingBuffer<'a> {
    fn len(&self) -> usize {
        match *self {
            DecodingBuffer::U8(ref buf) => buf.len(),
            DecodingBuffer::U16(ref buf) => buf.len(),
            DecodingBuffer::U32(ref buf) => buf.len(),
            DecodingBuffer::U64(ref buf) => buf.len(),
            DecodingBuffer::F32(ref buf) => buf.len(),
            DecodingBuffer::F64(ref buf) => buf.len(),
        }
    }

    fn byte_len(&self) -> usize {
        match *self {
            DecodingBuffer::U8(_) => 1,
            DecodingBuffer::U16(_) => 2,
            DecodingBuffer::U32(_) => 4,
            DecodingBuffer::U64(_) => 8,
            DecodingBuffer::F32(_) => 4,
            DecodingBuffer::F64(_) => 8,
        }
    }

    fn copy<'b>(&'b mut self) -> DecodingBuffer<'b>
    where
        'a: 'b,
    {
        match *self {
            DecodingBuffer::U8(ref mut buf) => DecodingBuffer::U8(buf),
            DecodingBuffer::U16(ref mut buf) => DecodingBuffer::U16(buf),
            DecodingBuffer::U32(ref mut buf) => DecodingBuffer::U32(buf),
            DecodingBuffer::U64(ref mut buf) => DecodingBuffer::U64(buf),
            DecodingBuffer::F32(ref mut buf) => DecodingBuffer::F32(buf),
            DecodingBuffer::F64(ref mut buf) => DecodingBuffer::F64(buf),
        }
    }
}

#[derive(Debug)]
struct StripDecodeState {
    strip_index: usize,
    strip_offsets: Vec<u64>,
    strip_bytes: Vec<u64>,
}

/// Decoding limits
#[derive(Clone, Debug)]
pub struct Limits {
    /// The maximum size of any `DecodingResult` in bytes, the default is
    /// 256MiB. If the entire image is decoded at once, then this will
    /// be the maximum size of the image. If it is decoded one strip at a
    /// time, this will be the maximum size of a strip.
    pub decoding_buffer_size: usize,
    /// The maximum size of any ifd value in bytes, the default is
    /// 1MiB.
    pub ifd_value_size: usize,
    /// Maximum size for intermediate buffer which may be used to limit the amount of data read per
    /// segment even if the entire image is decoded at once.
    pub intermediate_buffer_size: usize,
}

impl Default for Limits {
    fn default() -> Limits {
        Limits {
            decoding_buffer_size: 256 * 1024 * 1024,
            intermediate_buffer_size: 128 * 1024 * 1024,
            ifd_value_size: 1024 * 1024,
        }
    }
}

/// The representation of a MRC decoder
#[derive(Debug)]
pub struct Decoder<R>
where
    R: Read + Seek,
{
    reader: SmartReader<R>,
    byte_order: ByteOrder,
    limits: Limits,
    width: u32,
    height: u32,
    header: Option<Header>,
    // bits_per_sample: Vec<u8>,
    // samples: u8,
    // sample_format: Vec<SampleFormat>,
    strip_decoder: Option<StripDecodeState>,
}

impl<R: Read + Seek> Decoder<R> {
    /// Create a new decoder that decodes from the stream ```r```
    pub fn new(r: R) -> MrcResult<Decoder<R>> {
        Decoder {
            reader: SmartReader::wrap(r, ByteOrder::LittleEndian),
            byte_order: ByteOrder::LittleEndian,
            limits: Default::default(),
            width: 0,
            height: 0,
            header: None,
            // bits_per_sample: vec![1],
            // samples: 1,
            // sample_format: vec![SampleFormat::Uint],
            // photometric_interpretation: PhotometricInterpretation::BlackIsZero,
            strip_decoder: None,
        }
        .init()
    }

    pub fn with_limits(mut self, limits: Limits) -> Decoder<R> {
        self.limits = limits;
        self
    }

    pub fn dimensions(&self) -> MrcResult<(u32, u32)> {
        Ok((self.width, self.height))
    }

    fn read_header(&mut self) -> MrcResult<()> {
        // TODO: implement <01-10-20, kunzaatko> //
        Ok(())
    }

    /// Reads in the next image.
    /// If there is no further image in the TIFF file a format error is returned.
    /// To determine whether there are more images call `MrcDecoder::more_images` instead.
    fn next_image(&mut self) -> MrcResult<()> {
        // TODO: implement <01-10-20, kunzaatko> //
        Ok(())
    }

    /// Initializes the decoder.
    pub fn init(mut self) -> MrcResult<Decoder<R>> {
        self.read_header()?;
        self.next_image()?;
        Ok(self)
    }
}
