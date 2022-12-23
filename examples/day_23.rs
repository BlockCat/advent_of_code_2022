type InputType = Vec<u32>;

pub fn main() {
    let numbers = input();

    // println!("Exercise 1: {}", exercise_1(numbers.clone()));
    // println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    include_str!("../input/day_23.txt").lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> usize {

}

fn exercise_1(input: InputType) -> usize {
    unimplemented!()
}
fn exercise_2(input: InputType) -> usize {
    unimplemented!()    
}