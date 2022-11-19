pub fn square(s: u32) -> u64 {
    assert!(s >= 1 && s <= 64, "Square must be between 1 and 64");
    2_u64.pow(s-1 as u32)
}

pub fn total() -> u64 {
    u64::max_value()
}
