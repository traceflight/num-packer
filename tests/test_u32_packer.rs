use num_packer::U32Packer;

#[test]
fn test_u32_packer_u64() {
    let packed = u64::pack_u32(3000000000, 1500000000);
    assert_eq!(
        packed,
        (3000000000u32 as u64) << 32 | (1500000000u32 as u64)
    );
    let (first, second) = packed.unpack_u32();
    assert_eq!((first, second), (3000000000, 1500000000));
}

#[test]
fn test_u32_packer_u64_min() {
    let packed = u64::pack_u32(0, 0);
    assert_eq!(packed, (0u32 as u64) << 32 | (0u32 as u64));
    let (first, second) = packed.unpack_u32();
    assert_eq!((first, second), (0, 0));
}

#[test]
fn test_u32_packer_u64_max() {
    let packed = u64::pack_u32(4294967295, 4294967295);
    assert_eq!(
        packed,
        (4294967295u32 as u64) << 32 | (4294967295u32 as u64)
    );
    let (first, second) = packed.unpack_u32();
    assert_eq!((first, second), (4294967295, 4294967295));
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_u32_packer_usize() {
    let packed = usize::pack_u32(3000000000, 1500000000);
    assert_eq!(
        packed,
        (3000000000u32 as usize) << 32 | (1500000000u32 as usize)
    );
    let (first, second) = packed.unpack_u32();
    assert_eq!((first, second), (3000000000, 1500000000));
}

#[test]
fn test_u32_packer_i64() {
    let packed = i64::pack_u32(3000000000, 1500000000);
    assert_eq!(
        packed,
        (3000000000u32 as i64) << 32 | (1500000000u32 as i64)
    );
    let (first, second) = packed.unpack_u32();
    assert_eq!((first, second), (3000000000, 1500000000));
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_u32_packer_isize() {
    let packed = isize::pack_u32(3000000000, 1500000000);
    assert_eq!(
        packed,
        (3000000000u32 as isize) << 32 | (1500000000u32 as isize)
    );
    let (first, second) = packed.unpack_u32();
    assert_eq!((first, second), (3000000000, 1500000000));
}
