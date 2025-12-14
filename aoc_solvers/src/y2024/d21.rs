use std::collections::HashMap;
use std::fmt::Formatter;
use helpers::prelude::*;
use KeypadButton::*;
use ControlButton::*;

pub fn solve_a(input: &str) -> usize {
    input
        .lines()
        .map(|line| (get_numerical_part(line), line.chars().map(KeypadButton::from).collect::<Vec<_>>()))
        .map(|(num, inputs)| (
            num,
            get_required_inputs_for_control(
                get_required_inputs_for_control(
                    get_required_inputs_for_keypad(inputs)
                )
            ).len()
        ))
        .map(|(num, amount_inputs)| num * amount_inputs)
        .sum()
}

fn get_numerical_part(line: &str) -> usize {
    parse(&line.chars().take(line.len() - 1).skip_while(|c| *c == '0').collect::<String>())
}

fn get_required_inputs_for_keypad(mut keypad_inputs: Vec<KeypadButton>) -> Vec<ControlButton> {
    keypad_inputs.insert(0, Kba);

    let res = keypad_inputs
        .windows(2)
        .flat_map(|w| get_keypad_inputs_from_to(w[0], w[1]))
        .collect();

    println!("{res:?}");

    res
}

fn get_keypad_inputs_from_to(current: KeypadButton, target: KeypadButton) -> Vec<ControlButton> {
    let mut inputs = vec![];

    // get the keys position on the keypad
    let current_pos = current.get_position();
    let target_pos = target.get_position();
    // calculate the diff to determine how the arm must be moved
    let (x_diff, y_diff) = (target_pos.x - current_pos.x, target_pos.y - current_pos.y);

    // on the control pad, moving farther left is more expensive than moving up or down,
    // while moving up or down is more expensive than moving right or activate,
    // so perform moves in this order (but don't hover over the gap, switch order then)

    // on the keypad, the Gap is at (0, 0)

    if x_diff < 0 && current_pos + (x_diff, 0) == p!(0, 0) {
        match y_diff {
            yd if y_diff > 0 => (0..yd).for_each(|_| inputs.push(Up)),
            yd if y_diff < 0 => (0..yd.abs()).for_each(|_| inputs.push(Down)),
            _ => {}
        }
        (0..x_diff.abs()).for_each(|_| inputs.push(Left));
    } else if x_diff < 0 {
        (0..x_diff.abs()).for_each(|_| inputs.push(Left));
        match y_diff {
            yd if y_diff > 0 => (0..yd).for_each(|_| inputs.push(Up)),
            yd if y_diff < 0 => (0..yd.abs()).for_each(|_| inputs.push(Down)),
            _ => {}
        }
    } else if current_pos + (0, y_diff) == p!(0, 0) {
        (0..x_diff).for_each(|_| inputs.push(Right));
        match y_diff {
            yd if y_diff > 0 => (0..yd).for_each(|_| inputs.push(Up)),
            yd if y_diff < 0 => (0..yd.abs()).for_each(|_| inputs.push(Down)),
            _ => {}
        }
    } else {
        match y_diff {
            yd if y_diff > 0 => (0..yd).for_each(|_| inputs.push(Up)),
            yd if y_diff < 0 => (0..yd.abs()).for_each(|_| inputs.push(Down)),
            _ => {}
        }
        (0..x_diff).for_each(|_| inputs.push(Right))
    }

    inputs.push(Activate);

    inputs
}

fn get_required_inputs_for_control(mut control_inputs: Vec<ControlButton>) -> Vec<ControlButton> {
    control_inputs.insert(0, Activate);

    let res = control_inputs
        .windows(2)
        .flat_map(|w| get_control_inputs_from_to(w[0], w[1]))
        .collect();

    println!("{res:?}");

    res
}

fn get_control_inputs_from_to(current: ControlButton, target: ControlButton) -> Vec<ControlButton> {
    let mut inputs = vec![];

    // get the keys position on the keypad
    let current_pos = current.get_position();
    let target_pos = target.get_position();
    // calculate the diff to determine how the arm must be moved
    let (x_diff, y_diff) = (target_pos.x - current_pos.x, target_pos.y - current_pos.y);

    // on the control pad, moving farther left is more expensive than moving up or down,
    // while moving up or down is more expensive than moving right or activate,
    // so perform moves in this order (but don't hover over the gap, switch order then)

    // on the keypad, the Gap is at (0, 1)

    if x_diff < 0 && current_pos + (x_diff, 0) == p!(0, 1) {
        match y_diff {
            _ if y_diff > 0 => inputs.push(Up),
            _ if y_diff < 0 => inputs.push(Down),
            _ => {}
        }
        (0..x_diff.abs()).for_each(|_| inputs.push(Left));
    } else if x_diff < 0 {
        (0..x_diff.abs()).for_each(|_| inputs.push(Left));
        match y_diff {
            _ if y_diff > 0 => inputs.push(Up),
            _ if y_diff < 0 => inputs.push(Down),
            _ => {}
        }
    } else if current_pos + (0, y_diff) == p!(0, 1) {
        (0..x_diff).for_each(|_| inputs.push(Right));

        match y_diff {
            _ if y_diff > 0 => inputs.push(Up),
            _ if y_diff < 0 => inputs.push(Down),
            _ => {}
        }
    } else {
        match y_diff {
            _ if y_diff > 0 => inputs.push(Up),
            _ if y_diff < 0 => inputs.push(Down),
            _ => {}
        }
        (0..x_diff).for_each(|_| inputs.push(Right))
    }

    inputs.push(Activate);

    inputs
}

