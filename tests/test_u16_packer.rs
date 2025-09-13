use num_packer::U16Packer;

#[test]
fn test_u16_packer_u32() {
    let packed = u32::pack_u16(50000, 15000);
    assert_eq!(packed, (50000u16 as u32) << 16 | (15000u16 as u32));
    let (first, second) = packed.unpack_u16();
    assert_eq!((first, second), (50000, 15000));
}

#[test]
fn test_u16_packer_u32_min() {
    let packed = u32::pack_u16(0, 0);
    assert_eq!(packed, (0u16 as u32) << 16 | (0u16 as u32));
    let (first, second) = packed.unpack_u16();
    assert_eq!((first, second), (0, 0));
}

#[test]
fn test_u16_packer_u32_max() {
    let packed = u32::pack_u16(65535, 65535);
    assert_eq!(packed, (65535u16 as u32) << 16 | (65535u16 as u32));
    let (first, second) = packed.unpack_u16();
    assert_eq!((first, second), (65535, 65535));
}

#[test]
fn test_u16_packer_u64() {
    let packed = u64::pack_u16(50000, 15000);
    assert_eq!(packed, (50000u16 as u64) << 16 | (15000u16 as u64));
    let (first, second) = packed.unpack_u16();
    assert_eq!((first, second), (50000, 15000));
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#[test]
fn test_u16_packer_usize() {
    let packed = usize::pack_u16(50000, 15000);
    assert_eq!(packed, (50000u16 as usize) << 16 | (15000u16 as usize));
    let (first, second) = packed.unpack_u16();
    assert_eq!((first, second), (50000, 15000));
}

#[test]
fn test_u16_packer_i32() {
    let packed = i32::pack_u16(50000, 15000);
    assert_eq!(packed, (50000u16 as i32) << 16 | (15000u16 as i32));
    let (first, second) = packed.unpack_u16();
    assert_eq!((first, second), (50000, 15000));
}

#[test]
fn test_u16_packer_i32_min() {
    let packed = i32::pack_u16(0, 0);
    assert_eq!(packed, (0u16 as i32) << 16 | (0u16 as i32));
    let (first, second) = packed.unpack_u16();
    assert_eq!((first, second), (0, 0));
}

#[test]
fn test_u16_packer_i32_max() {
    let packed = i32::pack_u16(65535, 65535);
    assert_eq!(packed, (65535u16 as i32) << 16 | (65535u16 as i32));
    let (first, second) = packed.unpack_u16();
    assert_eq!((first, second), (65535, 65535));
}

#[test]
fn test_u16_packer_i64() {
    let packed = i64::pack_u16(50000, 15000);
    assert_eq!(packed, (50000u16 as i64) << 16 | (15000u16 as i64));
    let (first, second) = packed.unpack_u16();
    assert_eq!((first, second), (50000, 15000));
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#[test]
fn test_u16_packer_isize() {
    let packed = isize::pack_u16(50000, 15000);
    assert_eq!(packed, (50000u16 as isize) << 16 | (15000u16 as isize));
    let (first, second) = packed.unpack_u16();
    assert_eq!((first, second), (50000, 15000));
}
