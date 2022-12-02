const WIN_TABLE: [usize; 9] = [3, 6, 0, 0, 3, 6, 6, 0, 3];
const ROUND_TABLE: [usize; 9] = [3, 1, 2, 1, 2, 3, 2, 3, 1];

pub fn main() {
    let input_text = include_str!("../input/day_02.txt")
        .lines()
        .map(|line| {
            let mut chars = line.chars();

            let a = chars.next().unwrap();
            let b = chars.nth(1).unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();

    println!("Exercise 1: {}", exercise_1(&input_text));
    println!("Exercise 2: {}", exercise_2(&input_text));
}

fn exercise_1(input: &Vec<(char, char)>) -> usize {
    input
        .iter()
        .map(|(a, b)| {
            let a = *a as u8 - b'A';
            let b = *b as u8 - b'X';
            1 + b as usize + WIN_TABLE[(a * 3 + b) as usize]
        })
        .sum()
}

fn exercise_2(input: &Vec<(char, char)>) -> usize {
    input
        .iter()
        .map(|(da, db)| {
            let a = *da as u8 - b'A';
            let b = *db as u8 - b'X';
            3 * b as usize + ROUND_TABLE[(a * 3 + b) as usize]
        })
        .sum()
}
