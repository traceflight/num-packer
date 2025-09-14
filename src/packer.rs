/// Trait for packing and unpacking two `bool` values into a single value.
pub trait BoolPacker {
    /// Packs two `bool` values into a single value of the implementing type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::BoolPacker;
    /// let packed = u8::pack_bool(true, false);
    /// assert_eq!(packed, 0b10);
    /// ```
    fn pack_bool(first: bool, second: bool) -> Self;

    /// Gets the first `bool` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::BoolPacker;
    /// let packed = u8::pack_bool(true, false);
    /// assert_eq!(packed.first_bool(), true);
    /// ```
    fn first_bool(&self) -> bool;

    /// Gets the second `bool` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::BoolPacker;
    /// let packed = u8::pack_bool(true, false);
    /// assert_eq!(packed.second_bool(), false);
    /// ```
    fn second_bool(&self) -> bool;

    /// Unpacks the single value back into two `bool` values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::BoolPacker;
    /// let packed = u8::pack_bool(true, false);
    /// let (first, second) = packed.unpack_bool();
    /// assert_eq!((first, second), (true, false));
    /// ```
    fn unpack_bool(&self) -> (bool, bool) {
        (self.first_bool(), self.second_bool())
    }
}

/// Trait for packing and unpacking two `u8` values into a single value.
pub trait U8Packer {
    /// Packs two `u8` values into a single value of the implementing type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::U8Packer;
    /// let packed = u16::pack_u8(200, 55);
    /// assert_eq!(packed, (200 << 8) + 55);
    /// ```
    fn pack_u8(first: u8, second: u8) -> Self;

    /// Gets the first `u8` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::U8Packer;
    /// let packed = u16::pack_u8(200, 55);
    /// assert_eq!(packed.first_u8(), 200);
    /// ```
    fn first_u8(&self) -> u8;

    /// Gets the second `u8` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::U8Packer;
    /// let packed = u16::pack_u8(200, 55);
    /// assert_eq!(packed.second_u8(), 55);
    /// ```
    fn second_u8(&self) -> u8;

    /// Unpacks the single value back into two `u8` values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::U8Packer;
    /// let packed = u16::pack_u8(200, 55);
    /// let (first, second) = packed.unpack_u8();
    /// assert_eq!((first, second), (200, 55));
    /// ```
    fn unpack_u8(&self) -> (u8, u8) {
        (self.first_u8(), self.second_u8())
    }
}

/// Trait for packing and unpacking two `u16` values into a single value.
pub trait U16Packer {
    /// Packs two `u16` values into a single value of the implementing type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::U16Packer;
    /// let packed = u32::pack_u16(50000, 40000);
    /// assert_eq!(packed, (50000 << 16) + 40000);
    /// ```
    fn pack_u16(first: u16, second: u16) -> Self;

    /// Gets the first `u16` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::U16Packer;
    /// let packed = u32::pack_u16(50000, 40000);
    /// assert_eq!(packed.first_u16(), 50000);
    /// ```
    fn first_u16(&self) -> u16;

    /// Gets the second `u16` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::U16Packer;
    /// let packed = u32::pack_u16(50000, 40000);
    /// assert_eq!(packed.second_u16(), 40000);
    /// ```
    fn second_u16(&self) -> u16;

    /// Unpacks the single value back into two `u16` values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::U16Packer;
    /// let packed = u32::pack_u16(50000, 40000);
    /// let (first, second) = packed.unpack_u16();
    /// assert_eq!((first, second), (50000, 40000));
    /// ```
    fn unpack_u16(&self) -> (u16, u16) {
        (self.first_u16(), self.second_u16())
    }
}

/// Trait for packing and unpacking two `u32` values into a single value.
pub trait U32Packer {
    /// Packs two `u32` values into a single value of the implementing type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::U32Packer;
    /// let packed = u64::pack_u32(200, 55);
    /// assert_eq!(packed, (200u64 << 32) + 55);
    /// ```
    fn pack_u32(first: u32, second: u32) -> Self;

    /// Gets the first `u32` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::U32Packer;
    /// let packed = u64::pack_u32(200, 55);
    /// assert_eq!(packed.first_u32(), 200);
    /// ```
    fn first_u32(&self) -> u32;

    /// Gets the second `u32` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::U32Packer;
    /// let packed = u64::pack_u32(200, 55);
    /// assert_eq!(packed.second_u32(), 55);
    /// `
    fn second_u32(&self) -> u32;

    /// Unpacks the single value back into two `u32` values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::U32Packer;
    /// let packed = u64::pack_u32(200, 55);
    /// let (first, second) = packed.unpack_u32();
    /// assert_eq!((first, second), (200, 55));
    /// ```
    fn unpack_u32(&self) -> (u32, u32) {
        (self.first_u32(), self.second_u32())
    }
}

