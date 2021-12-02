use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

enum Commands {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse(input: &str) -> Result<Vec<Commands>> {
    let commands: Vec<Commands> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let val = parts[1].parse().expect("expected number");
            match parts[0] {
                "forward" => Commands::Forward(val),
                "down" => Commands::Down(val),
                "up" => Commands::Up(val),
                _ => panic!("Unexpected command: {}", parts[0]),
            }
        })
        .collect();

    Ok(commands)
}

fn part1(input: &str) -> Result<()> {
    let commands = parse(input)?;
    let mut x = 0;
    let mut y = 0;

    for c in commands {
        match c {
            Commands::Forward(v) => {
                x += v;
            }
            Commands::Down(v) => {
                y += v;
            }
            Commands::Up(v) => {
                y -= v;
            }
        }
    }

    let result = x * y;
    println!("{}", result);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let commands = parse(input)?;
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for c in commands {
        match c {
            Commands::Forward(v) => {
                x += v;
                y += aim * v;
            }
            Commands::Down(v) => {
                aim += v;
            }
            Commands::Up(v) => {
                aim -= v;
            }
        }
    }

    let result = x * y;
    println!("{}", result);

    Ok(())
}
