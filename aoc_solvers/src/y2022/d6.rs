use std::collections::HashSet;

pub fn solve_a(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<_>>();

    chars
        .windows(4)
        .enumerate()
        .find_map(|(index, w)| if w.iter().collect::<HashSet<_>>().len() == 4 { Some(index) } else { None })
        .unwrap() + 4
}

pub fn solve_b(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<_>>();

    chars
        .windows(14)
        .enumerate()
        .find_map(|(index, w)| if w.iter().collect::<HashSet<_>>().len() == 14 { Some(index) } else { None })
        .unwrap() + 14
}