use itermore::IterMore;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let values: Vec<i32> = input
        .lines()
        .map(|l| l.parse().expect("expected a number"))
        .collect();

    let mut increases = 0;
    let mut prev = values[0];
    for (_i, item) in values.iter().skip(1).enumerate() {
        if *item > prev {
            increases += 1;
        }

        prev = *item;
    }

    println!("{}", increases);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let values: Vec<i32> = input
        .lines()
        .map(|l| l.parse().expect("expected a number"))
        .collect();

    let mut increases = 0;
    let mut prev_window = values[0] + values[1] + values[2];
    for [a, b, c] in values.iter().skip(1).windows() {
        let window_sum = a + b + c;
        if window_sum > prev_window {
            increases += 1;
        }

        prev_window = window_sum;
    }

    println!("{}", increases);

    Ok(())
}
