use clap::{Arg, Command};
use std::io::Read;

pub mod day1;

fn cli() -> Command {
    Command::new("aoc2023-rust")
        .version("0.1.0")
        .author("Gunnsteinn Þórisson <gussi@gussi.is>")
        .about("Solutions to Advent of Code 2023")
        .subcommand(
            Command::new("solve")
                .about("Solve a problem")
                .arg(
                    Arg::new("day")
                        .help("The day to solve")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("part")
                        .help("The part to solve")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            Command::new("download")
                .about("Download input for a day")
                .arg(
                    Arg::new("day")
                        .help("The day to download input for")
                        .required(true)
                        .index(1),
                ),
        )
}

fn main() {
    match cli().get_matches().subcommand() {
        Some(("solve", sync_matches)) => {
            let day = sync_matches.get_one::<String>("day").unwrap();
            let part = sync_matches.get_one::<String>("part").unwrap();

            solve(day.parse().unwrap(), part.parse().unwrap());
        }
        Some(("download", sync_matches)) => {
            let day = sync_matches.get_one::<String>("day").unwrap();
            let key = std::env::var("AOC_SESSION_KEY");

            if key.is_err() {
                println!("AOC_SESSION_KEY environment variable not set");
            }

            download(day.parse().unwrap(), key.unwrap());
        }
        _ => {
            cli().print_help().unwrap();
        }
    }
}

fn solve(day: u8, part: u8) {
    println!("Solving day {} part {}", day, part);

    match day {
        1 => match part {
            1 => crate::day1::part1::main(),
            2 => crate::day1::part2::main(),
            _ => println!("Part {} not found for day {}", part, day),
        },
        _ => println!("Day {} not found", day),
    }
}

fn download(day: u8, key: String) {
    println!("Downloading input for day {}", day);

    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    let cookie = format!("session={}", key);
    let output = format!("input/day{}.txt", day);

    let mut cmd = std::process::Command::new("curl");
    cmd.arg("-H");
    cmd.arg(format!("Cookie: {}", cookie));
    cmd.arg(url);
    cmd.arg("-o");
    cmd.arg(output.clone());
    cmd.output().expect("Failed to execute curl");

    println!("Downloaded input to {}", output);
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    input
}
