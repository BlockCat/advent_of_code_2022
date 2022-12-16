#![feature(is_sorted)]

use std::collections::{HashMap, HashSet};

type InputType = Vec<(String, usize, Vec<String>)>;

pub fn main() {
    let numbers = input();

    // println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    include_str!("../input/test.txt")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> (String, usize, Vec<String>) {
    let mut parts = line.split(' ');

    let valve = String::from(parts.nth(1).unwrap());
    let flow_rate = {
        let s = parts.nth(2).unwrap();
        s[5..s.len() - 1].parse().unwrap()
    };

    let start = parts.skip(4);

    let target_valves = start.map(|x| x.replace(',', "")).collect();

    // println!("{}, {}, {:?}", valve, flow_rate, target_valves);
    (valve, flow_rate, target_valves)
}

fn exercise_1(input: InputType) -> usize {
    let valves: HashMap<String, (usize, Vec<String>)> =
        input.iter().cloned().map(|(a, b, c)| (a, (b, c))).collect();

    // F(valve, time) = max(valve * (time - 1) + max([F(n, time - 2) | n in neighbours of valve]), max([F(n, time - 1) | n in neighbours of valve]) )

    let mut map = HashMap::new();

    step1(
        &valves,
        &mut map,
        &Vec::with_capacity(6),
        String::from("AA"),
        30,
    )
}

fn step1(
    valves: &HashMap<String, (usize, Vec<String>)>,
    visited: &mut HashMap<(String, usize, Vec<String>), usize>,
    openened: &Vec<String>,
    valve: String,
    time: usize,
) -> usize {
    let (flow, neighbours) = valves.get(&valve).unwrap();

    if time == 0 {
        return 0;
    }

    if let Some(val) = visited.get(&(valve.clone(), time, openened.clone())) {
        return *val;
    }

    // Go to neighbours
    let max_neighbours = neighbours
        .iter()
        .map(|x| step1(valves, visited, openened, x.clone(), time - 1))
        .max()
        .unwrap();

    // Stay but stand still
    let standing_still = step1(valves, visited, openened, valve.clone(), time - 1);

    let val = if *flow == 0 || openened.contains(&valve) {
        max_neighbours
    } else {
        let turn_valve = (time - 1) * *flow;

        let mut openened = openened.clone();

        openened.push(valve.clone());
        openened.sort();

        let turn_valve = turn_valve + step1(valves, visited, &openened, valve.clone(), time - 1);

        turn_valve.max(max_neighbours)
    };

    let val = val.max(standing_still);

    visited.insert((valve, time, openened.clone()), val);

    val
}

fn exercise_2(input: InputType) -> usize {
    let valves: HashMap<String, (usize, Vec<String>)> =
        input.iter().cloned().map(|(a, b, c)| (a, (b, c))).collect();

    // F(valve, time) = max(valve * (time - 1) + max([F(n, time - 2) | n in neighbours of valve]), max([F(n, time - 1) | n in neighbours of valve]) )

    let mut map = HashMap::new();

    step2(
        &valves,
        &mut map,
        &Vec::with_capacity(6),
        String::from("AA"),
        String::from("AA"),
        26,
    )
}

