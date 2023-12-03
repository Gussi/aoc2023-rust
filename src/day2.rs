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

    fn get_power(&self) -> usize {
        self.red * self.green * self.blue
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

    fn get_max_for_each_color(&self) -> Cubes {
        let mut max: Cubes = Cubes {
            red: 0,
            green: 0,
            blue: 0,
        };

        self.cubes.iter().for_each(|cubes| {
            if cubes.red > max.red {
                max.red = cubes.red;
            }

            if cubes.green > max.green {
                max.green = cubes.green;
            }

            if cubes.blue > max.blue {
                max.blue = cubes.blue;
            }
        });

        return max;
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

    use super::*;

    pub fn solve() -> String {
        let input = crate::read_input();

        let games: Vec<Game> = input
            .lines()
            .map(|line| {
                Game::new_from_line(line)
            }).collect();

        let answer: usize = games.iter().map(|game| {
            game.get_max_for_each_color().get_power()
        }).sum();

        return format!("Answer: {}", answer);
    }
}
