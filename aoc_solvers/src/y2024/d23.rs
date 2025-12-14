use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn solve_a(input: &str) -> usize {
    let connections = input
        .lines()
        .map(Connection::from)
        .collect::<Vec<_>>();

    let name_connections_map = connections
        .iter()
        .flat_map(|conn| [conn.a.clone(), conn.b.clone()])
        .map(|com_name| {
            let conns = connections.iter().flat_map(|conn| conn.get_other(&com_name)).collect::<HashSet<_>>();
            (com_name, conns)
        })
        .collect::<HashMap<_, _>>();

    // Just compare the currently viewed computer with the 2-permutations of its connections and
    // check if they form a group of size 3 (they must all be interconnected)
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

    // count the groups where at least one computer starts with 't'
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

    let name_connections_map = connections
        .iter()
        .flat_map(|conn| [conn.a.clone(), conn.b.clone()])
        .map(|com_name| {
            let conns = connections.iter().flat_map(|conn| conn.get_other(&com_name)).collect::<HashSet<_>>();
            (com_name, conns)
        })
        .collect::<HashMap<_, _>>();

    // The maximum amount of members in a fully connected group is the amount of connections a computer
    // has plus itself. In the example, every computer has the same amount of connections, so the
    // maximum amount of connections is just the amount of connections of the first computer
    let max_group_size = name_connections_map.iter().next().unwrap().1.len() + 1;

    // The idea is to go from the maximum amount of computers in the group to potentially zero and check if
    // a group of this size can be formed with anyone. If not, try to lower the expected group size by one
    // and check again.
    let mut current = max_group_size;
    loop {
        // With the currently expected group size, try to find a set of computers of this size
        // where all are interconnected to each other.
        let found = name_connections_map
            .iter()
            .find_map(|(name, conns)| {
                // create a temporary extended connection set which contains also the current computer
                let mut extended = (*conns).clone();
                extended.insert((*name).clone());

                // Iterate over every computer in the connection, get its connections and count the amount
                // of elements in the intersection between its connections and the extended connection set.
                // Keep only the computers where the intersection is current - 1 big.
                // This works because for example the current expected group size is 4 (like in the example),
                // the currently viewed computers ('com') connections must have an intersection size of 3, the
                // computer 'name' and 2 other. Together with com, this is 4. The computer 'name' must find current - 1
                // computers in its connection which fulfill this condition. 'name' and the other computers than are
                // the searched group
                let set = conns
                    .iter()
                    .filter(|com| name_connections_map.get(*com).unwrap().intersection(&extended).count() == current - 1)
                    .collect::<HashSet<_>>();

                if set.len() == current - 1 {
                    Some((name, set))
                } else {
                    None
                }
            })
            .map(|(name, set)| {
                let mut vec = vec![name.clone()];
                set.iter().for_each(|com| vec.push((*com).clone()));
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