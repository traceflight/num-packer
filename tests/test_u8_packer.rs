use num_packer::U8Packer;

#[test]
fn test_u8_packer_u16() {
    let packed = u16::pack_u8(200, 55);
    assert_eq!(packed, (200u8 as u16) << 8 | (55u8 as u16));
    let (first, second) = packed.unpack_u8();
    assert_eq!((first, second), (200, 55));
}

#[test]
fn test_u8_packer_u16_min() {
    let packed = u16::pack_u8(0, 0);
    assert_eq!(packed, (0u8 as u16) << 8 | (0u8 as u16));
    let (first, second) = packed.unpack_u8();
    assert_eq!((first, second), (0, 0));
}

#[test]
fn test_u8_packer_u16_max() {
    let packed = u16::pack_u8(255, 255);
    assert_eq!(packed, (255u8 as u16) << 8 | (255u8 as u16));
    let (first, second) = packed.unpack_u8();
    assert_eq!((first, second), (255, 255));
}

#[test]
fn test_u8_packer_u32() {
    let packed = u32::pack_u8(200, 55);
    assert_eq!(packed, (200u8 as u32) << 8 | (55u8 as u32));
    let (first, second) = packed.unpack_u8();
    assert_eq!((first, second), (200, 55));
}

#[test]
fn test_u8_packer_u32_min() {
    let packed = u32::pack_u8(0, 0);
    assert_eq!(packed, (0u8 as u32) << 8 | (0u8 as u32));
    let (first, second) = packed.unpack_u8();
    assert_eq!((first, second), (0, 0));
}

#[test]
fn test_u8_packer_u32_max() {
    let packed = u32::pack_u8(255, 255);
    assert_eq!(packed, (255u8 as u32) << 8 | (255u8 as u32));
    let (first, second) = packed.unpack_u8();
    assert_eq!((first, second), (255, 255));
}

#[test]
fn test_u8_packer_u64() {
    let packed = u64::pack_u8(200, 55);
    assert_eq!(packed, (200u8 as u64) << 8 | (55u8 as u64));
    let (first, second) = packed.unpack_u8();
    assert_eq!((first, second), (200, 55));
}

#[test]
fn test_u8_packer_usize() {
    let packed = usize::pack_u8(200, 55);
    assert_eq!(packed, (200u8 as usize) << 8 | (55u8 as usize));
    let (first, second) = packed.unpack_u8();
    assert_eq!((first, second), (200, 55));
}

#[test]
fn test_u8_packer_i16() {
    let packed = i16::pack_u8(200, 55);
    assert_eq!(packed, (200u8 as i16) << 8 | (55u8 as i16));
    let (first, second) = packed.unpack_u8();
    assert_eq!((first, second), (200, 55));
}

#[test]
fn test_u8_packer_i16_max() {
    let packed = i16::pack_u8(255, 255);
    assert_eq!(packed, (255u8 as i16) << 8 | (255u8 as i16));
    let (first, second) = packed.unpack_u8();
    assert_eq!((first, second), (255, 255));
}

#[test]
fn test_u8_packer_i32() {
    let packed = i32::pack_u8(200, 55);
    assert_eq!(packed, (200u8 as i32) << 8 | (55u8 as i32));
    let (first, second) = packed.unpack_u8();
    assert_eq!((first, second), (200, 55));
}

#[test]
fn test_u8_packer_i64() {
    let packed = i64::pack_u8(200, 55);
    assert_eq!(packed, (200u8 as i64) << 8 | (55u8 as i64));
    let (first, second) = packed.unpack_u8();
    assert_eq!((first, second), (200, 55));
}

#[test]
fn test_u8_packer_isize() {
    let packed = isize::pack_u8(200, 55);
    assert_eq!(packed, (200u8 as isize) << 8 | (55u8 as isize));
    let (first, second) = packed.unpack_u8();
    assert_eq!((first, second), (200, 55));
}
