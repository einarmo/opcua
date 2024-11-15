// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

//! Contains definitions of the simple OPC UA scalar types.
use std::io::{Read, Write};

use crate::encoding::*;

// OPC UA Part 6 - Mappings 1.03 Specification

// Standard UA types onto Rust types:

// Boolean  -> bool
// SByte    -> i8
// Byte     -> u8
// Int16    -> i16
// UInt16   -> u16
// Int32    -> i32
// UInt32   -> u32
// Int64    -> i64
// UInt64   -> u64
// Float    -> f32
// Double   -> f64

impl BinaryEncodable for bool {
    fn byte_len(&self) -> usize {
        1
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        // 0, or 1 for true or false, single byte
        write_u8(stream, if *self { 1 } else { 0 })
    }
}

impl BinaryDecodable for bool {
    fn decode<S: Read>(stream: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
        Ok(read_u8(stream)? == 1)
    }
}

impl BinaryEncodable for i8 {
    fn byte_len(&self) -> usize {
        1
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_u8(stream, *self as u8)
    }
}

impl BinaryDecodable for i8 {
    fn decode<S: Read>(stream: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
        Ok(read_u8(stream)? as i8)
    }
}

/// An unsigned byt integer value between 0 and 255.
impl BinaryEncodable for u8 {
    fn byte_len(&self) -> usize {
        1
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_u8(stream, *self)
    }
}

impl BinaryDecodable for u8 {
    fn decode<S: Read>(stream: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
        read_u8(stream)
    }
}

/// A signed integer value between −32768 and 32767.
impl BinaryEncodable for i16 {
    fn byte_len(&self) -> usize {
        2
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i16(stream, *self)
    }
}

impl BinaryDecodable for i16 {
    fn decode<S: Read>(stream: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
        read_i16(stream)
    }
}

/// An unsigned integer value between 0 and 65535.
impl BinaryEncodable for u16 {
    fn byte_len(&self) -> usize {
        2
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_u16(stream, *self)
    }
}

impl BinaryDecodable for u16 {
    fn decode<S: Read>(stream: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
        read_u16(stream)
    }
}

/// A signed integer value between −2147483648 and 2147483647.
impl BinaryEncodable for i32 {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self)
    }
}

impl BinaryDecodable for i32 {
    fn decode<S: Read>(stream: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
        read_i32(stream)
    }
}

/// An unsigned integer value between 0 and 4294967295.
impl BinaryEncodable for u32 {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_u32(stream, *self)
    }
}
impl BinaryDecodable for u32 {
    fn decode<S: Read>(stream: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
        read_u32(stream)
    }
}

/// A signed integer value between −9223372036854775808 and 9223372036854775807.
impl BinaryEncodable for i64 {
    fn byte_len(&self) -> usize {
        8
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i64(stream, *self)
    }
}

impl BinaryDecodable for i64 {
    fn decode<S: Read>(stream: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
        read_i64(stream)
    }
}

/// An unsigned integer value between 0 and 18446744073709551615.
impl BinaryEncodable for u64 {
    fn byte_len(&self) -> usize {
        8
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_u64(stream, *self)
    }
}

impl BinaryDecodable for u64 {
    fn decode<S: Read>(stream: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
        read_u64(stream)
    }
}

/// An IEEE single precision (32 bit) floating point value.
impl BinaryEncodable for f32 {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_f32(stream, *self)
    }
}

impl BinaryDecodable for f32 {
    fn decode<S: Read>(stream: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
        read_f32(stream)
    }
}

/// An IEEE double precision (64 bit) floating point value.
impl BinaryEncodable for f64 {
    fn byte_len(&self) -> usize {
        8
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_f64(stream, *self)
    }
}

impl BinaryDecodable for f64 {
    fn decode<S: Read>(stream: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
        read_f64(stream)
    }
}
