use std::ops::{Add, Mul};

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

pub fn solve_b(input: &str) -> u128 {
    let mut monkeys = input
        .split("\n\n")
        .map(|s| Monkey::from(s))
        .collect::<Vec<_>>();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            let changes = monkey.stress_turn();

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

    fn stress_turn(&mut self) -> Vec<(usize, u128)> {
        // todo I might need to keep track of all possible remainders at once
        
        self.inspections += self.items.len();
        self.items
            .drain(..)
            .map(|item| Modulo::new(item, self.test_div))
            .map(|item| self.operation.calc_stress(item)) // inspect
            .map(|item| {
                // test
                if item.remainder == 0 {
                    (self.if_true, item.remainder)
                } else {
                    (self.if_false, item.remainder)
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

    fn calc_stress(
        &self,
        old: Modulo,
    ) -> Modulo {
        match self {
            Operation::Add(value) => old + Modulo::new(value.get(old.remainder), old.m),
            Operation::Mul(value) => old * Modulo::new(value.get(old.remainder), old.m),
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
