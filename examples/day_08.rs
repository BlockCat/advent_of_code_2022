use aoc_2022::{
    direction::{Direction, ALL_DIRECTIONS},
    grid::{Grid, StaticGrid},
    vector::Vector2,
};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::collections::HashSet;

type IT = StaticGrid<u8>;

pub fn main() {
    let numbers = input();
    println!("1: {}", exercise_1(numbers.clone()));
    println!("2: {}", exercise_2(numbers));
}

fn input() -> IT {
    let l = include_str!("../input/day_08.txt")
        .lines()
        .map(|line| line.chars().map(|x| x as u8 - b'0').collect())
        .collect();
    StaticGrid::from_vec(l)
}

fn exercise_1(input: IT) -> usize {
    let mut visible = HashSet::new();
    let height = input.height as isize;
    let width = input.width as isize;

    let mut cc = |acc, x, y| {
        let val = *input.get_vec(&Vector2::new([x, y])).unwrap() as i32;
        if val > acc {
            visible.insert((x, y));
            val
        } else {
            acc
        }
    };

    for y in 0..height {
        (0..width).fold(-1i32, |acc, x| cc(acc, x, y));
        (0..width).rev().fold(-1i32, |acc, x| cc(acc, x, y));
    }
    for x in 0..width {
        (0..height).fold(-1i32, |acc, y| cc(acc, x, y));
        (0..height).rev().fold(-1i32, |acc, y| cc(acc, x, y));
    }

    visible.len()
}
fn exercise_2(input: IT) -> usize {
    (1..(input.width - 1))
        .flat_map(|x| (1..(input.height - 1)).map(move |y| Vector2::new([x as isize, y as isize])))
        .par_bridge()
        .map(|cord| {
            ALL_DIRECTIONS
                .into_iter()
                .map(|x| check_dir(cord, x, &input))
                .product::<usize>()
        })
        .max()
        .unwrap()
}

fn check_dir(sloc: Vector2, dir: Direction, input: &IT) -> usize {
    let height = *input.get_vec(&sloc).unwrap();
    let mut count = 0;
    let mut loc = sloc + dir;

    while let Some(x) = input.get_vec(&loc) {
        count += 1;
        if *x < height {
            loc = loc + dir;
        } else {
            return count;
        }
    }
    count
}
