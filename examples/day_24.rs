use aoc_2022::{
    direction::{Direction, ALL_DIRECTIONS},
    grid::{Grid, StaticGrid},
    vector::{Vector2, VectorN},
};
use rayon::prelude::*;
use std::{
    borrow::{BorrowMut, Cow},
    cell::{Cell, RefCell},
    collections::{HashSet, VecDeque},
    ops::DerefMut,
    sync::atomic::AtomicU8,
};

type InputType = StaticGrid<char>;

pub fn main() {
    let numbers = input();

    // println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    StaticGrid::from_vec(
        include_str!("../input/day_24.txt")
            .lines()
            .map(parse_line)
            .collect(),
    )
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn exercise_1(input: InputType) -> usize {
    let start = Vector2::new([1, 0]);
    let end = Vector2::new([input.width as isize - 2, input.height as isize - 1]);

    assert_eq!(Some(&'.'), input.get_vec(&start));
    assert_eq!(Some(&'.'), input.get_vec(&end));
    assert_eq!(Some(&'#'), input.get_vec(&(start + Direction::East)));
    assert_eq!(Some(&'#'), input.get_vec(&(start + Direction::West)));
    assert_eq!(Some(&'#'), input.get_vec(&(end + Direction::East)));
    assert_eq!(Some(&'#'), input.get_vec(&(end + Direction::West)));

    let blizzards = extract_blizzards(input.clone());

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_front((0, start, blizzards));

    while let Some((steps, pos, field)) = queue.pop_back() {
        assert!(field.get_vec(&pos).unwrap() == &0);
        if pos == end {
            return steps;
        }

        if !visited.insert((pos, field.clone())) {
            continue;
        }

        let field = blizzard_step(field);

        for dir in ALL_DIRECTIONS {
            let pos = pos + dir;
            if field.get_vec(&pos).unwrap_or(&std::u8::MAX) == &0
                && input.get_vec(&pos) != Some(&'#')
            {
                // println!("next pos: {}, {:?}", steps + 1, pos);
                queue.push_front((steps + 1, pos, field.clone()));
            }
        }

        if field.get_vec(&pos).unwrap_or(&std::u8::MAX) == &0 {
            queue.push_front((steps + 1, pos, field.clone()));
        }
    }

    unreachable!()
}

fn extract_blizzards(input: StaticGrid<char>) -> StaticGrid<u8> {
    let blizzards = input
        .iter()
        .filter_map(|(a, b)| {
            let dir = match b {
                '>' => Some(Direction::East),
                '<' => Some(Direction::West),
                '^' => Some(Direction::North),
                'v' => Some(Direction::South),
                _ => None,
            };
            dir.map(|d| (a, d))
        })
        .collect::<Vec<_>>();
    let blizzards = {
        let mut x: StaticGrid<u8> = StaticGrid::new(input.width, input.height);

        for (v, d) in blizzards {
            *x.get_mut_vec(&v).unwrap() |= 1 << d as u8;
        }
        x
    };

    blizzards
}

fn blizzard_step(grid: StaticGrid<u8>) -> StaticGrid<u8> {
    let mut ngrid = StaticGrid::new(grid.width, grid.height);

    grid.iter()
        .par_bridge()
        // .filter(|x| x.1 > &0)
        .flat_map(|(pos, dirs)| {
            ALL_DIRECTIONS
                .into_par_iter()
                .filter_map(move |dir| {
                    if (dirs & (1 << dir as u8)) != 0 {
                        let p = pos + dir;
                        let p = match dir {
                            Direction::North if p[1] == 0 => {
                                Vector2::new([p[0], grid.height as isize - 2])
                            }
                            Direction::South if p[1] == grid.height as isize - 1 => {
                                Vector2::new([p[0], 1])
                            }
                            Direction::East if p[0] == grid.width as isize - 1 => {
                                Vector2::new([1, p[1]])
                            }
                            Direction::West if p[0] == 0 => {
                                Vector2::new([grid.width as isize - 2, p[1]])
                            }
                            _ => p,
                        };
                        Some((p, (1 << dir as u8) as u8))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(pos, x)| {
            let index = pos[0] + pos[1] * ngrid.width as isize;
            ngrid.grid[index as usize] |= x;
        });

    ngrid
}

fn exercise_2(input: InputType) -> usize {
    let start = Vector2::new([1, 0]);
    let end = Vector2::new([input.width as isize - 2, input.height as isize - 1]);

    assert_eq!(Some(&'.'), input.get_vec(&start));
    assert_eq!(Some(&'.'), input.get_vec(&end));
    assert_eq!(Some(&'#'), input.get_vec(&(start + Direction::East)));
    assert_eq!(Some(&'#'), input.get_vec(&(start + Direction::West)));
    assert_eq!(Some(&'#'), input.get_vec(&(end + Direction::East)));
    assert_eq!(Some(&'#'), input.get_vec(&(end + Direction::West)));

    let blizzards = extract_blizzards(input.clone());

    let (steps_1, blizzards) = find_path(start, blizzards, end, &input);
    println!("forward: {}", steps_1);
    let (steps_2, blizzards) = find_path(end, blizzards, start, &input);
    println!("backward: {}", steps_2);
    let (steps_3, _) = find_path(start, blizzards, end, &input);
    println!("forward: {}", steps_3);

    steps_1 + steps_2 + steps_3
}

fn find_path(
    start: VectorN<2>,
    blizzards: StaticGrid<u8>,
    end: VectorN<2>,
    input: &StaticGrid<char>,
) -> (usize, StaticGrid<u8>) {
    let mut queue: VecDeque<(usize, VectorN<2>, Cow<StaticGrid<u8>>)> = VecDeque::new();
    let mut visited: HashSet<(VectorN<2>, Cow<StaticGrid<u8>>)> = HashSet::new();
    queue.push_front((0, start, Cow::Owned(blizzards)));
    while let Some((steps, pos, field)) = queue.pop_back() {
        // assert!(field.get_vec(&pos).unwrap() == &0);
        if pos == end {
            return (steps, field.into_owned());
        }

        if !visited.insert((pos, field.clone())) {
            continue;
        }

        let field: Cow<StaticGrid<u8>> = Cow::Owned(blizzard_step(field.into_owned()));

        for dir in ALL_DIRECTIONS {
            let pos = pos + dir;
            if field.get_vec(&pos).unwrap_or(&std::u8::MAX) == &0
                && input.get_vec(&pos) != Some(&'#')
            {
                // println!("next pos: {}, {:?}", steps + 1, pos);
                queue.push_front((steps + 1, pos, field.clone()));
            }
        }

        if field.get_vec(&pos).unwrap_or(&std::u8::MAX) == &0 {
            queue.push_front((steps + 1, pos, field.clone()));
        }
    }
    unreachable!()
}

fn print_blizzards(grid: &StaticGrid<u8>) {
    for y in 1..grid.height as isize - 1 {
        for x in 1..grid.width as isize - 1 {
            let d = grid.get(x, y).unwrap();
            print!("{}", d.count_ones());
        }
        println!();
    }
}
