use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn solve_a(input: &str) -> usize {
    let connections = input
        .lines()
        .map(Connection::from)
        .collect::<Vec<_>>();

    let unique_computer_names = connections
        .iter()
        .flat_map(|conn| [conn.a.clone(), conn.b.clone()])
        .collect::<HashSet<_>>();

    let name_computer_map = unique_computer_names
        .into_iter()
        .map(|com_name| {
            let conns = connections.iter().flat_map(|conn| conn.get_other(&com_name)).collect::<HashSet<_>>();
            (com_name, conns)
        })
        .collect::<HashMap<_, _>>();

    let groups = name_computer_map
        .iter()
        .flat_map(|(name, conns)| conns
            .iter()
            .permutations(2)
            .filter(|pair| {
                let com_a_name = pair[0];
                let com_a_connections = name_computer_map.get(com_a_name).unwrap();
                let com_b_name = pair[1];
                let com_b_connections = name_computer_map.get(com_b_name).unwrap();
                com_a_connections.contains(name) && com_b_connections.contains(name) && com_a_connections.contains(com_b_name) && com_b_connections.contains(com_a_name)
            })
            .map(|pair| {
                let mut group = vec![name.clone(), pair[0].clone(), pair[1].clone()];
                group.sort();
                group
            })
        )
        .collect::<HashSet<_>>();

    groups
        .iter()
        .filter(|g| g.iter().any(|name| name.starts_with("t")))
        .count()
}

pub fn solve_b(_input: &str) -> usize {
    0
}

struct Connection {
    a: String,
    b: String
}

impl Connection {
    //fn contains(&self, computer: &Computer) -> bool {
    //    self.a == *computer || self.b == *computer
    //}
//
    fn get_other(&self, computer: &String) -> Option<String> {
        if self.a == *computer {
            Some(self.b.clone())
        } else if self.b == *computer {
            Some(self.a.clone())
        } else {
            None
        }
    }
}

impl From<&str> for Connection {
    fn from(value: &str) -> Self {
        let mut split = value.split("-");

        Connection {
            a: split.next().unwrap().to_string(),
            b: split.next().unwrap().to_string(),
        }
    }
}