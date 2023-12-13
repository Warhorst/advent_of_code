use std::collections::HashMap;

use num::Integer;

pub fn solve_8a(input: &str) -> usize {
    let instructions = input.lines().next().unwrap().chars().collect::<Vec<_>>();

    let mut key_ways_map = HashMap::new();

    for line in input.lines().skip(2) {
        let split = line.split(" = ").collect::<Vec<_>>();
        let key = split[0].to_string();
        let foo = split[1].replace("(", "").replace(")", "");
        let values = foo.split(", ").collect::<Vec<_>>();
        key_ways_map.insert(key, (values[0].to_string(), values[1].to_string()));
    }

    let start = "AAA".to_string();
    let mut current = start.clone();
    let mut count = 0;

    'outer: loop {
        for ins in &instructions {
            if *ins == 'L' {
                current = key_ways_map.get(&current).unwrap().0.clone()
            } else {
                current = key_ways_map.get(&current).unwrap().1.clone()
            }

            count += 1;

            if current == "ZZZ".to_string() {
                break 'outer;
            }
        }
    }

    count
}

pub fn solve_8b(input: &str) -> usize {
    let instructions = input.lines().next().unwrap().chars().collect::<Vec<_>>();

    let mut key_ways_map = HashMap::new();

    for line in input.lines().skip(2) {
        let split = line.split(" = ").collect::<Vec<_>>();
        let key = split[0].to_string();
        let foo = split[1].replace("(", "").replace(")", "");
        let values = foo.split(", ").collect::<Vec<_>>();
        key_ways_map.insert(key, (values[0].to_string(), values[1].to_string()));
    }

    let current_nodes = key_ways_map
        .keys()
        .filter(|s| s.ends_with("A"))
        .cloned()
        .collect::<Vec<_>>();

    let mut counts = vec![];

    for mut current in current_nodes.into_iter() {
        let mut count = 0;

        'outer: loop {
            for ins in &instructions {
                if *ins == 'L' {
                    current = key_ways_map.get(&current).unwrap().0.clone()
                } else {
                    current = key_ways_map.get(&current).unwrap().1.clone()
                }

                count += 1;

                if current.ends_with("Z") {
                    break 'outer;
                }
            }
        }

        counts.push(count)
    }

    let mut lcm = counts[0];

    for count in counts.into_iter().skip(1) {
        lcm = count.lcm(&lcm)
    }

    lcm
}
