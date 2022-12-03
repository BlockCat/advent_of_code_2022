#![feature(test)]
extern crate test;

use std::collections::HashSet;

use self::test::Bencher;

include!("../examples/day_03.rs");

#[bench]
fn d3_in(b: &mut Bencher) {
    b.iter(|| input());
}

#[bench]
fn d3_e1(b: &mut Bencher) {
    let a = input();
    b.iter(|| exercise_1(a.clone()));
}

#[bench]
fn d3_e2(b: &mut Bencher) {
    let a = input();

    b.iter(|| exercise_2(a.clone()));
}
