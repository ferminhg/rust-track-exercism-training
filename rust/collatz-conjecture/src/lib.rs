pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut value = n;
    let mut steps = 0;

    while value > 1 {
        value = match value % 2 {
            0 => value / 2,
            _ => {
                if value + 1 > u64::MAX / 3 {
                    return None;
                }
                3 * value + 1
            }
        };
        steps += 1;
    }
    Some(steps)
}
