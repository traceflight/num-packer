use num_packer::BoolPacker;

#[test]
fn test_bool_packer_u8() {
    let packed = u8::pack_bool(true, false);
    assert_eq!(packed, 0b10);
    let (first, second) = packed.unpack_bool();
    assert_eq!((first, second), (true, false));
}

#[test]
fn test_bool_packer_u8_all_true() {
    let packed = u8::pack_bool(true, true);
    assert_eq!(packed, 0b11);
    let (first, second) = packed.unpack_bool();
    assert_eq!((first, second), (true, true));
}

#[test]
fn test_bool_packer_u16() {
    let packed = u16::pack_bool(true, false);
    assert_eq!(packed, 0b10);
    let (first, second) = packed.unpack_bool();
    assert_eq!((first, second), (true, false));
}

#[test]
fn test_bool_packer_u32() {
    let packed = u32::pack_bool(true, false);
    assert_eq!(packed, 0b10);
    let (first, second) = packed.unpack_bool();
    assert_eq!((first, second), (true, false));
}

#[test]
fn test_bool_packer_u64() {
    let packed = u64::pack_bool(true, false);
    assert_eq!(packed, 0b10);
    let (first, second) = packed.unpack_bool();
    assert_eq!((first, second), (true, false));
}

#[test]
fn test_bool_packer_usize() {
    let packed = usize::pack_bool(true, false);
    assert_eq!(packed, 0b10);
    let (first, second) = packed.unpack_bool();
    assert_eq!((first, second), (true, false));
}

#[test]
fn test_bool_packer_i8() {
    let packed = i8::pack_bool(true, false);
    assert_eq!(packed, 0b10);
    let (first, second) = packed.unpack_bool();
    assert_eq!((first, second), (true, false));
}

#[test]
fn test_bool_packer_i16() {
    let packed = i16::pack_bool(true, false);
    assert_eq!(packed, 0b10);
    let (first, second) = packed.unpack_bool();
    assert_eq!((first, second), (true, false));
}

#[test]
fn test_bool_packer_i32() {
    let packed = i32::pack_bool(true, false);
    assert_eq!(packed, 0b10);
    let (first, second) = packed.unpack_bool();
    assert_eq!((first, second), (true, false));
}

#[test]
fn test_bool_packer_i64() {
    let packed = i64::pack_bool(true, false);
    assert_eq!(packed, 0b10);
    let (first, second) = packed.unpack_bool();
    assert_eq!((first, second), (true, false));
}

#[test]
fn test_bool_packer_isize() {
    let packed = isize::pack_bool(true, false);
    assert_eq!(packed, 0b10);
    let (first, second) = packed.unpack_bool();
    assert_eq!((first, second), (true, false));
}