/// Trait for packing and unpacking two `i8` values into a single value.
pub trait I8Packer {
    /// Packs two `i8` values into a single value of the implementing type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::I8Packer;
    /// let packed = u16::pack_i8(-100, 55);
    /// assert_eq!(packed, ((-100i8 as u8 as u16) << 8) + (55u8 as u16));
    /// ```
    fn pack_i8(first: i8, second: i8) -> Self;

    /// Gets the first `i8` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::I8Packer;
    /// let packed = u16::pack_i8(-100, 55);
    /// assert_eq!(packed.first_i8(), -100);
    /// ```
    fn first_i8(&self) -> i8;

    /// Gets the second `i8` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::I8Packer;
    /// let packed = u16::pack_i8(-100, 55);
    /// assert_eq!(packed.second_i8(), 55);
    /// ```
    fn second_i8(&self) -> i8;

    /// Unpacks the single value back into two `i8` values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::I8Packer;
    /// let packed = u16::pack_i8(-100, 55);
    /// let (first, second) = packed.unpack_i8();
    /// assert_eq!((first, second), (-100, 55));
    /// ```
    fn unpack_i8(&self) -> (i8, i8) {
        (self.first_i8(), self.second_i8())
    }
}

/// Trait for packing and unpacking two `i16` values into a single value.
pub trait I16Packer {
    /// Packs two `i16` values into a single value of the implementing type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::I16Packer;
    /// let packed = u32::pack_i16(-20000, 15000);
    /// assert_eq!(packed, ((-20000i16 as u16 as u32) << 16) + (15000u16 as u32));
    /// ```
    fn pack_i16(first: i16, second: i16) -> Self;

    /// Gets the first `i16` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::I16Packer;
    /// let packed = u32::pack_i16(-20000, 15000);
    /// assert_eq!(packed.first_i16(), -20000);
    /// ```
    fn first_i16(&self) -> i16;

    /// Gets the second `i16` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::I16Packer;
    /// let packed = u32::pack_i16(-20000, 15000);
    /// assert_eq!(packed.second_i16(), 15000);
    /// ```
    fn second_i16(&self) -> i16;

    /// Unpacks the single value back into two `i16` values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::I16Packer;
    /// let packed = u32::pack_i16(-20000, 15000);
    /// let (first, second) = packed.unpack_i16();
    /// assert_eq!((first, second), (-20000, 15000));
    /// ```
    fn unpack_i16(&self) -> (i16, i16) {
        (self.first_i16(), self.second_i16())
    }
}

/// Trait for packing and unpacking two `i32` values into a single value.
pub trait I32Packer {
    /// Packs two `i32` values into a single value of the implementing type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::I32Packer;
    /// let packed = u64::pack_i32(-200, 15);
    /// assert_eq!(packed, ((-200i32 as u32 as u64) << 32) + (15u32 as u64));
    /// ```
    fn pack_i32(first: i32, second: i32) -> Self;

    /// Gets the first `i32` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::I32Packer;
    /// let packed = u64::pack_i32(-200, 15);
    /// assert_eq!(packed.first_i32(), -200);
    /// ```
    fn first_i32(&self) -> i32;

    /// Gets the second `i32` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::I32Packer;
    /// let packed = u64::pack_i32(-200, 15);
    /// assert_eq!(packed.second_i32(), 15);
    /// ```
    fn second_i32(&self) -> i32;

    /// Unpacks the single value back into two `i32` values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::I32Packer;
    /// let packed = u64::pack_i32(-200, 15);
    /// let (first, second) = packed.unpack_i32();
    /// assert_eq!((first, second), (-200, 15));
    /// ```
    fn unpack_i32(&self) -> (i32, i32) {
        (self.first_i32(), self.second_i32())
    }
}

/// Trait for packing and unpacking two `f32` values into a single value.
pub trait F32Packer {
    /// Packs two `f32` values into a single value of the implementing type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::F32Packer;
    /// let packed = u64::pack_f32(1.5, -2.5);
    /// ```
    fn pack_f32(first: f32, second: f32) -> Self;

    /// Gets the first `f32` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::F32Packer;
    /// let packed = u64::pack_f32(1.5, -2.5);
    /// assert_eq!(packed.first_f32(), 1.5);
    /// ```
    fn first_f32(&self) -> f32;

    /// Gets the second `f32` value from the packed representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::F32Packer;
    /// let packed = u64::pack_f32(1.5, -2.5);
    /// assert_eq!(packed.second_f32(), -2.5);
    /// ```
    fn second_f32(&self) -> f32;

    /// Unpacks the single value back into two `f32` values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_packer::F32Packer;
    /// let packed = u64::pack_f32(1.5, -2.5);
    /// let (first, second) = packed.unpack_f32();
    /// assert_eq!((first, second), (1.5, -2.5));
    /// ```
    fn unpack_f32(&self) -> (f32, f32) {
        (self.first_f32(), self.second_f32())
    }
}
