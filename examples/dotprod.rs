/// Slightly modify the example on crate `simd` to make it work on stable channel 
/// Reference link : https://github.com/rust-lang-nursery/simd

extern crate ssimd;
use ssimd::{f32x4,f32x8};

#[inline(never)]
pub fn dot(x: &[f32], y: &[f32]) -> f32 {
    assert_eq!(x.len(), y.len());

    let len = std::cmp::min(x.len(), y.len());

    let mut sum = f32x4::splat(0.0);
    let mut i = 0;
    while i < len & !3 {
        let x = f32x4::load(x, i);
        let y = f32x4::load(y, i);
        sum = sum + x * y;
        i += 4
    }
    sum.extract(0) + sum.extract(1) + sum.extract(2) + sum.extract(3)
}

#[inline(never)]
pub fn dot8(x: &[f32], y: &[f32]) -> f32 {
    assert_eq!(x.len(), y.len());

    let len = std::cmp::min(x.len(), y.len());

    let mut sum = f32x8::splat(0.0);
    let mut i = 0;
    while i < len & !7 {
        let x = f32x8::load(x, i);
        let y = f32x8::load(y, i);
        sum = sum + x * y;
        i += 8
    }
    
    sum.extract(0) + sum.extract(1) + sum.extract(2) + sum.extract(3) +
        sum.extract(4) + sum.extract(5) + sum.extract(6) + sum.extract(7)
}

fn main() {
    println!("{}", dot(&[1.0, 3.0, 5.0, 7.0], &[2.0, 4.0, 6.0, 8.0]));
    println!("{}", dot(&[1.0, 3.0, 6.0, 7.0, 10.0, 6.0, 3.0, 2.0],
                       &[2.0, 4.0, 6.0, 8.0, 2.0, 4.0, 6.0, 8.0]));

    println!("{}", dot8(&[1.0, 3.0, 5.0, 7.0], &[2.0, 4.0, 6.0, 8.0]));
    println!("{}", dot8(&[1.0, 3.0, 6.0, 7.0, 10.0, 6.0, 3.0, 2.0],
                       &[2.0, 4.0, 6.0, 8.0, 2.0, 4.0, 6.0, 8.0]));
}
