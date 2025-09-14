use num_packer::I8Packer;

#[test]
fn test_i8_packer_u16() {
    let packed = u16::pack_i8(-100, 20);
    let (first, second) = packed.unpack_i8();
    assert_eq!((first, second), (-100, 20));
}

#[test]
fn test_i8_packer_u16_min() {
    let packed = u16::pack_i8(-128, -128);
    let (first, second) = packed.unpack_i8();
    assert_eq!((first, second), (-128, -128));
}

#[test]
fn test_i8_packer_u16_max() {
    let packed = u16::pack_i8(127, 127);
    let (first, second) = packed.unpack_i8();
    assert_eq!((first, second), (127, 127));
}

#[test]
fn test_i8_packer_u32() {
    let packed = u32::pack_i8(-125, 55);
    let (first, second) = packed.unpack_i8();
    assert_eq!((first, second), (-125, 55));
}

#[test]
fn test_i8_packer_u32_zero() {
    let packed = u32::pack_i8(0, 0);
    let (first, second) = packed.unpack_i8();
    assert_eq!((first, second), (0, 0));
}

#[test]
fn test_i8_packer_u64() {
    let packed = u64::pack_i8(-125, 55);
    let (first, second) = packed.unpack_i8();
    assert_eq!((first, second), (-125, 55));
}

#[test]
fn test_i8_packer_usize() {
    let packed = usize::pack_i8(-125, 55);
    let (first, second) = packed.unpack_i8();
    assert_eq!((first, second), (-125, 55));
}

#[test]
fn test_i8_packer_i16() {
    let packed = i16::pack_i8(-125, 55);
    let (first, second) = packed.unpack_i8();
    assert_eq!((first, second), (-125, 55));
}

#[test]
fn test_i8_packer_i32() {
    let packed = i32::pack_i8(-125, 55);
    let (first, second) = packed.unpack_i8();
    assert_eq!((first, second), (-125, 55));
}

#[test]
fn test_i8_packer_i64() {
    let packed = i64::pack_i8(-125, 55);
    let (first, second) = packed.unpack_i8();
    assert_eq!((first, second), (-125, 55));
}

#[test]
fn test_i8_packer_isize() {
    let packed = isize::pack_i8(-125, 55);
    let (first, second) = packed.unpack_i8();
    assert_eq!((first, second), (-125, 55));
}
