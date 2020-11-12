#![feature(test)]

extern crate test;

use rand::prelude::*;
use test::Bencher;

#[bench]
fn bench_cache_rng(b: &mut Bencher) {
    let mut rng = rand::thread_rng();

    b.iter(|| {
        (0..100).for_each(|i| {
            rng.gen::<u32>();
        });
    });
}
#[bench]
fn bench_no_cache_rng(b: &mut Bencher) {
    b.iter(|| {
        (0..100).for_each(|i| {
            rand::thread_rng().gen::<u32>();
        });
    });
}

#[test]
fn it_adds_two() {
    assert_eq!(4, 4);
}
