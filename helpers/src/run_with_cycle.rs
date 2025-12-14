/// Run some operation efficiently a ridiculous number of times.
///
/// The results of the operation form a cycle at some point. This helper finds that cycle, skips
/// all of them and only performs the necessary operations.
pub fn run_n_times_with_cycle<T: Clone + Eq>(
    n: usize,
    value: &mut T,
    operation: impl Fn(&mut T)
) {
    let mut current = value.clone();
    let mut prevs: Vec<T> = vec![];

    for _ in 0..n {
        operation(&mut current);

        if prevs.contains(&current) {
            prevs.push(current.clone());
            break
        } else {
            prevs.push(current.clone());
        }
    }

    let cycle_elem = &prevs[prevs.len() - 1];

    let first_index_of_cycle = prevs
        .iter()
        .enumerate()
        .skip_while(|(_, p)| p != &cycle_elem)
        .map(|(i, _)| i)
        .next().unwrap();

    let diff = n - first_index_of_cycle;
    let cycle_size = prevs.len() - first_index_of_cycle - 1;
    let full_cycle_runs = diff / cycle_size;
    let num_runs_after_last_cycle = diff - full_cycle_runs * cycle_size;

    for _ in 0..(first_index_of_cycle + num_runs_after_last_cycle) {
        operation(value)
    }
}