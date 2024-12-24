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

    let name_connections_map = unique_computer_names
        .into_iter()
        .map(|com_name| {
            let conns = connections.iter().flat_map(|conn| conn.get_other(&com_name)).collect::<HashSet<_>>();
            (com_name, conns)
        })
        .collect::<HashMap<_, _>>();

    let groups = name_connections_map
        .iter()
        .flat_map(|(name, conns)| conns
            .iter()
            .permutations(2)
            .filter(|pair| {
                let com_a_name = pair[0];
                let com_a_connections = name_connections_map.get(com_a_name).unwrap();
                let com_b_name = pair[1];
                let com_b_connections = name_connections_map.get(com_b_name).unwrap();
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

pub fn solve_b(input: &str) -> String {
    let connections = input
        .lines()
        .map(Connection::from)
        .collect::<Vec<_>>();

    let unique_computer_names = connections
        .iter()
        .flat_map(|conn| [conn.a.clone(), conn.b.clone()])
        .collect::<HashSet<_>>();

    let name_connections_map = unique_computer_names
        .into_iter()
        .map(|com_name| {
            let conns = connections.iter().flat_map(|conn| conn.get_other(&com_name)).collect::<HashSet<_>>();
            (com_name, conns)
        })
        .collect::<HashMap<_, _>>();

    // num outgoing connections + itself = max amount of interconnected computers
    let max_group_size = name_connections_map.iter().next().unwrap().1.len() + 1;

    let mut current = max_group_size;

    loop {
        let found = name_connections_map
            .iter()
            .find_map(|(name, conns)| {
                let mut extended = (*conns).clone();
                extended.insert((*name).clone());

                let potential = conns
                    .iter()
                    .filter(|conn| name_connections_map.get(*conn).unwrap().intersection(&extended).count() == current - 1)
                    .collect::<HashSet<_>>();

                if potential.len() == current - 1 {
                    Some((name, potential))
                } else {
                    None
                }
            })
            .map(|(name, potential)| {
                let mut vec = vec![name.clone()];
                potential.iter().for_each(|com| vec.push((*com).clone()));
                vec.sort();
                vec.into_iter().join(",")
            });

        if found.is_some() {
            break found;
        }

        current -= 1;

        if current == 0 {
            break None
        }

    }.unwrap()
}

struct Connection {
    a: String,
    b: String,
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