use Operation::*;

pub fn solve_a(input: &str) -> usize {
    input.split(",")
        .map(calculate_hash)
        .sum()
}

pub fn solve_b(input: &str) -> usize {
    let entries = input
        .split(",")
        .map(Entry::from)
        .collect::<Vec<_>>();

    let mut boxes = vec![Vec::<Entry>::new(); 256];

    for entry in entries {
        let bx = boxes.get_mut(entry.box_index).unwrap();
        let index_of_existing = bx
            .iter()
            .enumerate()
            .find_map(|(i, e)| if e.label == entry.label {
                Some(i)
            } else {
                None
            });

        match entry.operation {
            Remove => if let Some(i) = index_of_existing {
                bx.remove(i);
            }
            Add(_) => match index_of_existing {
                None => bx.push(entry),
                Some(i) => { let _ = std::mem::replace(&mut bx[i], entry); }
            }
        }
    }

    boxes
        .into_iter()
        .flat_map(|bx| bx
            .into_iter()
            .enumerate()
            .map(|(i, e)| (e.box_index + 1) * (i + 1) * match e.operation {
                Add(fl) => fl,
                Remove => panic!("invalid state")
            }))
        .sum()
}

fn calculate_hash(s: &str) -> usize {
    let mut current_value = 0;

    for c in s.chars() {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

#[derive(Clone, Debug)]
struct Entry {
    label: String,
    box_index: usize,
    operation: Operation,
}

impl From<&str> for Entry {
    fn from(s: &str) -> Self {
        if s.ends_with("-") {
            let label = s.replace("-", "");
            Entry {
                box_index: calculate_hash(&label),
                label,
                operation: Remove,
            }
        } else {
            let mut split = s.split("=");
            let label = split.next().unwrap().to_string();
            Entry {
                box_index: calculate_hash(&label),
                label,
                operation: Add(split.next().unwrap().parse::<usize>().unwrap()),
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Remove,
    Add(usize),
}