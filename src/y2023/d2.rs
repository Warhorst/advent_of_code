pub fn solve_2a(input: &str) -> usize {
    input
        .lines()
        .map(Game::from_line)
        .filter(|g| g.max_red <= 12 && g.max_green <= 13 && g.max_blue <= 14)
        .map(|g| g.id)
        .sum()
}

pub fn solve_2b(input: &str) -> usize {
    input
        .lines()
        .map(Game::from_line)
        .map(|g| g.max_red * g.max_green * g.max_blue)
        .sum()
}

#[derive(Debug)]
struct Game {
    id: usize,
    max_red: usize,
    max_blue: usize,
    max_green: usize,
}

impl Game {
    fn from_line(line: &str) -> Self {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        let split = line.split(":").collect::<Vec<_>>();
        let id = split[0].split(" ").skip(1).next().unwrap().parse::<usize>().unwrap();

        for list in split[1].split(";") {
            for num_color in list.split(",") {
                let s = num_color.trim().split(" ").collect::<Vec<_>>();
                let amount = s[0].parse::<usize>().unwrap();
                let color = s[1];

                match (color, amount) {
                    ("red", amount) if amount > max_red => max_red = amount,
                    ("blue", amount) if amount > max_blue => max_blue = amount,
                    ("green", amount) if amount > max_green => max_green = amount,
                    _ => {}
                }
            }
        }

        Game {
            id,
            max_red,
            max_blue,
            max_green,
        }
    }
}