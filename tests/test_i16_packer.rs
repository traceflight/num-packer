use num_packer::I16Packer;

#[test]
fn test_i16_packer_u32() {
    let packed = u32::pack_i16(-100, 15000);
    assert_eq!(packed.first_i16(), -100);
    assert_eq!(packed.second_i16(), 15000);
    let (first, second) = packed.unpack_i16();
    assert_eq!((first, second), (-100, 15000));
}

#[test]
fn test_i16_packer_u32_zero() {
    let packed = u32::pack_i16(0, 0);
    assert_eq!(packed.first_i16(), 0);
    assert_eq!(packed.second_i16(), 0);
    let (first, second) = packed.unpack_i16();
    assert_eq!((first, second), (0, 0));
}

#[test]
fn test_i16_packer_u32_min() {
    let packed = u32::pack_i16(-32768, -32768);
    assert_eq!(packed.first_i16(), -32768);
    assert_eq!(packed.second_i16(), -32768);
    let (first, second) = packed.unpack_i16();
    assert_eq!((first, second), (-32768, -32768));
}

#[test]
fn test_i16_packer_u32_max() {
    let packed = u32::pack_i16(32767, 32767);
    assert_eq!(packed.first_i16(), 32767);
    assert_eq!(packed.second_i16(), 32767);
    let (first, second) = packed.unpack_i16();
    assert_eq!((first, second), (32767, 32767));
}

#[test]
fn test_i16_packer_u64() {
    let packed = u64::pack_i16(-100, 15000);
    assert_eq!(packed.first_i16(), -100);
    assert_eq!(packed.second_i16(), 15000);
    let (first, second) = packed.unpack_i16();
    assert_eq!((first, second), (-100, 15000));
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#[test]
fn test_i16_packer_usize() {
    let packed = usize::pack_i16(-100, 15000);
    assert_eq!(packed.first_i16(), -100);
    assert_eq!(packed.second_i16(), 15000);
    let (first, second) = packed.unpack_i16();
    assert_eq!((first, second), (-100, 15000));
}

#[test]
fn test_i16_packer_i32() {
    let packed = i32::pack_i16(-100, 15000);
    assert_eq!(packed.first_i16(), -100);
    assert_eq!(packed.second_i16(), 15000);
    let (first, second) = packed.unpack_i16();
    assert_eq!((first, second), (-100, 15000));
}

#[test]
fn test_i16_packer_i32_zero() {
    let packed = i32::pack_i16(0, 0);
    assert_eq!(packed.first_i16(), 0);
    assert_eq!(packed.second_i16(), 0);
    let (first, second) = packed.unpack_i16();
    assert_eq!((first, second), (0, 0));
}

#[test]
fn test_i16_packer_i64() {
    let packed = i64::pack_i16(-100, 15000);
    assert_eq!(packed.first_i16(), -100);
    assert_eq!(packed.second_i16(), 15000);
    let (first, second) = packed.unpack_i16();
    assert_eq!((first, second), (-100, 15000));
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#[test]
fn test_i16_packer_isize() {
    let packed = isize::pack_i16(-100, 15000);
    assert_eq!(packed.first_i16(), -100);
    assert_eq!(packed.second_i16(), 15000);
    let (first, second) = packed.unpack_i16();
    assert_eq!((first, second), (-100, 15000));
}
