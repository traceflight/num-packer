//! Provides traits and implementations for packing and unpacking two numbers into a single value.
//!
//! ## Implementations
//!
//! * Pack two `bool` into `u8`/`i8`, `u16`/`i16`, `u32`/`i32`, `u64`/`i64`, `usize`/`isize`
//! * Pack two `u8` into `u16`/`i16`, `u32`/`i32`, `u64`/`i64`, `usize`/`isize`
//! * Pack two `u16` into `u32`/`i32`, `u64`/`i64`, `usize`/`isize`(if target pointer width is 32 or 64)
//! * Pack two `u32` into `u64`/`i64`, `usize`/`isize`(if target pointer width is 64)
//! * Pack two `i8` into `u16`/`i16`, `u32`/`i32`, `u64`/`i64`, `usize`/`isize`
//! * Pack two `i16` into `u32`/`i32`, `u64`/`i64`, `usize`/`isize`(if target pointer width is 32 or 64)
//! * Pack two `i32` into `u64`/`i64`, `usize`/`isize`(if target pointer width is 64)
//! * Pack two `f32` into `u64`/`i64`, `usize`/`isize`(if target pointer width is 64)
//!
//! ## Example
//!
//! Pack two `u8` into `u16`.
//!
//! ```rust
//! use num_packer::U8Packer;
//!
//! let packed = u16::pack_u8(200, 55);
//! let (first, second) = packed.unpack_u8();
//! assert_eq!((first, second), (200, 55));
//! ```
//!

#![no_std]

mod impls;
mod packer;

pub use packer::*;
