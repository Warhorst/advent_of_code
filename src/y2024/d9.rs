use std::fmt::{Debug, Formatter};
use Entry::*;

pub fn solve_a(input: &str) -> usize {
    let mut file_system = input
        .chars()
        .enumerate()
        .map(|(i, c)| (i, c.to_digit(10).unwrap()))
        .map(|(i, amount)| (match i % 2 == 0 {
            true => File(i / 2),
            false => Empty
        }, amount))
        .flat_map(|(entry, amount)| (0..amount).into_iter().map(move |_| entry))
        .collect::<Vec<_>>();

    compress(&mut file_system);

    file_system
        .into_iter()
        .enumerate()
        .filter_map(|(i, e)| match e {
            File(id) => Some((i, id)),
            Empty => None
        })
        .map(|(i, id)| i * id)
        .sum()
}

fn compress(file_system: &mut Vec<Entry>) {
    let mut head = 0;
    let mut tail = file_system.len() - 1;

    while head < tail {
        match (file_system.get(head).unwrap(), file_system.get(tail).unwrap()) {
            (Empty, Empty) => tail -= 1,
            (Empty, File(_)) => {
                // need to swap
                file_system.swap(head, tail)
            }
            (File(_), Empty) => {
                // just swapped
                head += 1;
                tail -= 1;
            }
            (File(_), File(_)) => head += 1
        }
    }
}

pub fn solve_b(input: &str) -> usize {
    let mut file_system = input
        .chars()
        .enumerate()
        .map(|(i, c)| (i, c.to_digit(10).unwrap() as usize))
        .map(|(i, amount)| (match i % 2 == 0 {
            true => File(i / 2),
            false => Empty
        }, amount))
        .collect::<Vec<_>>();

    compress_whole_files(&mut file_system);

    file_system
        .into_iter()
        .flat_map(|(entry, amount)| (0..amount).into_iter().map(move |_| entry))
        .enumerate()
        .filter_map(|(i, e)| match e {
            File(id) => Some((i, id)),
            Empty => None
        })
        .map(|(i, id)| i * id)
        .sum()
}

fn compress_whole_files(file_system: &mut Vec<(Entry, usize)>) {
    let mut cur_file_index = file_system.len() - 1;

    // go from the end of the file system to the beginning and rearrange any files in your way
    // if the index reaches 0, there is no more file in front of it, so we are done
    while cur_file_index != 0 {
        // if the file at the current index is an empty block, skip to the next
        if let (Empty, _) = file_system[cur_file_index] {
            cur_file_index -= 1;
            continue;
        }

        // find an empty spot from the beginning of the file system with enough space for the current file block
        let index_opt = (0..cur_file_index)
            .into_iter()
            .find(|i| if let ((Empty, amount_a), (File(_), amount_b)) = (file_system[*i], file_system[cur_file_index]) {
                amount_a >= amount_b
            } else {
                false
            });

        match index_opt {
            Some(i) => {
                // a free spot was found, rearrange
                let cur_file = file_system[cur_file_index];
                let potential_entry = file_system[i];

                // this check is already true due to the previous checks, I only do it for destructuring
                if let ((Empty, free), (File(id), file_size)) = (potential_entry, cur_file) {
                    // set the current file block to empty
                    let _ = std::mem::replace(&mut file_system[cur_file_index], (Empty, file_size));
                    // set the found empty block to the file
                    let _ = std::mem::replace(&mut file_system[i], (File(id), file_size));

                    if free - file_size > 0 {
                        // if the empty block provided more space than needed, add the difference as a new empty spot
                        // this does not invalidate cur_file_index, as it now automatically points to the
                        // next element
                        file_system.insert(i + 1, (Empty, free - file_size));
                    }
                } else {
                    unreachable!()
                }
            }
            None => {
                // no empty spot in front of the file with enough space exists, so just skip to the
                // next file
                cur_file_index -= 1
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Entry {
    File(usize),
    Empty,
}

impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            File(id) => id.to_string(),
            Empty => ".".to_string()
        })
    }
}