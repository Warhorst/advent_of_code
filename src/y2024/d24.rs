use std::collections::HashMap;
use regex::Regex;
use crate::aoc_lib::*;
use Operator::*;

pub fn solve_a(input: &str) -> usize {
    let mut split = input.split("\n\n");

    let initial_values_regex = Regex::new(r"(?m)^(.*):\s([01])$").unwrap();

    let mut name_value_map = regex_captures(
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
        }
    ).into_iter().collect::<HashMap<_, _>>();

    let mut values = target_expressions_map
        .keys()
        .filter(|target| target.starts_with("z"))
        .map(|target| (target, get_value(target, &target_expressions_map, &mut name_value_map)))
        .collect::<Vec<_>>();

    values.sort_by(|(a, _), (b, _)| b.cmp(a));

    values.into_iter().fold(0, |mut acc, (_, value)| {
        acc <<= 1;
        match value {
            true => acc |= 1,
            false => acc |= 0
        };
        acc
    })
}

fn get_value(
    name: &String,
    target_expressions_map: &HashMap<String, (String, Operator, String)>,
    name_value_map: &mut HashMap<String, bool>
) -> bool {
    if let Some(value) = name_value_map.get(name) {
        return *value;
    }

    let (a, op, b) = target_expressions_map.get(name).unwrap();

    let a_value = get_value(a, target_expressions_map, name_value_map);
    let b_value = get_value(b, target_expressions_map, name_value_map);

    match op {
        And => a_value && b_value,
        Or => a_value || b_value,
        Xor => (a_value && !b_value) || (!a_value && b_value)
    }
}

pub fn solve_b(_input: &str) -> usize {
    0
}

#[derive(Debug)]
enum Operator {
    And,
    Or,
    Xor,
}