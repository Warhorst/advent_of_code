pub fn solve_6a(input: &str) -> usize {
    let times = input
        .lines()
        .next()
        .unwrap()
        .replace("Time:", "")
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let distances = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .replace("Distance:", "")
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut pairs = vec![];

    for (i, time) in times.iter().enumerate() {
        pairs.push((time, distances[i]))
    }

    pairs
        .into_iter()
        .map(|(time, distance)| get_max_distances(*time, distance).len())
        .product()
}

pub fn solve_6b(input: &str) -> usize {
    let time = input
        .lines()
        .next()
        .unwrap()
        .replace("Time:", "")
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let distance = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .replace("Distance:", "")
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    get_max_distances(time, distance).len()
}

fn get_max_distances(time: usize, distance: usize) -> Vec<usize> {
    let mut results = vec![];

    for charge_time in 0..=time {
        let speed_per_ms = charge_time;
        let time_to_drive = time - charge_time;
        let driven_distance = speed_per_ms * time_to_drive;

        if driven_distance > distance {
            results.push(charge_time)
        }
    }

    results
}