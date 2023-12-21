use std::collections::HashMap;
use std::ops::Index;
use crate::y2023::d19::Check::*;
use crate::y2023::d19::FollowUp::*;
use crate::y2023::d19::Instruction::*;
use crate::y2023::d19::Value::*;

pub fn solve_19a(input: &str) -> usize {
    let split = input.split("\r\n\r\n").collect::<Vec<_>>();

    let key_instructions_map = split[0]
        .lines()
        .map(Instructions::from)
        .map(|ins| (ins.key.clone(), ins))
        .collect::<HashMap<_, _>>();

    split[1]
        .lines()
        .map(Part::from)
        .map(|part| get_value_for_part(part, &key_instructions_map))
        .sum()
}

pub fn solve_19b(_input: &str) -> u128 {
    // let key_instructions_map = input
    //     .split("\r\n\r\n")
    //     .next()
    //     .unwrap()
    //     .lines()
    //     .map(Instructions::from)
    //     .map(|ins| (ins.key.clone(), ins))
    //     .collect::<HashMap<_, _>>();



    0
}

fn get_value_for_part(part: Part, key_instructions_map: &HashMap<String, Instructions>) -> usize {
    let mut current_key = "in".to_string();

    loop {
        let follow_up = key_instructions_map.get(&current_key).unwrap().get_followup_for_part(&part);

        match follow_up {
            Accept => return part.get_value(),
            Reject => return 0,
            Goto(new_key) => current_key = new_key
        }
    }
}

/// Current idea: Find all conditions which lead to accepts. Remove pairs of pos and neg conditions, determine the number of possible values for x,m,a and s based on the remaining conditions and
/// multiply them. Important: as no repetitions are allowed, the following formula must be used: #x * (#m - 1) * (#a - 2) * (#s - 3) where #n is the amount of possible values
#[allow(dead_code)]
fn recursive(key: &str, key_instructions_map: &HashMap<String, Instructions>) -> Vec<Condition> {
    let result = vec![];

    let instructions = key_instructions_map.get(key).unwrap();

    for ins in &instructions.instructions {
        match ins {
            CheckAndGoto(_,_,_,_) => {} // add the condition to the path, if follow_up is accept or goto
            JustGoto(follow_up) => match follow_up {
                Accept => {} // if all other checks don't match
                Reject => {
                    // don't add anything to the final result

                }
                Goto(_) => {
                    // recursive call with new ranges
                }
            }
        }
    }

    result
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
enum Condition {
    Pos(ConditionSpec),
    Neg(ConditionSpec)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct ConditionSpec {
    value: Value,
    check: Check,
    number: usize
}

#[derive(Debug)]
struct Instructions {
    key: String,
    instructions: Vec<Instruction>,
}

impl Instructions {
    fn get_followup_for_part(&self, part: &Part) -> FollowUp {
        self
            .instructions
            .iter()
            .find_map(|ins| ins.get_followup_for_part(part))
            .expect("at least one instruction should return a followup")
    }
}

#[derive(Debug)]
enum Instruction {
    CheckAndGoto(Value, Check, usize, FollowUp),
    JustGoto(FollowUp),
}

impl Instruction {
    fn get_followup_for_part(&self, part: &Part) -> Option<FollowUp> {
        match self {
            CheckAndGoto(value, check, number, follow_up) => {
                let value_to_check = match value {
                    X => part.x,
                    M => part.m,
                    A => part.a,
                    S => part.s,
                };

                let can_follow_up = match check {
                    Greater => value_to_check > *number,
                    Less => value_to_check < *number
                };

                if can_follow_up {
                    Some(follow_up.clone())
                } else {
                    None
                }
            }
            JustGoto(follow_up) => Some(follow_up.clone())
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Value {
    X,
    M,
    A,
    S,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Check {
    Greater,
    Less,
}

#[derive(Clone, Debug)]
enum FollowUp {
    Accept,
    Reject,
    Goto(String),
}

impl From<&str> for Instructions {
    fn from(line: &str) -> Self {
        let mut split = line.split("{");
        let key = split.next().unwrap().to_string();
        let instructions_str = split.next().unwrap().replace("}", "");

        let instructions = instructions_str
            .split(",")
            .map(|instruction| if instruction.contains(":") {
                let split = instruction.split(":").collect::<Vec<_>>();
                let value_str = split[0].index(0..1);
                let check_str = split[0].index(1..2);
                let number = split[0].index(2..).parse::<usize>().unwrap();
                let goto_str = split[1];

                let value = match value_str {
                    "x" => X,
                    "m" => M,
                    "a" => A,
                    "s" => S,
                    _ => panic!("invalid value")
                };

                let check = match check_str {
                    "<" => Less,
                    ">" => Greater,
                    _ => panic!("invalid check")
                };

                let goto = match goto_str {
                    "A" => Accept,
                    "R" => Reject,
                    s => Goto(s.to_string())
                };

                CheckAndGoto(value, check, number, goto)
            } else {
                let follow_up = match instruction {
                    "A" => Accept,
                    "R" => Reject,
                    s => Goto(s.to_string())
                };
                JustGoto(follow_up)
            })
            .collect();

        Instructions {
            key,
            instructions,
        }
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn get_value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl From<&str> for Part {
    fn from(line: &str) -> Self {
        let cleanup = line.replace("{", "").replace("}", "");
        let split = cleanup.split(",").collect::<Vec<_>>();
        let x = split[0].replace("x=", "").parse::<usize>().unwrap();
        let m = split[1].replace("m=", "").parse::<usize>().unwrap();
        let a = split[2].replace("a=", "").parse::<usize>().unwrap();
        let s = split[3].replace("s=", "").parse::<usize>().unwrap();

        Part {
            x,
            m,
            a,
            s,
        }
    }
}