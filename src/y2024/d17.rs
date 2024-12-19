use crate::aoc_lib::*;
use itertools::Itertools;
use Ins::*;

pub fn solve_a(input: &str) -> String {
    Device::new(input)
        .run()
        .into_iter()
        .map(|val| val.to_string())
        .join(",")
}

pub fn solve_b(input: &str) -> usize {
    Device::new(input).find_a_producing_input()
}

#[derive(Clone, Debug)]
struct Device {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    original_instructions: Vec<usize>,
    instructions: Vec<Ins>,
    ins_pointer: usize,
}

impl Device {
    fn new(input: &str) -> Self {
        let mut split = input.split("\n\n");
        let mut register_lines = split.next().unwrap().lines();
        let reg_a = parse(register_lines.next().unwrap().split(" ").last().unwrap());
        let reg_b = parse(register_lines.next().unwrap().split(" ").last().unwrap());
        let reg_c = parse(register_lines.next().unwrap().split(" ").last().unwrap());

        let original_instructions = split
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .split(",")
            .map(|s| parse::<usize>(s))
            .collect::<Vec<_>>();
        let instructions = original_instructions
            .windows(2)
            .enumerate()
            .filter_map(|(i, vals)| if i % 2 == 0 {
                Some(Ins::new(vals[0], vals[1]))
            } else {
                None
            })
            .collect();

        Device {
            reg_a,
            reg_b,
            reg_c,
            original_instructions,
            instructions,
            ins_pointer: 0,
        }
    }

    fn run(&mut self) -> Vec<usize> {
        let mut output: Vec<usize> = vec![];

        while let Some(ins) = self.instructions.get(self.ins_pointer) {
            match ins {
                Adv(op) => self.reg_a = self.reg_a / 2_usize.pow(self.combo_value(*op) as u32),
                Bxl(op) => self.reg_b = self.reg_b ^ op,
                Bst(op) => self.reg_b = self.combo_value(*op) % 8,
                Jnz(op) => if self.reg_a != 0 {
                    self.ins_pointer = *op / 2
                }
                Bxc(_) => self.reg_b = self.reg_b ^ self.reg_c,
                Out(op) => output.push(self.combo_value(*op) % 8),
                Bdv(op) => self.reg_b = self.reg_a / 2_usize.pow(self.combo_value(*op) as u32),
                Cdv(op) => self.reg_c = self.reg_a / 2_usize.pow(self.combo_value(*op) as u32)
            };

            if let Jnz(_) = ins {
                if self.reg_a == 0 {
                    self.ins_pointer += 1
                }
            } else {
                self.ins_pointer += 1
            }
        }

        output
    }

    fn find_a_producing_input(&self) -> usize {
        let mut selection = vec![];
        self.find_a_producing_input_inner(self.original_instructions.len(), &mut selection).unwrap()
    }

    fn find_a_producing_input_inner(&self, index: usize, selection: &mut Vec<usize>) -> Option<usize> {
        let get_a = |selection: &Vec<usize>| selection.iter().fold(0, |mut acc, item| {
            acc <<= 3;
            acc |= item;
            acc
        });

        if index == 0 {
            return Some(get_a(selection));
        }

        for i in 0..=7 {
            selection.push(i);

            let a = get_a(selection);

            let result = self.run_with_a(a);
            let expected = &self.original_instructions[(index - 1)..];

            if &result == expected {
                match self.find_a_producing_input_inner(index - 1, selection) {
                    Some(a) => return Some(a),
                    None => {
                        selection.pop();
                    }
                }
            } else {
                selection.pop();
            }
        }

        None
    }

    fn run_with_a(&self, a: usize) -> Vec<usize> {
        let mut clone = self.clone();
        clone.reg_a = a;
        clone.run()
    }

    fn combo_value(&self, val: usize) -> usize {
        match val {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Ins {
    Adv(usize),
    Bxl(usize),
    Bst(usize),
    Jnz(usize),
    Bxc(usize),
    Out(usize),
    Bdv(usize),
    Cdv(usize),
}

impl Ins {
    fn new(op_code: usize, operand: usize) -> Self {
        match op_code {
            0 => Adv(operand),
            1 => Bxl(operand),
            2 => Bst(operand),
            3 => Jnz(operand),
            4 => Bxc(operand),
            5 => Out(operand),
            6 => Bdv(operand),
            7 => Cdv(operand),
            _ => unreachable!()
        }
    }
}