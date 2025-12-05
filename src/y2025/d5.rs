use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let split = input.split_once("\n\n").unwrap();

    let ranges = split
        .0
        .lines()
        .map(|line| {
            let split = line.split_once("-").unwrap();
            parse::<usize>(split.0)..=parse::<usize>(split.1)
        })
        .collect::<Vec<_>>();

    split
        .1
        .lines()
        .map(parse::<usize>)
        .filter(|num| ranges.iter().any(|r| r.contains(num)))
        .count()
}

pub fn solve_b(input: &str) -> usize {
    // A very simple approach would be to just collect all the numbers in a set and return its lenght.
    // This, however, takes forever and uses large amounts of RAM. So instead I merge the ranges which overlap
    // and return the sum of their elements.
    
    let split = input.split_once("\n\n").unwrap();

    let mut ranges = split
        .0
        .lines()
        .map(|line| {
            let split = line.split_once("-").unwrap();
            parse::<usize>(split.0)..=parse::<usize>(split.1)
        })
        .collect::<Vec<_>>();

    // Sort by start, so that possible overlapping ranges are next to each other
    ranges.sort_by(|r0, r1| r0.start().cmp(r1.start()));

    let mut merged_ranges = vec![];
    let mut i = 0;
    let mut current_range = None;

    loop {
        if current_range.is_none() {
            current_range = Some(ranges[i].clone());
        }

        if let Some(r) = &current_range
            && r.contains(ranges[i + 1].start())
        {
            // The current range overlaps with the next range, so merge
            current_range = Some(*r.start()..=*(r.end().max(ranges[i + 1].end())))
        } else {
            // The ranges don't overlap, so this merge is done
            merged_ranges.push(current_range.take().unwrap());
        }

        i += 1;

        if i == ranges.len() - 1 {
            // When the loop ends, put the current range in the merged ranges
            merged_ranges.push(current_range.take().unwrap());
            break;
        }
    }

    merged_ranges.into_iter().map(|r| r.count()).sum()
}
