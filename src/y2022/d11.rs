use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul, Rem},
};

use crate::aoc_lib::parse;

pub fn solve_a(input: &str) -> usize {
    let mut monkeys = input
        .split("\n\n")
        .map(|s| Monkey::from(s))
        .collect::<Vec<_>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            let changes = monkey.turn();

            for change in changes {
                monkeys.get_mut(change.0).unwrap().throw_to(change.1);
            }
        }
    }

    monkeys.sort_by(|m0, m1| m1.inspections.cmp(&m0.inspections));

    monkeys[0].inspections * monkeys[1].inspections
}

// https://en.wikipedia.org/wiki/Modular_arithmetic
pub fn solve_b(input: &str) -> u128 {
    let monkeys = input
        .split("\n\n")
        .map(|s| Monkey::from(s))
        .collect::<Vec<_>>();

    let divisors = monkeys.iter().map(|m| m.test_div).collect::<HashSet<_>>();

    let mut stress_monkeys = monkeys.into_iter().map(|m| StressMonkey::new(m, divisors.iter())).collect::<Vec<_>>();

    for _ in 0..10000 {
        for i in 0..stress_monkeys.len() {
            let monkey = stress_monkeys.get_mut(i).unwrap();
            let changes = monkey.turn();

            for change in changes {
                stress_monkeys.get_mut(change.0).unwrap().throw_to(change.1);
            }
        }
    }

    stress_monkeys.sort_by(|m0, m1| m1.inspections.cmp(&m0.inspections));

    stress_monkeys[0].inspections as u128 * stress_monkeys[1].inspections as u128
}

/// Reject humanity. Return to
#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test_div: u128,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

impl Monkey {
    fn turn(&mut self) -> Vec<(usize, u128)> {
        self.inspections += self.items.len();
        self.items
            .drain(..)
            .map(|item| self.operation.calc(item)) // inspect
            .map(|item| item / 3) // relief
            .map(|item| {
                // test
                if item % self.test_div == 0 {
                    (self.if_true, item)
                } else {
                    (self.if_false, item)
                }
            })
            .collect()
    }

    fn throw_to(
        &mut self,
        item: u128,
    ) {
        self.items.push(item);
    }
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let split = value.split("\n").collect::<Vec<_>>();

        let items = split[1]
            .split("Starting items: ")
            .skip(1)
            .next()
            .unwrap()
            .split(", ")
            .map(|s| parse::<u128>(s))
            .collect();

        let operation = Operation::from(
            split[2]
                .split("Operation: new = old ")
                .skip(1)
                .next()
                .unwrap(),
        );

        let test_div = parse::<u128>(
            split[3]
                .split("Test: divisible by ")
                .skip(1)
                .next()
                .unwrap(),
        );
        let if_true = parse::<usize>(
            split[4]
                .split("If true: throw to monkey ")
                .skip(1)
                .next()
                .unwrap(),
        );
        let if_false = parse::<usize>(
            split[5]
                .split("If false: throw to monkey ")
                .skip(1)
                .next()
                .unwrap(),
        );

        Monkey {
            items,
            operation,
            test_div,
            if_true,
            if_false,
            inspections: 0,
        }
    }
}

/// Monkey, but a lot more stressful
#[derive(Debug)]
pub struct StressMonkey {
    items: Vec<SuperModulo>,
    operation: Operation,
    test_div: u128,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

impl StressMonkey {
    fn new<'a>(
        monkey: Monkey,
        all_divisors: impl Iterator<Item = &'a u128> + 'a,
    ) -> Self {
        let all_divisors = all_divisors.copied().collect::<Vec<_>>();
        StressMonkey {
            items: monkey
                .items
                .into_iter()
                .map(|item| SuperModulo::new(item, &all_divisors))
                .collect(),
            operation: monkey.operation,
            test_div: monkey.test_div,
            if_true: monkey.if_true,
            if_false: monkey.if_false,
            inspections: monkey.inspections,
        }
    }

