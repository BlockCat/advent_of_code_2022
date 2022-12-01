use std::collections::BinaryHeap;

pub fn main() {
    let heap = include_str!("../input/day_01.txt")
        .split("\n\n")        
        .map(|set| set.lines().flat_map(|line| line.parse::<u32>().ok()).sum())
        .collect::<BinaryHeap<u32>>();

    println!("Ex1: {}", heap.peek().unwrap());
    println!("Ex2: {}", heap.iter().take(3).sum::<u32>());
}
