use std::iter::Rev;
use crate::aoc_lib::*;
use itertools::Itertools;
use Ins::*;
use rayon::prelude::*;

pub fn solve_a(input: &str) -> String {
    let mut device = Device::new(input);

    //println!("{:?}", device);

    device.run()
}

pub fn solve_b(input: &str) -> usize {
    let device = Device::new(input);
    let searched = device.original_instructions.iter().map(|i| i.to_string()).join(",");

    0

    //(1..8)
    //    .into_iter()
    //    .map(|i| {
    //        let mut clone = device.clone();
    //        let a =  i * 8usize.pow(device.original_instructions.len() as u32 - 1);
    //        clone.reg_a = a;
    //        let result = clone.run();
    //        (a, result)
    //    })
    //    .inspect(|(a, result)| println!("{a} - {result}"))
    //    .find_map(|(a, result)| {
    //        if result == device.original_instructions.iter().map(|i| i.to_string()).join(",") {
    //            Some(a)
    //        } else {
    //            None
    //        }
    //    })
    //    .unwrap()

    //// bad brute force method
    //let min = 8usize.pow(device.original_instructions.len() as u32 - 1);
    //let max = 7 * min;
//
    //let foo = (min..max)
    //    .par_bridge()
    //    .map(|i| {
    //        let mut clone = device.clone();
    //        clone.reg_a = i;
    //        let res = clone.run();
    //        (i, res == searched)
    //    })
    //    .filter(|(_, success)| *success)
    //    .collect::<Vec<_>>();
    //foo.into_iter().min_by(|(a_0, _), (a_1, _)| a_0.cmp(a_1)).unwrap().0
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

    fn run(&mut self) -> String {
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

        output.into_iter().map(|val| val.to_string()).join(",")
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