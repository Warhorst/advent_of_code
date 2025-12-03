use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use regex::Regex;
use crate::aoc_lib::*;
use Operator::*;

pub fn solve_a(input: &str) -> usize {
    let device = Device::from_input(input);
    device.calculate_z()
}

pub fn solve_b(input: &str) -> usize {
    // todo I still have no idea, maybe this will help if I find the energy to try this again: https://www.reddit.com/r/adventofcode/comments/1hla5ql/2024_day_24_part_2_a_guide_on_the_idea_behind_the/

    let device = Device::from_input(input);

    let x = device.get_x();
    let y = device.get_y();

    println!("X: {x}, Y: {y}");
    let sum = x + y;
    println!("Sum: {sum}");
    let current_z = device.calculate_z();
    println!("Z: {current_z}");
    println!("Sum Binary: {sum:#046b}");
    println!("Z   Binary: {current_z:#046b}");

    let all_gates = device
        .z_targets()
        .map(|target| (target, device.get_producing_gates(target)))
        .inspect(|(target, gates)| {
            let grouped = gates.iter().fold(HashMap::<usize, Vec<Gate>>::new(), |mut map, (index, gate)| {
                map.entry(*index).and_modify(|index_gates| index_gates.push(gate.clone())).or_insert(vec![gate.clone()]);
                map
            });

            println!("{target}");
            for i in 0..grouped.len() {
                grouped.get(&i).unwrap().iter().for_each(|gate| print!("{gate}    "));
                println!()
            }

            //println!("{target}: Gates: {:?}, Num Gates: {}", gates, gates.len())
        })
        .fold(HashSet::new(), |mut all_gates, (_, gates)| {
            all_gates.extend(gates);
            all_gates
        });

    println!("All gates: {}", all_gates.len());

    //let mut xs = all_gates.iter().flat_map(|(_, gate)| [gate.a.clone(), gate.b.clone()]).filter(|origin| origin.starts_with("x")).collect::<Vec<_>>();
    //xs.sort();
    //xs.dedup();
    //println!("XS: {:?}", xs);
    //let mut ys = all_gates.iter().flat_map(|(_, gate)| [gate.a.clone(), gate.b.clone()]).filter(|origin| origin.starts_with("y")).collect::<Vec<_>>();
    //ys.sort();
    //ys.dedup();
    //println!("YS: {:?}", ys);

    //target_expressions_map
    //    .keys()
    //    .filter(|target| target.starts_with("z"))
    //    .map(|target| (target, get_sub_gates(target, &target_expressions_map)))
    //    .for_each(|(target, sub_gates)| println!("{target}: {:?}", sub_gates));

    //let sub_gates = target_expressions_map
    //    .keys()
    //    .filter(|target| target.starts_with("z"))
    //    .flat_map(|target| get_sub_gates(target, &target_expressions_map))
    //    .collect::<HashSet<_>>();

    0
}

struct Device {
    initial_values: HashMap<String, bool>,
    target_gate_map: HashMap<String, Gate>,
}

impl Device {
    fn from_input(input: &str) -> Self {
        let mut split = input.split("\n\n");

        let initial_values_regex = Regex::new(r"(?m)^(.*):\s([01])$").unwrap();

        let initial_values = regex_captures(
            split.next().unwrap(),
            &initial_values_regex,
            |caps| (caps[0].to_string(), parse::<usize>(caps[1]) == 1),
        )
            .into_iter()
            .collect::<HashMap<_, _>>();

        let expressions_regex = Regex::new(r"(?m)^(.{3})\s(.*)\s(.{3})\s->\s(.{3})$").unwrap();

        let target_gate_map = regex_captures(
            split.next().unwrap(),
            &expressions_regex,
            |caps| {
                let a = caps[0].to_string();
                let b = caps[2].to_string();

                let op = match caps[1] {
                    "AND" => And,
                    "OR" => Or,
                    "XOR" => Xor,
                    _ => unreachable!()
                };
                let target = caps[3].to_string();

                (target, Gate {
                    a,
                    op,
                    b,
                })
            },
        ).into_iter().collect::<HashMap<_, _>>();

        Device {
            initial_values,
            target_gate_map,
        }
    }

    fn get_value(&self, name: &String) -> bool {
        match self.initial_values.get(name) {
            Some(value) => *value,
            None => {
                let gate = self.target_gate_map.get(name).unwrap();
                let a_value = self.get_value(&gate.a);
                let b_value = self.get_value(&gate.b);

                match gate.op {
                    And => a_value && b_value,
                    Or => a_value || b_value,
                    Xor => (a_value && !b_value) || (!a_value && b_value)
                }
            }
        }
    }

    fn get_x(&self) -> usize {
        Self::bits_to_num(
            self.initial_values
                .iter()
                .filter(|(name, _)| name.starts_with("x"))
                .map(|(name, value)| (name, *value))
        )
    }

    fn get_y(&self) -> usize {
        Self::bits_to_num(
            self.initial_values
                .iter()
                .filter(|(name, _)| name.starts_with("y"))
                .map(|(name, value)| (name, *value))
        )
    }

    fn calculate_z(&self) -> usize {
        Self::bits_to_num(
            self.z_targets().map(|target| (target, self.get_value(target)))
        )
    }

    fn bits_to_num<'a>(names_and_bits: impl IntoIterator<Item=(&'a String, bool)>) -> usize {
        let mut names_and_bits = names_and_bits.into_iter().collect::<Vec<_>>();

        names_and_bits.sort_by(|(a, _), (b, _)| b.cmp(a));

        names_and_bits.into_iter().fold(0, |mut acc, (_, value)| {
            acc <<= 1;
            match value {
                true => acc |= 1,
                false => acc |= 0
            };
            acc
        })
    }

    fn z_targets(&self) -> impl Iterator<Item=&String> {
        let mut zs = self.target_gate_map.keys().filter(|target| target.starts_with("z")).collect::<Vec<_>>();
        zs.sort();
        zs.into_iter()
    }

    fn get_producing_gates(&self, target: &String) -> Vec<(usize, Gate)> {
        let mut gates = Vec::new();

        self.add_producing_gates(target, 0, &mut gates);

        gates
    }

    fn add_producing_gates(&self, target: &String, depth: usize, gates: &mut Vec<(usize, Gate)>) {
        if let Some(gate) = self.get_gate(target) {
            gates.push((depth, gate.clone()));

            self.add_producing_gates(&gate.a, depth + 1, gates);
            self.add_producing_gates(&gate.b, depth + 1, gates);
        }
    }

    fn get_gate(&self, target: &String) -> Option<&Gate> {
        self.target_gate_map.get(target)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Gate {
    a: String,
    b: String,
    op: Operator,
}

impl std::fmt::Display for Gate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?} {}", self.a, self.op, self.b)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Operator {
    And,
    Or,
    Xor,
}
