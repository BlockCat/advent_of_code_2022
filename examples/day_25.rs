type InputType = Vec<isize>;

pub fn main() {
    let numbers = input();

    let l = "2=-0=1-0012-=-2=0=01";
    let r = parse_line(l);
    println!(
        "s: {}, r: {}, t: {}, ok: {}",
        l,
        r,
        29698499442451isize,
        29698499442451 == r
    );

    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    // println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    include_str!("../input/day_25.txt")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> isize {
    line.chars().rev().enumerate().fold(0, |acc, (index, c)| {
        let pow = 5isize.pow(index as u32);
        match c {
            '=' => acc - pow * 2,
            '-' => acc - pow,
            '0' => acc,
            '1' => acc + pow,
            '2' => acc + pow * 2,
            _ => unreachable!(),
        }
    })
}

fn exercise_1(input: InputType) -> String {
    let x: isize = input.into_iter().sum();

    to_string(x)
}
#[cfg(test)]
mod tests {
    use crate::parse_line;

    // #[test]
    fn a() {
        assert_eq!(1, parse_line("1"));
        // 1=-0-2     1747
        assert_eq!(parse_line("12111"), 906);
        assert_eq!(parse_line("2=0="), 198);
        assert_eq!(parse_line("21"), 11);
        assert_eq!(parse_line("2=01"), 201);
        assert_eq!(parse_line("111"), 31);
        assert_eq!(parse_line("20012"), 1257);
        assert_eq!(parse_line("112"), 32);
        assert_eq!(parse_line("1=-1="), 353);
        assert_eq!(parse_line("1-12"), 107);
        assert_eq!(parse_line("12"), 7);
        assert_eq!(parse_line("1="), 3);
        assert_eq!(parse_line("122"), 37);
    }

    #[test]
    fn c() {
        for x in -2..40 {
            println!("x {:03}: {}", x, super::to_string(x));
        }
    }
}

fn to_string(mut number: isize) -> String {
    number += 2;

    let mut n = String::new();

    let translate = ['=', '-', '0', '1', '2'];

    if number == 0 {
        return translate[0].to_string();
    }

    let steps = number.ilog(5) + 1;

    for _ in 0..=steps {
        let c = translate[(number) as usize % 5];
        number /= 5;

        n.push(c);
        number += 2;
    }

    if n.ends_with('0') {
        n.pop();    
    }

    n.chars().rev().collect()


}
