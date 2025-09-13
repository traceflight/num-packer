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
    fn unpack_bool(&self) -> (bool, bool);
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
    fn unpack_u8(&self) -> (u8, u8);
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
    fn unpack_u16(&self) -> (u16, u16);
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
    fn unpack_u32(&self) -> (u32, u32);
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
    fn unpack_i8(&self) -> (i8, i8);
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
    fn unpack_i16(&self) -> (i16, i16);
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
    fn unpack_i32(&self) -> (i32, i32);
}
