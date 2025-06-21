fn main() {
    let mut period = 0u64;
    let mut next = 0u32;

    while {
        next = next.wrapping_mul(1_103_515_245).wrapping_add(12345);
        period += 1;
        next != 0
    } {}

    assert_eq!(period, 32);
}
