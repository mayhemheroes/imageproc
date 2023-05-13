#![no_main]
use libfuzzer_sys::fuzz_target;
use imageproc::{geometry::arc_length, point::Point};

fuzz_target!(|input: (Vec<(u64, u64)>, bool)| {
    let (points, closed) = input;
    let points_vec: Vec<Point<u64>> = points.iter().map(|(x, y)| Point{ x: *x, y: *y }).collect();
    arc_length(points_vec.as_slice(), closed);
});