fn step2(
    valves: &HashMap<String, (usize, Vec<String>)>,
    visited: &mut HashMap<(String, String, usize, Vec<String>), usize>,
    openened: &Vec<String>,
    valve_a: String,
    valve_b: String,
    time: usize,
) -> usize {
    let (flow_a, neighbours_a) = valves.get(&valve_a).unwrap();
    let (flow_b, neighbours_b) = valves.get(&valve_b).unwrap();

    let (valve_a, valve_b) = { (valve_a.clone().min(valve_b.clone()), valve_a.max(valve_b)) };

    // assert!(valve_a <= valve_b);

    if time == 0 {
        return 0;
    }

    if let Some(val) = visited.get(&(valve_a.clone(), valve_b.clone(), time, openened.clone())) {
        return *val;
    }

    // Go to neighbours or stand still for both
    let max_neighbours = neighbours_a
        .iter()
        .cloned()
        .chain([valve_a.clone()])
        .flat_map(|valve_a| {
            neighbours_b
                .iter()
                .cloned()
                .chain([valve_b.clone()])
                .map(move |valve_b| (valve_a.clone(), valve_b))
        })
        .map(|(valve_me, valve_ele)| {
            step2(valves, visited, openened, valve_ele, valve_me, time - 1)
        })
        .max()
        .unwrap();

    let can_a_turn = *flow_a > 0 && !openened.contains(&valve_a);
    let can_b_turn = *flow_b > 0 && !openened.contains(&valve_b);

    let val = match (can_a_turn, can_b_turn) {
        (true, true) => {
            let turn_a = one_turn(openened, &valve_a, &valve_b, time, valves, visited);
            let turn_b = one_turn(openened, &valve_b, &valve_a, time, valves, visited);

            if valve_a == valve_b {
                turn_a.max(turn_b).max(max_neighbours)
            } else {
                let both = both_turn(
                    openened, &valve_a, &valve_b, *flow_a, *flow_a, time, valves, visited,
                );
                both.max(turn_a).max(turn_b).max(max_neighbours)
            }
        }
        (true, false) => {
            one_turn(openened, &valve_a, &valve_b, time, valves, visited).max(max_neighbours)
        }
        (false, true) => {
            one_turn(openened, &valve_b, &valve_a, time, valves, visited).max(max_neighbours)
        }
        (false, false) => max_neighbours,
    };

    visited.insert(
        (valve_a.clone(), valve_b.clone(), time, openened.clone()),
        val,
    );
    visited.insert((valve_b, valve_a, time, openened.clone()), val);

    val
}

fn both_turn(
    openened: &Vec<String>,
    valve_a: &String,
    valve_b: &String,
    flow_a: usize,
    flow_b: usize,
    time: usize,
    valves: &HashMap<String, (usize, Vec<String>)>,
    visited: &mut HashMap<(String, String, usize, Vec<String>), usize>,
) -> usize {
    let mut openened = openened.clone();
    openened.push(valve_a.clone());
    openened.push(valve_b.clone());
    openened.sort();
    let add_a = flow_a * (time - 1);
    let add_b = flow_b * (time - 1);
    let turn_value = add_a
        + add_b
        + step2(
            valves,
            visited,
            &openened,
            valve_a.clone(),
            valve_b.clone(),
            time - 1,
        );
    turn_value
}

fn one_turn(
    openened: &Vec<String>,
    valve_turn: &String,
    valve_move: &String,
    time: usize,
    valves: &HashMap<String, (usize, Vec<String>)>,
    visited: &mut HashMap<(String, String, usize, Vec<String>), usize>,
) -> usize {
    let mut openened = openened.clone();
    openened.push(valve_turn.clone());

    let turn_flow = valves[valve_turn].0;
    let neighbours = &valves[valve_move].1;

    let added = turn_flow * (time - 1);

    let ele = neighbours
        .iter()
        .cloned()
        .chain([valve_move.clone()])
        .map(|x| step2(valves, visited, &openened, valve_turn.clone(), x, time - 1))
        .max()
        .unwrap();

    added + ele
}

fn ele_turn(
    openened: &Vec<String>,
    valve_me: &String,
    valve_ele: &String,
    flow_ele: &usize,
    time: usize,
    flow_me: &usize,
    valves: &HashMap<String, (usize, Vec<String>)>,
    visited: &mut HashMap<(String, String, usize, Vec<String>), usize>,
) -> usize {
    let mut openened = openened.clone();
    openened.push(valve_ele.clone());

    let ele_add = *flow_ele * (time - 1);

    let me = &valves[valve_me].1;

    let me = me
        .iter()
        .cloned()
        .chain([valve_me.clone()])
        .map(|x| step2(valves, visited, &openened, x, valve_ele.clone(), time - 1))
        .max()
        .unwrap();

    ele_add + me
}
