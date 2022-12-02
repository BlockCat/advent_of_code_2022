#![feature(test)]
extern crate test;
use self::test::Bencher;

include!("../examples/day_02.rs");

#[bench]
fn exercise_1_bench(b: &mut Bencher) {
    let input_text = include_str!("../input/day_02.txt")
        .lines()
        .map(|line| {
            let mut chars = line.chars();

            let a = chars.next().unwrap();
            let b = chars.nth(1).unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();
    b.iter(|| exercise_1(&input_text));
}

#[bench]
fn exercise_2_bench(b: &mut Bencher) {
    let input_text = include_str!("../input/day_02.txt")
        .lines()
        .map(|line| {
            let mut chars = line.chars();

            let a = chars.next().unwrap();
            let b = chars.nth(1).unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();
    b.iter(|| exercise_2(&input_text));
}
