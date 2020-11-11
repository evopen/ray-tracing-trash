#![feature(test)]

extern crate test;

use test::Bencher;
use rand::prelude::*;

#[bench]
fn bench_xor_1000_ints(b: &mut Bencher) {
    let mut rng = rand::thread_rng();

    b.iter(|| {
        rng.gen::<u32>();
    });
}

#[test]
fn it_adds_two() {
    assert_eq!(4, 4);
}