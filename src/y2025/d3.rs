pub fn solve_a(input: &str) -> usize {
    input
        .lines()
        .map(|line| largest_line_value::<2>(line))
        .sum()
}

pub fn solve_b(input: &str) -> usize {
    input
        .lines()
        .map(|line| largest_line_value::<12>(line))
        .sum()
}

fn largest_line_value<const C: usize>(line: &str) -> usize {
    let digits = line
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let mut selection = [0; C];
    let mut current_index = 0;
    let mut count = 0;

    for i in (1..=C).rev() {
        let (index, digit) = digits
            .iter()
            .enumerate()
            .skip(current_index)
            .filter(|(index, _)| *index < digits.len() - (i - 1))
            .max_by(|(i0, d0), (i1, d1)| d0.cmp(d1).then(i1.cmp(i0)))
            .unwrap();

        selection[count] = *digit;
        count += 1;
        current_index = index + 1;
    }

    selection
        .into_iter()
        .enumerate()
        .map(|(i, digit)| digit as usize * 10usize.pow((C - 1 - i) as u32))
        .sum()
}
