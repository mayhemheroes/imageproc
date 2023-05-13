#![no_main]
use libfuzzer_sys::fuzz_target;
use imageproc::{geometry::approximate_polygon_dp, point::Point};

fuzz_target!(|input: (Vec<(u64, u64)>, f64, bool)| {
    let (curve, epsilon, closed) = input;
    let curve: Vec<Point<u64>> = curve.into_iter().map(|(x, y)| Point{ x, y }).collect();
    approximate_polygon_dp(curve.as_slice(), epsilon, closed);
});