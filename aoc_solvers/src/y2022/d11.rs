use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul, Rem},
};

use helpers::prelude::*;

pub fn solve_a(input: &str) -> usize {
    let mut monkeys = input
        .split("\n\n")
        .map(Monkey::from)
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

/// How does this work?
///
/// The issue in this puzzle: As the stress is no longer divided by 3 after
/// each turn, and the number of rounds is much higher, the stress per item is way higher.
/// So high in fact that it is not possible to express the stress with a u128.
///
/// However, the exact stress is irrelevant in this case. The only important value
/// here is the number of inspections, which is indirectly determined by the check
/// which monkey to give the item next. To perform this check, we only need to know
/// if the result of the monkeys operation is dividable by its test value.
///
/// To solve this, we can use [Modular Arithmetics](https://en.wikipedia.org/wiki/Modular_arithmetic).
/// The only operations performed by the monkeys are addition and multiplication. These have the following
/// properties:
/// - (a + b) mod n = (a mod n + b mod n) mod n 
/// - (a * b) mod n = (a mod n) * (b mod n) mod n
///
/// This means: Instead of calculating the actual numbers, we can use their remainders (with the test value of the monkey as the modulus).
/// This keeps the stress value as low as the highest monkey test value.
///
/// However, there is a problem here: Every monkey has a different test value and therefore a different modulus. So
/// we cannot just give the resulting remainder to the next monkey and it continues with that. Instead,
/// the items in this puzzle are [SuperModulo]s, a term I just made up. This contains all the current remainders
/// for all possible test values the monkeys might have. So an operation of one monkey is treated as if all monkeys
/// perform it with their respective test values. Using this, each monkey can perform their operation and
/// afterwards check for their remainder in the [SuperModulo].
pub fn solve_b(input: &str) -> u128 {
    let monkeys = input
        .split("\n\n")
        .map(Monkey::from)
        .collect::<Vec<_>>();

    let divisors = monkeys.iter().map(|m| m.test_div).collect::<HashSet<_>>();

    let mut monkeys = monkeys
        .into_iter()
        .map(|m| StressMonkey::new(m, divisors.iter()))
        .collect::<Vec<_>>();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            let changes = monkey.turn();

            for change in changes {
                monkeys.get_mut(change.0).unwrap().throw_to(change.1);
            }
        }
    }

    monkeys.sort_by(|m0, m1| m1.inspections.cmp(&m0.inspections));

    monkeys[0].inspections as u128 * monkeys[1].inspections as u128
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
            .nth(1)
            .unwrap()
            .split(", ")
            .map(parse::<u128>)
            .collect();

        let operation = Operation::from(
            split[2]
                .split("Operation: new = old ")
                .nth(1)
                .unwrap(),
        );

        let test_div = parse::<u128>(
            split[3]
                .split("Test: divisible by ")
                .nth(1)
                .unwrap(),
        );
        let if_true = parse::<usize>(
            split[4]
                .split("If true: throw to monkey ")
                .nth(1)
                .unwrap(),
        );
        let if_false = parse::<usize>(
            split[5]
                .split("If false: throw to monkey ")
                .nth(1)
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
