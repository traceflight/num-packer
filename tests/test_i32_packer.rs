use num_packer::I32Packer;

#[test]
fn test_i32_packer_u64() {
    let packed = u64::pack_i32(-100, 1500000000);
    let (first, second) = packed.unpack_i32();
    assert_eq!((first, second), (-100, 1500000000));
}

#[test]
fn test_i32_packer_u64_zero() {
    let packed = u64::pack_i32(0, 0);
    let (first, second) = packed.unpack_i32();
    assert_eq!((first, second), (0, 0));
}

#[test]
fn test_i32_packer_u64_min() {
    let packed = u64::pack_i32(-2147483648, -2147483648);
    let (first, second) = packed.unpack_i32();
    assert_eq!((first, second), (-2147483648, -2147483648));
}

#[test]
fn test_i32_packer_u64_max() {
    let packed = u64::pack_i32(2147483647, 2147483647);
    let (first, second) = packed.unpack_i32();
    assert_eq!((first, second), (2147483647, 2147483647));
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_i32_packer_usize() {
    let packed = usize::pack_i32(-100, 1500000000);
    let (first, second) = packed.unpack_i32();
    assert_eq!((first, second), (-100, 1500000000));
}

#[test]
fn test_i32_packer_i64() {
    let packed = i64::pack_i32(-100, 1500000000);
    let (first, second) = packed.unpack_i32();
    assert_eq!((first, second), (-100, 1500000000));
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_i32_packer_isize() {
    let packed = isize::pack_i32(-100, 1500000000);
    let (first, second) = packed.unpack_i32();
    assert_eq!((first, second), (-100, 1500000000));
}
