#![no_main]
use libfuzzer_sys::fuzz_target;
use imageproc::{geometry::convex_hull, point::Point};

fuzz_target!(|points_vec: Vec<(u64, u64)>| {
    if points_vec.iter().any(|(x, y)| *x > 10000 || *y > 10000) {
        return;
    }
    let points: Vec<Point<u64>> = points_vec.into_iter().map(|(x, y)| Point{ x, y }).collect();
    convex_hull(points.as_slice());
});