    fn turn(&mut self) -> Vec<(usize, SuperModulo)> {
        self.inspections += self.items.len();
        self.items
            .drain(..)
            .map(|item| self.operation.calc_stressful(item)) // inspect
            .map(|item| {
                // test
                if &item % self.test_div == 0 {
                    (self.if_true, item)
                } else {
                    (self.if_false, item)
                }
            })
            .collect()
    }

    fn throw_to(
        &mut self,
        item: SuperModulo,
    ) {
        self.items.push(item);
    }
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(Value),
    Mul(Value),
}

impl Operation {
    fn calc(
        &self,
        old: u128,
    ) -> u128 {
        match self {
            Operation::Add(value) => old + value.get(old),
            Operation::Mul(value) => old * value.get(old),
        }
    }

    fn calc_stressful(
        &self,
        old: SuperModulo,
    ) -> SuperModulo {
        match self {
            Operation::Add(value) => old.clone() + value.get_stressful(old),
            Operation::Mul(value) => old.clone() * value.get_stressful(old),
        }
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        // Expects something like "* old" or "+ 3"

        let split = value.split(" ").collect::<Vec<_>>();
        let value = if split[1] == "old" {
            Value::Old
        } else {
            Value::Val(parse(split[1]))
        };

        if split[0] == "+" {
            Operation::Add(value)
        } else {
            Operation::Mul(value)
        }
    }
}

impl Rem<u128> for &SuperModulo {
    type Output = u128;

    fn rem(
        self,
        rhs: u128,
    ) -> Self::Output {
        match self.mods.get(&rhs) {
            Some(modulo) => modulo.remainder,
            None => panic!("Invalid divisor"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Value {
    Old,
    Val(u128),
}

impl Value {
    fn get(
        &self,
        old: u128,
    ) -> u128 {
        match self {
            Value::Old => old,
            Value::Val(v) => *v,
        }
    }

    fn get_stressful(
        &self,
        old: SuperModulo,
    ) -> SuperModulo {
        match self {
            Value::Old => old,
            Value::Val(v) => SuperModulo::new(*v, old.get_all_divisors()),
        }
    }
}

#[derive(Clone, Debug)]
struct SuperModulo {
    mods: HashMap<u128, Modulo>,
}

impl SuperModulo {
    fn new<'a>(
        value: u128,
        all_divisors: impl IntoIterator<Item = &'a u128> + 'a,
    ) -> Self {
        SuperModulo {
            mods: all_divisors
                .into_iter()
                .map(|div| (*div, Modulo::new(value, *div)))
                .collect(),
        }
    }

    fn get_all_divisors(&self) -> impl Iterator<Item = &u128> {
        self.mods.keys()
    }
}

impl Add for SuperModulo {
    type Output = SuperModulo;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        SuperModulo {
            mods: self
                .mods
                .iter()
                .map(|(div, modulo)| (*div, *modulo + *rhs.mods.get(div).unwrap()))
                .collect(),
        }
    }
}

impl Mul for SuperModulo {
    type Output = SuperModulo;

    fn mul(
        self,
        rhs: Self,
    ) -> Self::Output {
        SuperModulo {
            mods: self
                .mods
                .iter()
                .map(|(div, modulo)| (*div, *modulo * *rhs.mods.get(div).unwrap()))
                .collect(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Modulo {
    pub remainder: u128,
    pub m: u128,
}

impl Modulo {
    fn new(
        value: u128,
        m: u128,
    ) -> Self {
        Modulo {
            remainder: value % m,
            m,
        }
    }
}

impl Add for Modulo {
    type Output = Modulo;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        if self.m != rhs.m {
            panic!("Addition is only possible if both numbers have the same divident!")
        }

        Modulo::new(self.remainder + rhs.remainder, self.m)
    }
}

impl Mul for Modulo {
    type Output = Modulo;

    fn mul(
        self,
        rhs: Self,
    ) -> Self::Output {
        if self.m != rhs.m {
            panic!("Multiplication is only possible if both numbers have the same divident!")
        }

        Modulo::new(self.remainder * rhs.remainder, self.m)
    }
}
