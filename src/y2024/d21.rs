use std::fmt::Formatter;
use crate::aoc_lib::*;
use KeypadButton::*;
use ControlButton::*;

pub fn solve_a(input: &str) -> usize {
    // too high: 158428

    // todo does not work, because going left is more expensive than other moves

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
        .inspect(|(num, amount_inputs)| println!("{}, {}", num, amount_inputs))
        .map(|(num, amount_inputs)| num * amount_inputs)
        .sum()
}

fn get_numerical_part(line: &str) -> usize {
    parse(&line.chars().take(line.len() - 1).skip_while(|c| *c == '0').collect::<String>())
}

fn get_required_inputs_for_keypad(mut keypad_inputs: Vec<KeypadButton>) -> Vec<ControlButton> {
    keypad_inputs.insert(0, KBA);

    keypad_inputs
        .windows(2)
        .flat_map(|w| get_keypad_inputs_from_to(w[0], w[1]))
        .collect()
}

fn get_keypad_inputs_from_to(current: KeypadButton, target: KeypadButton) -> Vec<ControlButton> {
    let mut inputs = vec![];

    let current_pos = get_keypad_button_pos(current);
    let target_pos = get_keypad_button_pos(target);
    let (x_diff, y_diff) = (target_pos.x - current_pos.x, target_pos.y - current_pos.y);

    if current_pos + (x_diff, 0) == p!(0, 0) {
        // going x first would move over gap, which is invalid, so move over y first

        match y_diff {
            yd if y_diff > 0 => (0..yd).into_iter().for_each(|_| inputs.push(Up)),
            yd if y_diff < 0 => (0..yd.abs()).into_iter().for_each(|_| inputs.push(Down)),
            _ => {}
        }

        match x_diff {
            xd if x_diff > 0 => (0..xd).into_iter().for_each(|_| inputs.push(Right)),
            xd if x_diff < 0 => (0..xd.abs()).into_iter().for_each(|_| inputs.push(Left)),
            _ => {}
        }
    } else {
        match x_diff {
            xd if x_diff > 0 => (0..xd).into_iter().for_each(|_| inputs.push(Right)),
            xd if x_diff < 0 => (0..xd.abs()).into_iter().for_each(|_| inputs.push(Left)),
            _ => {}
        }

        match y_diff {
            yd if y_diff > 0 => (0..yd).into_iter().for_each(|_| inputs.push(Up)),
            yd if y_diff < 0 => (0..yd.abs()).into_iter().for_each(|_| inputs.push(Down)),
            _ => {}
        }
    }

    inputs.push(Activate);

    inputs
}

fn get_keypad_button_pos(button: KeypadButton) -> Position {
    match button {
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
        KBA => p!(2, 0),
        //KGap => p!(0, 0),
    }
}

fn get_required_inputs_for_control(mut control_inputs: Vec<ControlButton>) -> Vec<ControlButton> {
    control_inputs.insert(0, Activate);

    control_inputs
        .windows(2)
        .flat_map(|w| get_control_inputs_from_to(w[0], w[1]))
        .collect()
}

fn get_control_inputs_from_to(current: ControlButton, target: ControlButton) -> Vec<ControlButton> {
    let mut inputs = vec![];

    let current_pos = get_control_button_pos(current);
    let target_pos = get_control_button_pos(target);
    let (x_diff, y_diff) = (target_pos.x - current_pos.x, target_pos.y - current_pos.y);

    if current_pos + (x_diff, 0) == p!(0, 1) {
        // going x first would move over gap, which is invalid, so move over y first

        match y_diff {
            yd if y_diff > 0 => (0..yd).into_iter().for_each(|_| inputs.push(Up)),
            yd if y_diff < 0 => (0..yd.abs()).into_iter().for_each(|_| inputs.push(Down)),
            _ => {}
        }

        match x_diff {
            xd if x_diff > 0 => (0..xd).into_iter().for_each(|_| inputs.push(Right)),
            xd if x_diff < 0 => (0..xd.abs()).into_iter().for_each(|_| inputs.push(Left)),
            _ => {}
        }
    } else {
        match x_diff {
            xd if x_diff > 0 => (0..xd).into_iter().for_each(|_| inputs.push(Right)),
            xd if x_diff < 0 => (0..xd.abs()).into_iter().for_each(|_| inputs.push(Left)),
            _ => {}
        }

        match y_diff {
            yd if y_diff > 0 => (0..yd).into_iter().for_each(|_| inputs.push(Up)),
            yd if y_diff < 0 => (0..yd.abs()).into_iter().for_each(|_| inputs.push(Down)),
            _ => {}
        }
    }

    inputs.push(Activate);

    inputs
}

fn get_control_button_pos(button: ControlButton) -> Position {
    match button {
        Up => p!(1, 1),
        Down => p!(1, 0),
        Left => p!(0, 0),
        Right => p!(2, 0),
        Activate => p!(2, 1),
        //CGap => p!(0, 1),
    }
}

pub fn solve_b(_input: &str) -> usize {
    0
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
    KBA,
    //KGap,
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
            'A' => KBA,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Copy)]
enum ControlButton {
    Up,
    Down,
    Left,
    Right,
    Activate,
    //CGap,
}

impl std::fmt::Debug for ControlButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Up => '^',
            Down => 'v',
            Left => '<',
            Right => '>',
            Activate => 'A',
            //CGap => 'G'
        })
    }
}