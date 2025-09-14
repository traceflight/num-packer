use crate::{
    BoolPacker, F32Packer, I8Packer, I16Packer, I32Packer, U8Packer, U16Packer, U32Packer,
};

macro_rules! impl_bool_packer_for_num {
    ($t:ty) => {
        impl BoolPacker for $t {
            fn pack_bool(first: bool, second: bool) -> Self {
                ((first as $t) << 1) | (second as $t)
            }

            fn first_bool(&self) -> bool {
                (self & 0b10) != 0
            }

            fn second_bool(&self) -> bool {
                (self & 0b01) != 0
            }
        }
    };
}

impl_bool_packer_for_num!(u8);
impl_bool_packer_for_num!(u16);
impl_bool_packer_for_num!(u32);
impl_bool_packer_for_num!(u64);
impl_bool_packer_for_num!(usize);
impl_bool_packer_for_num!(i8);
impl_bool_packer_for_num!(i16);
impl_bool_packer_for_num!(i32);
impl_bool_packer_for_num!(i64);
impl_bool_packer_for_num!(isize);

// impl U8Packer
macro_rules! impl_u8_packer_for_num {
    ($t:ty) => {
        impl U8Packer for $t {
            fn pack_u8(first: u8, second: u8) -> Self {
                ((first as $t) << 8) | (second as $t)
            }

            fn first_u8(&self) -> u8 {
                ((self >> 8) & 0xFF) as u8
            }

            fn second_u8(&self) -> u8 {
                (self & 0xFF) as u8
            }
        }
    };
}

impl_u8_packer_for_num!(u16);
impl_u8_packer_for_num!(u32);
impl_u8_packer_for_num!(u64);
impl_u8_packer_for_num!(usize);
impl_u8_packer_for_num!(i16);
impl_u8_packer_for_num!(i32);
impl_u8_packer_for_num!(i64);
impl_u8_packer_for_num!(isize);

// impl U16Packer
macro_rules! impl_u16_packer_for_num {
    ($t:ty) => {
        impl U16Packer for $t {
            fn pack_u16(first: u16, second: u16) -> Self {
                ((first as $t) << 16) | (second as $t)
            }

            fn first_u16(&self) -> u16 {
                ((self >> 16) & 0xFFFF) as u16
            }

            fn second_u16(&self) -> u16 {
                (self & 0xFFFF) as u16
            }
        }
    };
}

impl_u16_packer_for_num!(u32);
impl_u16_packer_for_num!(u64);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_u16_packer_for_num!(usize);
impl_u16_packer_for_num!(i32);
impl_u16_packer_for_num!(i64);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_u16_packer_for_num!(isize);

// impl U32Packer
macro_rules! impl_u32_packer_for_num {
    ($t:ty) => {
        impl U32Packer for $t {
            fn pack_u32(first: u32, second: u32) -> Self {
                ((first as $t) << 32) | (second as $t)
            }

            fn first_u32(&self) -> u32 {
                ((self >> 32) & 0xFFFFFFFF) as u32
            }

            fn second_u32(&self) -> u32 {
                (self & 0xFFFFFFFF) as u32
            }
        }
    };
}

impl_u32_packer_for_num!(u64);
#[cfg(target_pointer_width = "64")]
impl_u32_packer_for_num!(usize);
impl_u32_packer_for_num!(i64);
#[cfg(target_pointer_width = "64")]
impl_u32_packer_for_num!(isize);

macro_rules! impl_i8_packer_for_num {
    ($t:ty) => {
        impl I8Packer for $t {
            fn pack_i8(first: i8, second: i8) -> Self {
                let first_u = first as u8 as $t;
                let second_u = second as u8 as $t;
                (first_u << 8) | second_u
            }

            fn first_i8(&self) -> i8 {
                ((self >> 8) & 0xFF) as u8 as i8
            }

            fn second_i8(&self) -> i8 {
                (self & 0xFF) as u8 as i8
            }
        }
    };
}

impl_i8_packer_for_num!(i16);
impl_i8_packer_for_num!(i32);
impl_i8_packer_for_num!(i64);
impl_i8_packer_for_num!(isize);
impl_i8_packer_for_num!(u16);
impl_i8_packer_for_num!(u32);
impl_i8_packer_for_num!(u64);
impl_i8_packer_for_num!(usize);

macro_rules! impl_i16_packer_for_num {
    ($t:ty) => {
        impl I16Packer for $t {
            fn pack_i16(first: i16, second: i16) -> Self {
                let first_u = first as u16 as $t;
                let second_u = second as u16 as $t;
                (first_u << 16) | second_u
            }

            fn first_i16(&self) -> i16 {
                ((self >> 16) & 0xFFFF) as u16 as i16
            }

            fn second_i16(&self) -> i16 {
                (self & 0xFFFF) as u16 as i16
            }
        }
    };
}

impl_i16_packer_for_num!(u32);
impl_i16_packer_for_num!(u64);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_i16_packer_for_num!(usize);
impl_i16_packer_for_num!(i32);
impl_i16_packer_for_num!(i64);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_i16_packer_for_num!(isize);

macro_rules! impl_i32_packer_for_num {
    ($t:ty) => {
        impl I32Packer for $t {
            fn pack_i32(first: i32, second: i32) -> Self {
                let first_u = first as u32 as $t;
                let second_u = second as u32 as $t;
                (first_u << 32) | second_u
            }

            fn first_i32(&self) -> i32 {
                ((self >> 32) & 0xFFFFFFFF) as u32 as i32
            }

            fn second_i32(&self) -> i32 {
                (self & 0xFFFFFFFF) as u32 as i32
            }
        }
    };
}

impl_i32_packer_for_num!(u64);
#[cfg(target_pointer_width = "64")]
impl_i32_packer_for_num!(usize);
impl_i32_packer_for_num!(i64);
#[cfg(target_pointer_width = "64")]
impl_i32_packer_for_num!(isize);

macro_rules! impl_f32_packer_for_num {
    ($t:ty) => {
        impl F32Packer for $t {
            fn pack_f32(first: f32, second: f32) -> Self {
                let first_u = first.to_bits() as $t;
                let second_u = second.to_bits() as $t;
                (first_u << 32) | second_u
            }

            fn first_f32(&self) -> f32 {
                let first = ((self >> 32) & 0xFFFFFFFF) as u32;
                f32::from_bits(first)
            }

            fn second_f32(&self) -> f32 {
                let second = (self & 0xFFFFFFFF) as u32;
                f32::from_bits(second)
            }
        }
    };
}

impl_f32_packer_for_num!(u64);
#[cfg(target_pointer_width = "64")]
impl_f32_packer_for_num!(usize);
impl_f32_packer_for_num!(i64);
#[cfg(target_pointer_width = "64")]
impl_f32_packer_for_num!(isize);
