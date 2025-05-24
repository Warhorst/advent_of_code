use std::collections::HashSet;

pub fn solve_a(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first_half = &line[0..line.len() / 2];
            let second_half = &line[line.len() / 2..];
            (first_half, second_half)
        })
        .flat_map(|(first, second)| first
            .chars()
            .find(|a| second
                .chars()
                .any(|b| *a == b)
            )
        )
        .map(|c| value(c))
        .sum()
}

pub fn solve_b(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();

    lines.windows(3)
        .step_by(3)
        .map(|block| {
            let a = block[0].chars().collect::<HashSet<_>>();
            let b = block[1].chars().collect::<HashSet<_>>();
            let c = block[2].chars().collect::<HashSet<_>>();

            // bitwise & is the intersection operator
            (&(&a & &b) & &c).into_iter().next().unwrap()
        })
        .map(|c| value(c))
        .sum()
}

fn value(c: char) -> usize {
    if c.is_lowercase() {
        c as usize - 96
    } else {
        c as usize - 64 + 26
    }
}