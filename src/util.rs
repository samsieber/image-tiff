/// Named such to avoid conflict when compiling on rust 1.73+
/// Once we move to 1.73+, we can replace usages of this with the actual std method
pub fn usize_div_ceil(numerator: usize, denominator: usize) -> usize {
    (numerator + denominator - 1) / denominator
}

pub fn u64_div_ceil(numerator: u64, denominator: u64) -> u64 {
    (numerator + denominator - 1) / denominator
}