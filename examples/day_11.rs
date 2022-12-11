#![feature(portable_simd)]
use std::{collections::HashMap, simd::Simd};

type InputType = [Monkey; 8];

pub fn main() {
    let numbers = input();
    println!("Exercise 1: {}", exercise_1(numbers));

    let numbers = input();
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    [
        Monkey {
            start_items: vec![74, 64, 74, 63, 53],
            operations: |old| old * 7,
            divisible: 5,
            next_monkey: [1, 6],
        },
        Monkey {
            start_items: vec![69, 99, 95, 62],
            operations: |old| old.pow(2),
            divisible: 17,
            next_monkey: [2, 5],
        },
        Monkey {
            start_items: vec![59, 81],
            operations: |old| old + 8,
            divisible: 7,
            next_monkey: [4, 3],
        },
        Monkey {
            start_items: vec![50, 67, 63, 57, 63, 83, 97],
            operations: |old| old + 4,
            divisible: 13,
            next_monkey: [0, 7],
        },
        Monkey {
            start_items: vec![61, 94, 85, 52, 81, 90, 94, 70],
            operations: |old| old + 3,
            divisible: 19,
            next_monkey: [7, 3],
        },
        Monkey {
            start_items: vec![69],
            operations: |old| old + 5,
            divisible: 3,
            next_monkey: [4, 2],
        },
        Monkey {
            start_items: vec![54, 55, 58],
            operations: |old| old + 7,
            divisible: 11,
            next_monkey: [1, 5],
        },
        Monkey {
            start_items: vec![79, 51, 83, 88, 93, 76],
            operations: |old| old * 3,
            divisible: 2,
            next_monkey: [0, 6],
        },
    ]
}

fn exercise_1(mut input: InputType) -> usize {
    // input.iter_mut().for_each(|x| x.start_items.reserve(30));
    let mut inspections = (0..20).fold([0; 8], |mut acc, _| {
        for monkey in 0..input.len() {
            while let Some(worry) = &input[monkey].start_items.pop() {
                acc[monkey] += 1;
                let (next_worry, next_monkey) = inspect(*worry, &input[monkey]);
                input[next_monkey].start_items.push(next_worry as usize);
            }
        }
        acc
    });

    inspections.sort();
    inspections.into_iter().rev().take(2).product()
}

fn inspect(worry: usize, monkey: &Monkey) -> (usize, usize) {
    let next_worry = (monkey.operations)(worry) / 3usize;
    let next_monkey = monkey.next_monkey[(next_worry % monkey.divisible == 0) as usize];
    (next_worry, next_monkey)
}

fn exercise_2(mut input: InputType) -> usize {
    let lessen = input.iter().map(|x| x.divisible).product::<usize>();
    // input.iter_mut().for_each(|x| x.start_items.reserve(30));

    let mut inspections = [0; 8];
    let mut visited = HashMap::with_capacity(1_000);

    let mut counter = 0;
    const MAX: usize = 10_000;

    while counter < MAX {
        monkey_round(&mut input, &mut inspections, lessen);
        counter += 1;

        let hash = input
            .iter()
            .map(|x| x.start_items.clone())
            .collect::<Vec<_>>();

        let grep = visited.insert(hash, (counter, inspections.clone()));

        if let Some((a, old_reps)) = grep {
            let cycle_length = counter - a;
            let repetitions = (MAX - counter) / cycle_length;
            let i = Simd::from_array(inspections);
            let old = Simd::from_array(old_reps);
            let diff = i - old;
            let diff = diff * Simd::splat(repetitions);
            let ne: Simd<usize, 8> = i + diff;

            inspections = ne.to_array();

            counter += repetitions * cycle_length;

            break;
        }
    }

    while counter < MAX {
        counter += 1;
        monkey_round(&mut input, &mut inspections, lessen);
        // if counter % 1000 == 0 || counter > 9990 || counter < 10 {
        //     println!("c{}", counter);
        // }
    }

    inspections.sort();
    inspections.into_iter().rev().take(2).product()
}

fn monkey_round(input: &mut [Monkey; 8], inspections: &mut [usize; 8], lessen: usize) {
    for monkey in 0..input.len() {
        while let Some(worry) = &input[monkey].start_items.pop() {
            inspections[monkey] += 1;
            let (next_worry, next_monkey) = inspect2(*worry, &input[monkey], lessen);
            input[next_monkey].start_items.push(next_worry as usize);
        }
    }
}

fn inspect2(worry: usize, monkey: &Monkey, lessen: usize) -> (usize, usize) {
    let next_worry = (monkey.operations)(worry) % lessen;

    let next_monkey = monkey.next_monkey[1 - (next_worry % monkey.divisible == 0) as usize];

    (next_worry, next_monkey)
}

struct Monkey {
    start_items: Vec<usize>,
    operations: fn(usize) -> usize,
    divisible: usize,
    next_monkey: [usize; 2],
}

// fn test_input() -> InputType {
//     vec![
//         Monkey {
//             start_items: (vec![79, 98]),
//             operations: Box::new(|old| old * 19),
//             divisible: 23,
//             true_test: 2,
//             false_test: 3,
//         },
//         Monkey {
//             start_items: (vec![54, 65, 75, 74]),
//             operations: Box::new(|old| old + 6),
//             divisible: 19,
//             true_test: 2,
//             false_test: 0,
//         },
//         Monkey {
//             start_items: (vec![79, 60, 97]),
//             operations: Box::new(|old| old * old),
//             divisible: 13,
//             true_test: 1,
//             false_test: 3,
//         },
//         Monkey {
//             start_items: (vec![74]),
//             operations: Box::new(|old| old + 3),
//             divisible: 17,
//             true_test: 0,
//             false_test: 1,
//         },
//     ]
// }
