use crate::aoc_lib::*;

use regex::Regex;

pub fn solve_a(input: &str) -> String {
    let mut blocks = input.split("\n\n");
    let mut stacks = Stacks::from(blocks.next().unwrap());
    let commands = Commands::from(blocks.next().unwrap());

    for command in commands.0 {
        stacks.apply(command);
    }

    stacks.tops()
}

pub fn solve_b(input: &str) -> String {
    let mut blocks = input.split("\n\n");
    let mut stacks = Stacks::from(blocks.next().unwrap());
    let commands = Commands::from(blocks.next().unwrap());

    for command in commands.0 {
        stacks.apply_9001(command);
    }

    stacks.tops()
}

struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn new(num: usize) -> Self {
        Stacks(vec![Vec::new(); num])
    }

    fn add(&mut self, index: usize, c: char) {
        self.0[index].push(c)
    }

    fn apply(&mut self, command: Command) {
        for _ in 0..command.amount {
            let removed = self.0[command.from - 1].pop().unwrap();
            self.0[command.to - 1].push(removed)
        }
    }

    fn apply_9001(&mut self, command: Command) {
        let removed = (0..command.amount)
            .into_iter()
            .map(|_| self.0[command.from - 1].pop().unwrap())
            .collect::<Vec<_>>();

        removed
            .into_iter()
            .rev()
            .for_each(|char| self.0[command.to - 1].push(char));
    }

    fn tops(&self) -> String {
        self.0
            .iter()
            .map(|stack| match stack.last() {
                Some(char) => *char,
                None => ' '
            })
            .collect::<String>().replace(" ", "")
    }
}

impl From<&str> for Stacks {
    fn from(input: &str) -> Self {
        let num_stacks = parse_char(input
            .lines()
            .last()
            .unwrap()
            .replace(" ", "")
            .chars()
            .last()
            .unwrap()) as usize;
        let mut stacks = Stacks::new(num_stacks);

        input
            .lines()
            .rev()
            .skip(1)
            .flat_map(|line| {
                let chars = line.chars().collect::<Vec<_>>();
                chars
                    .windows(3)
                    .step_by(4)
                    .map(|block| block[1])
                    .enumerate()
                    .filter(|(_, char)| *char != ' ')
                    .collect::<Vec<_>>()
            })
            .for_each(|(index, char)| stacks.add(index, char));

        stacks
    }
}

struct Commands(Vec<Command>);

struct Command {
    amount: usize,
    from: usize,
    to: usize
}

impl From<&str> for Commands {
    fn from(value: &str) -> Self {
        let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

        Commands(regex_captures(
            value,
            &regex,
            |caps| Command {
                amount: parse(caps[0]),
                from: parse(caps[1]),
                to: parse(caps[2]),
            }
        ).into_iter().collect())
    }
}