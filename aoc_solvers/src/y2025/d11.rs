use std::collections::{HashMap, HashSet};

pub fn solve_a(input: &str) -> usize {
    // Use dynamic programming to just perform DFS on the graph.
    // This assumes the graph is a https://en.wikipedia.org/wiki/Directed_acyclic_graph, which it is.
    
    let mut edges = HashSet::new();

    for line in input.lines() {
        let split = line.split_once(": ").unwrap();
        let from = split.0;

        for s in split.1.split(" ") {
            edges.insert((from.to_string(), s.to_string()));
        }
    }

    count_paths("you", "out", &edges)
}

fn count_paths(
    current: &str,
    goal: &str,
    edges: &HashSet<(String, String)>,
) -> usize {
    let mut sum = 0;

    for (_, next) in edges.iter().filter(|e| e.0 == current) {
        if next == goal {
            sum += 1;
        } else {
            sum += count_paths(next, goal, edges);
        }
    }

    sum
}

pub fn solve_b(input: &str) -> usize {
    // Basically the same as A, but with the fft-dac-constraint and good old memoization.
    
    let mut edges = HashSet::new();

    for line in input.lines() {
        let split = line.split_once(": ").unwrap();
        let from = split.0;

        for s in split.1.split(" ") {
            edges.insert((from.to_string(), s.to_string()));
        }
    }

    let mut cache = HashMap::new();
    count_paths_b("svr", "out", false, false, &edges, &mut cache)
}

fn count_paths_b(
    current: &str,
    goal: &str,
    visited_fft: bool,
    visited_dac: bool,
    edges: &HashSet<(String, String)>,
    cache: &mut HashMap<(String, String, bool, bool), usize>
) -> usize {
    if let Some(r) = cache.get(&(current.to_string(), goal.to_string(), visited_fft, visited_dac)) {
        return *r
    }
    
    let mut sum = 0;

    for (_, next) in edges.iter().filter(|e| e.0 == current) {
        if current == "fft" && !visited_dac {
            // Current is fft and dac was not visited yet, so go deeper with fft_visited = true
            sum += count_paths_b(next, goal, true, visited_dac, edges, cache)
        } else if current == "dac" && visited_fft {
            // Current is dac and fft was visited, so go deeper with dac_visited = true
            sum += count_paths_b(next, goal, visited_fft, true, edges, cache)
        } else if next == goal {
            if visited_fft && visited_dac {
                // The goal is reached and fft and dac were visited in order. One new path was found
                sum += 1
            } else {
                // Goal was reached but the fft-dac-constraint was not fulfilled. No new path found.
                sum += 0
            }
        } else {
            sum += count_paths_b(next, goal, visited_fft, visited_dac, edges, cache)
        }
    }

    cache.insert((current.to_string(), goal.to_string(), visited_fft, visited_dac), sum);

    sum
}
