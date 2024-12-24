use std::collections::{HashMap, HashSet};
use regex::Regex;
use crate::aoc_lib::*;
use Operator::*;

pub fn solve_a(input: &str) -> usize {
    let device = Device::from_input(input);
    device.calculate_z()
}

pub fn solve_b(input: &str) -> usize {
    let device = Device::from_input(input);

    let x = device.get_x();
    let y = device.get_y();

    println!("X: {x}, Y: {y}");
    let sum = x + y;
    println!("Sum: {sum}");
    let current_z = device.calculate_z();
    println!("Z: {current_z}");
    println!("Sum Binary: {:#045b}", sum);
    println!("Z   Binary: {:#045b}", current_z);

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

#[allow(dead_code)]
fn get_sub_gates(
    name: &String,
    target_expressions_map: &HashMap<String, (String, Operator, String)>
) -> HashSet<String> {
    let mut sub_gates = HashSet::new();
    add_sub_gates(name, target_expressions_map, &mut sub_gates);
    sub_gates
}

#[allow(dead_code)]
fn add_sub_gates(
    name: &String,
    target_expressions_map: &HashMap<String, (String, Operator, String)>,
    sub_gates: &mut HashSet<String>
) {
    sub_gates.insert(name.clone());

    match target_expressions_map.get(name) {
        Some((a, _, b)) => {
            add_sub_gates(a, target_expressions_map, sub_gates);
            add_sub_gates(b, target_expressions_map, sub_gates);
        }
        None => {}
    }
}

struct Device {
    initial_values: HashMap<String, bool>,
    target_expressions_map: HashMap<String, (String, Operator, String)>
}

impl Device {
    fn from_input(input: &str) -> Self {
        let mut split = input.split("\n\n");

        let initial_values_regex = Regex::new(r"(?m)^(.*):\s([01])$").unwrap();

        let initial_values = regex_captures(
            split.next().unwrap(),
            &initial_values_regex,
            |caps| (caps[0].to_string(), if parse::<usize>(caps[1]) == 1 { true } else { false }),
        )
            .into_iter()
            .collect::<HashMap<_, _>>();

        let expressions_regex = Regex::new(r"(?m)^(.{3})\s(.*)\s(.{3})\s->\s(.{3})$").unwrap();

        let target_expressions_map = regex_captures(
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

                (target, (a, op, b))
            },
        ).into_iter().collect::<HashMap<_, _>>();

        Device {
            initial_values,
            target_expressions_map
        }
    }

    fn get_value(&self, name: &String) -> bool {
        match self.initial_values.get(name) {
            Some(value) => *value,
            None => {
                let (a, op, b) = self.target_expressions_map.get(name).unwrap();
                let a_value = self.get_value(a);
                let b_value = self.get_value(b);

                match op {
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
            self.target_expressions_map
            .keys()
            .filter(|target| target.starts_with("z"))
            .map(|target| (target, self.get_value(target)))
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
}

#[derive(Debug)]
enum Operator {
    And,
    Or,
    Xor,
}