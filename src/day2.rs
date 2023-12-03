#[derive(Debug)]
#[derive(Clone)]
struct Game {
    id: usize,

    cubes: Vec<Cubes>,
}

#[derive(Debug)]
#[derive(Clone)]
struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

impl Cubes {

    fn within_limits(&self, limits: Cubes) -> bool {
        self.red <= limits.red
            && self.green <= limits.green
            && self.blue <= limits.blue
    }
}

impl Game {

    fn within_limits(&self, limits: Cubes) -> bool {
        self.cubes.iter().all(|cubes| cubes.within_limits(limits.clone()))
    }

    fn new_from_line(line: &str) -> Game {
        let mut parts = line.split(": ");

        let id = parts
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let cubes = parts
            .next()
            .unwrap()
            .split("; ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|data| {
                let mut cubes: Cubes = Cubes {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                data
                    .split(", ")
                    .collect::<Vec<&str>>()
                    .iter()
                    .for_each(|cube| {
                        let mut cube_parts = cube.split(" ");

                        let count = cube_parts.next().unwrap().parse::<usize>().unwrap();
                        let color = cube_parts.next().unwrap();

                        match color {
                            "red" => cubes.red = count,
                            "green" => cubes.green = count,
                            "blue" => cubes.blue = count,
                            _ => panic!("Unknown color"),
                        }
                    });

                cubes
            })
            .collect::<Vec<Cubes>>();

        Game {
            id,
            cubes,
        }
    }
}

pub mod part1 {

    use super::*;

    pub fn solve() -> String {
        let input = crate::read_input();

        let games: Vec<Game> = input
            .lines()
            .map(|line| {
                Game::new_from_line(line)
            }).collect();

        let cube_limits = Cubes {
            red: 12,
            green: 13,
            blue: 14,
        };

        let answer: usize = games
            .iter()
            .map(|game| {
                if game.within_limits(cube_limits.clone()) {
                    game.id
                } else {
                    0
                }
            })
            .sum();

        return format!("Answer: {}", answer);
    }
}

pub mod part2 {

    pub fn solve() -> String {
        return String::from("Note solved yet");
    }
}
