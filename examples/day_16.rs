use num_traits::Pow;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    ops::Not,
};

type InputType = Vec<(String, usize, Vec<String>)>;
type Valve = u8;

type ValveOpened = u64;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    include_str!("../input/day_16.txt")
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

    let translator = valves
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, a)| (a.clone().0, i as Valve))
        .collect::<HashMap<_, _>>();

    let start = translator[&"AA".to_string()];

    let valves = valves
        .into_iter()
        .map(|(v, (flow, ne))| {
            (
                translator[&v],
                (
                    flow,
                    ne.into_iter().map(|n| translator[&n]).collect::<Vec<_>>(),
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    // F(valve, time) = max(valve * (time - 1) + max([F(n, time - 2) | n in neighbours of valve]), max([F(n, time - 1) | n in neighbours of valve]) )

    let mut map = HashMap::new();

    step1(&valves, &mut map, 0, start, 30)
}

fn step1(
    valves: &HashMap<Valve, (usize, Vec<Valve>)>,
    visited: &mut HashMap<(Valve, usize, ValveOpened), usize>,
    openened: ValveOpened,
    valve: Valve,
    time: usize,
) -> usize {
    let (flow, neighbours) = valves.get(&valve).unwrap();

    if time == 0 {
        return 0;
    }

    if let Some(val) = visited.get(&(valve, time, openened)) {
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

    let val = if *flow == 0 || (openened & (1 << valve)) > 0 {
        max_neighbours
    } else {
        let turn_valve = (time - 1) * *flow;
        let openened = openened | (1 << valve);

        let turn_valve = turn_valve + step1(valves, visited, openened, valve.clone(), time - 1);

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

    let translator = valves
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, a)| (a.clone().0, i as Valve))
        .collect::<HashMap<_, _>>();

    let start = translator[&"AA".to_string()];

    let valves = valves
        .into_iter()
        .map(|(v, (flow, ne))| {
            (
                translator[&v],
                (
                    flow,
                    ne.into_iter().map(|n| translator[&n]).collect::<Vec<_>>(),
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    let combos = 2u64.pow(valves.len() as u32);

    let mut visited = Default::default();

    let mask = (2u64.pow(valves.len() as u32)) - 1;

    (0..=combos)
        .filter(|&a| a <= (a.not() & mask))
        .map(|x| {
            let step_a = step1(&valves, &mut visited, x, start, 26);
            let step_b = step1(&valves, &mut visited, !x & mask, start, 26);

            step_a + step_b
        })
        .max()
        .unwrap()
}
// fn step2(
//     valves: &HashMap<Valve, (usize, Vec<Valve>)>,
//     visited: &mut HashMap<(Valve, Valve, usize, u64), usize>,
//     openened: u64,
//     valve_a: Valve,
//     valve_b: Valve,
//     time: usize,
// ) -> usize {
//     let (valve_a, valve_b) = { (valve_a.min(valve_b), valve_a.max(valve_b)) };

//     if time == 0 {
//         return 0;
//     }

//     if let Some(val) = visited.get(&(valve_a, valve_b, time, openened.clone())) {
//         return *val;
//     }

//     let (can_a_turn, neighbours_a) = {
//         let x = valves.get(&valve_a).unwrap();

//         (x.0 > 0 && (openened & (1 << valve_a)) == 0, x.1.iter())
//     };
//     let (can_b_turn, neighbours_b) = {
//         let x = valves.get(&valve_b).unwrap();
//         (x.0 > 0 && (openened & (1 << valve_b)) == 0, x.1.clone())
//     };

//     // Go to neighbours or stand still for both
//     let max_neighbours = neighbours_a
//         .cloned()
//         .chain([valve_a.clone()])
//         .flat_map(|valve_a| {
//             neighbours_b
//                 .iter()
//                 .cloned()
//                 .chain([valve_b.clone()])
//                 .map(move |valve_b| (valve_a.clone(), valve_b))
//         })
//         .map(|(a, b)| (a.min(b), a.max(b)))
//         .collect::<HashSet<_>>();

//     let max_neighbours = max_neighbours
//         .into_iter()
//         .map(|(valve_a, valve_b)| step2(valves, visited, openened, valve_a, valve_b, time - 1))
//         .max()
//         .unwrap();

//     let val = match (can_a_turn, can_b_turn) {
//         (true, true) => {
//             let turned_a = turn_a(openened, valve_a, valve_b, time, valves, visited);
//             let turned_b = turn_a(openened, valve_b, valve_a, time, valves, visited);

//             if valve_a == valve_b {
//                 turned_a.max(turned_b).max(max_neighbours)
//             } else {
//                 let both = both_turn(openened, valve_a, valve_b, time, valves, visited);
//                 both.max(turned_a).max(turned_b).max(max_neighbours)
//             }
//         }
//         (true, false) => {
//             turn_a(openened, valve_a, valve_b, time, valves, visited).max(max_neighbours)
//         }
//         (false, true) => {
//             turn_a(openened, valve_b, valve_a, time, valves, visited).max(max_neighbours)
//         }
//         (false, false) => max_neighbours,
//     };

//     visited.insert((valve_a, valve_b, time, openened.clone()), val);

//     val
// }

// fn both_turn(
//     openened: ValveOpened,
//     valve_a: Valve,
//     valve_b: Valve,
//     time: usize,
//     valves: &HashMap<Valve, (usize, Vec<Valve>)>,
//     visited: &mut HashMap<(Valve, Valve, usize, ValveOpened), usize>,
// ) -> usize {
//     let openened = openened | (1 << valve_a) | (1 << valve_b);

//     let flow_a = valves[&valve_a].0;
//     let flow_b = valves[&valve_b].0;

//     let add_a = flow_a * (time - 1);
//     let add_b = flow_b * (time - 1);
//     let turn_value = add_a
//         + add_b
//         + step2(
//             valves,
//             visited,
//             openened,
//             valve_a.clone(),
//             valve_b.clone(),
//             time - 1,
//         );
//     turn_value
// }

// fn turn_a(
//     openened: ValveOpened,
//     valve_a: Valve,
//     valve_b: Valve,
//     time: usize,
//     valves: &HashMap<Valve, (usize, Vec<Valve>)>,
//     visited: &mut HashMap<(Valve, Valve, usize, ValveOpened), usize>,
// ) -> usize {
//     let openened = openened | (1 << valve_a);

//     let turn_flow = valves[&valve_a].0;
//     let added = turn_flow * (time - 1);

//     let ele = valves[&valve_b]
//         .1
//         .iter()
//         .cloned()
//         .chain([valve_b])
//         .map(|x| step2(valves, visited, openened, valve_a.clone(), x, time - 1))
//         .max()
//         .unwrap();

//     added + ele
// }

// fn turn_b(
//     openened: &Vec<Valve>,
//     valve_a: Valve,
//     valve_b: Valve,
//     time: usize,
//     valves: &HashMap<Valve, (usize, Vec<Valve>)>,
//     visited: &mut HashMap<(Valve, Valve, usize, Vec<Valve>), usize>,
// ) -> usize {
//     let mut openened = openened.clone();

//     openened.push(valve_b.clone());
//     openened.sort();

//     let turn_flow = valves[&valve_b].0;
//     let added = turn_flow * (time - 1);

//     let me = &valves[&valve_a].1;

//     let me = me
//         .iter()
//         .cloned()
//         .chain([valve_a.clone()])
//         .map(|x| step2(valves, visited, &openened, x, valve_b.clone(), time - 1))
//         .max()
//         .unwrap();

//     added + me
// }
