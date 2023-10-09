use std::ops::RangeInclusive;
use rand::Rng;

pub fn div_ceil(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

pub fn rand_product_in_range<R: Rng + ?Sized>(a: u32, range: RangeInclusive<u32>, rng: &mut R) -> u32 {
    let lower = div_ceil(*range.start(), a);
    let upper = *range.end() / a;
    rng.gen_range(lower..=upper)
}