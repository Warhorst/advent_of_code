pub fn solve_9a(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.split(" ").map(|s| s.parse::<isize>().unwrap()).collect::<Vec<_>>())
        .map(get_differences)
        .map(extrapolate)
        .sum()
}

pub fn solve_9b(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.split(" ").map(|s| s.parse::<isize>().unwrap()).collect::<Vec<_>>())
        .map(get_differences)
        .map(extrapolate_backwards)
        .sum()
}

fn get_differences(report: Vec<isize>) -> Vec<Vec<isize>> {
    let mut current_differences = report.clone();
    let mut differences: Vec<Vec<isize>> = vec![report];

    while differences.is_empty() || !differences.last().unwrap().iter().all(|n| *n == 0) {
        let mut new_differences = vec![];

        for (i, num) in current_differences.iter().enumerate().take(current_differences.len() - 1) {
            new_differences.push(current_differences[i + 1] - *num)
        }

        current_differences = new_differences.clone();
        differences.push(new_differences);
    }

    differences
}

fn extrapolate(differences: Vec<Vec<isize>>) -> isize {
    differences
        .into_iter()
        .rev()
        .map(|v| v[v.len() - 1])
        .fold(0, |acc, val| acc + val)
}

fn extrapolate_backwards(differences: Vec<Vec<isize>>) -> isize {
    differences
        .into_iter()
        .rev()
        .map(|v| v[0])
        .fold(0, |acc, val| val - acc)
}