pub fn solve_b(input: &str) -> usize {
    // todo current state: The idea is simple: transform the current version of A into a recursive version,
    //  use a cache and call it until depth 25. The problem: I just don't get it how to implement this
    //  recursively. I am extremely burned out of this puzzle and might try it later one day

    //let mut inputs = get_required_inputs_for_keypad(inputs);
    //(0..2)
    //    .into_iter()
    //    .for_each(|_| inputs = get_required_inputs_for_control(inputs.clone()));
    //inputs.len();

    input
        .lines()
        .map(|line| (get_numerical_part(line), line.chars().map(KeypadButton::from).collect::<Vec<_>>()))
        .map(|(num, inputs)| (
            num,
            get_num_keypad_presses(inputs, 1)
        ))
        .map(|(num, amount_inputs)| num * amount_inputs)
        .sum()
}

fn get_num_keypad_presses(inputs: Vec<KeypadButton>, depth: usize) -> usize {
    let inputs = get_required_inputs_for_keypad(inputs);
    let mut cache = HashMap::new();

    println!("{inputs:?}");

    //inputs.insert(0, Activate);

    let sum = inputs
        .windows(2)
        .enumerate()
        .map(|(i, w)| get_num_control_presses(w[0], w[1], i == 0, depth, &mut cache))
        .sum();

    println!("{sum}");

    sum
}

fn get_num_control_presses(
    current: ControlButton,
    target: ControlButton,
    is_first: bool,
    current_depth: usize,
    cache: &mut HashMap<(ControlButton, ControlButton, usize), usize>,
) -> usize {
    //let amount_opt = cache.get(&(current, target, current_depth)).copied();
    let amount_opt = None;

    match amount_opt {
        Some(amount) => {
            println!("Returning cached value for: {:?}, amount: {amount}", (current, target, current_depth));
            amount
        }
        None => {
            let new = if current_depth == 0 {
                //get_control_inputs_from_to(current, target).len()
                let inputs = if is_first {
                    let mut inputs = get_control_inputs_from_to(Activate, current);
                    inputs.extend(get_control_inputs_from_to(current, target));
                    inputs
                } else {
                    get_control_inputs_from_to(current, target)
                };

                println!("({current:?} to {target:?}), Inputs: {inputs:?}, Depth: {current_depth}");

                inputs.len()
            } else {
                //let mut inputs = get_control_inputs_from_to(current, target);

                let inputs = if is_first {
                    let mut inputs = get_control_inputs_from_to(Activate, current);
                    inputs.extend(get_control_inputs_from_to(current, target));
                    inputs
                } else {
                    get_control_inputs_from_to(current, target)
                };

                println!("({current:?} to {target:?}), Inputs: {inputs:?}, Depth: {current_depth}");

                // todo breaks when input is only one element long

                //if inputs.len() == 1 && next.is_some() {
                //    inputs.extend(get_control_inputs_from_to(target, next.unwrap()));
                //}

                if inputs.len() == 1 {
                    3 * current_depth
                } else {
                    inputs
                        .windows(2)
                        .enumerate()
                        .map(|(i, w)| get_num_control_presses(w[0], w[1], is_first && i == 0, current_depth - 1, cache))
                        .sum()
                }


            };

            cache.insert((current, target, current_depth), new);

            new
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum KeypadButton {
    KB0,
    KB1,
    KB2,
    KB3,
    KB4,
    KB5,
    KB6,
    KB7,
    KB8,
    KB9,
    Kba,
}

impl KeypadButton {
    fn get_position(&self) -> Position {
        match self {
            KB0 => p!(1, 0),
            KB1 => p!(0, 1),
            KB2 => p!(1, 1),
            KB3 => p!(2, 1),
            KB4 => p!(0, 2),
            KB5 => p!(1, 2),
            KB6 => p!(2, 2),
            KB7 => p!(0, 3),
            KB8 => p!(1, 3),
            KB9 => p!(2, 3),
            Kba => p!(2, 0),
        }
    }
}

impl From<char> for KeypadButton {
    fn from(value: char) -> Self {
        match value {
            '0' => KB0,
            '1' => KB1,
            '2' => KB2,
            '3' => KB3,
            '4' => KB4,
            '5' => KB5,
            '6' => KB6,
            '7' => KB7,
            '8' => KB8,
            '9' => KB9,
            'A' => Kba,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum ControlButton {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl ControlButton {
    fn get_position(&self) -> Position {
        match self {
            Up => p!(1, 1),
            Down => p!(1, 0),
            Left => p!(0, 0),
            Right => p!(2, 0),
            Activate => p!(2, 1),
        }
    }
}

impl std::fmt::Debug for ControlButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Up => '^',
            Down => 'v',
            Left => '<',
            Right => '>',
            Activate => 'A',
        })
    }
}
