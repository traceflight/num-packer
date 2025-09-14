use num_packer::F32Packer;

#[test]
fn test_f32_packer_u64() {
    let packed = u64::pack_f32(1.5, -2.5);
    let (first, second) = packed.unpack_f32();
    assert_eq!((first, second), (1.5, -2.5));
}

#[test]
fn test_f32_packer_u64_zero() {
    let packed = u64::pack_f32(0.0, 0.0);
    let (first, second) = packed.unpack_f32();
    assert_eq!((first, second), (0.0, 0.0));
}

#[test]
fn test_f32_packer_u64_max() {
    let packed = u64::pack_f32(f32::MAX, f32::MAX);
    let (first, second) = packed.unpack_f32();
    assert_eq!((first, second), (f32::MAX, f32::MAX));
}

#[test]
fn test_f32_packer_u64_min() {
    let packed = u64::pack_f32(f32::MIN, f32::MIN);
    let (first, second) = packed.unpack_f32();
    assert_eq!((first, second), (f32::MIN, f32::MIN));
}

#[test]
fn test_f32_packer_i64() {
    let packed = i64::pack_f32(1.5, -2.5);
    let (first, second) = packed.unpack_f32();
    assert_eq!((first, second), (1.5, -2.5));
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_f32_packer_usize() {
    let packed = usize::pack_f32(1.5, -2.5);
    let (first, second) = packed.unpack_f32();
    assert_eq!((first, second), (1.5, -2.5));
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_f32_packer_isize() {
    let packed = isize::pack_f32(1.5, -2.5);
    let (first, second) = packed.unpack_f32();
    assert_eq!((first, second), (1.5, -2.5));
}
