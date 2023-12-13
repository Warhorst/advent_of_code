use rayon::prelude::*;

pub fn solve_5a(input: &str) -> usize {
    let seeds = parse_seeds(input.lines().next().unwrap());

    let mappings = collect_mappings(input);

    seeds
        .iter()
        .map(|seed| get_destination(&mappings, *seed))
        .min().unwrap()
}

pub fn solve_5b(input: &str) -> usize {
    let seeds = parse_seeds(input.lines().next().unwrap());
    let mappings = collect_mappings(input);

    seeds
        .windows(2)
        .enumerate()
        .par_bridge()
        .filter(|(i, _)| i % 2 == 0)
        .flat_map(|(_, w)| {
            let start = w[0];
            let range = w[1];

            (start..(start + range))
                .into_par_iter()
                .map(|seed| get_destination(&mappings, seed))
        })
        .min()
        .unwrap()
}

fn parse_seeds(line: &str) -> Vec<usize> {
    line
        .replace("seeds:", "")
        .trim()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn collect_mappings(input: &str) -> Vec<Mapping> {
    let mut mappings = vec![];
    let mut current_mapping = Mapping::default();

    input
        .lines()
        .into_iter()
        .skip(2)
        .for_each(|line| {
            if line.is_empty() {
                mappings.push(current_mapping.clone());
                current_mapping = Mapping::default();
            } else if line.contains("map") {
                current_mapping.name = line.to_string();
            } else {
                current_mapping.ranges.push(Range::from_line(line))
            }
        });

    mappings.push(current_mapping);

    mappings
}

fn get_destination(mappings: &Vec<Mapping>, seed: usize) -> usize {
    let mut destination = seed;

    mappings.iter().for_each(|map| destination = map.map_source(destination));

    destination
}

#[derive(Clone, Debug, Default)]
struct Mapping {
    name: String,
    ranges: Vec<Range>
}

impl Mapping {
    fn map_source(&self, source: usize) -> usize {
        let range_opt = self.ranges
            .iter()
            .find(|r| r.contains_source(source));

        match range_opt {
            Some(r) => r.get_destination(source),
            None => source
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Range {
    destination_range: usize,
    source_range: usize,
    range_length: usize,
}

impl Range {
    fn from_line(line: &str) -> Self {
        let split = line.split(" ").collect::<Vec<_>>();

        Range {
            destination_range: split[0].parse::<usize>().unwrap(),
            source_range: split[1].parse::<usize>().unwrap(),
            range_length: split[2].parse::<usize>().unwrap(),
        }
    }

    fn get_destination(&self, source: usize) -> usize {
        let diff = source - self.source_range;
        self.destination_range + diff
    }

    fn contains_source(&self, source: usize) -> bool {
        (self.source_range..(self.source_range + self.range_length)).contains(&source)
    }
}