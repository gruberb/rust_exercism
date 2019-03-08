pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    1 << s-1 
}

pub fn total() -> u64 {
    let mut sum = 0;

    for x in 1..=64 {
        sum += square(x);
    }

    sum
}
