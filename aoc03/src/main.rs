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
    let values: Vec<&str> = input.lines().map(|l| l.trim()).collect();

    let first = &values[0];
    // let mut gamma = vec![];
    // let mut epsilon = vec![];
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for col in 0..first.len() {
        let mut ones = 0;
        let mut zeros = 0;
        for v in &values {
            if v.chars().nth(col).unwrap() == '0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }

        if zeros > ones {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    let g = usize::from_str_radix(gamma.trim(), 2).unwrap();
    let e = usize::from_str_radix(epsilon.trim(), 2).unwrap();
    let result = g * e;
    println!("{}", result);

    Ok(())
}

enum Common {
    Zero,
    One,
}

fn find_common(values: &[&str], col: usize, tie: char) -> (char, bool) {
    let mut ones = 0;
    let mut zeros = 0;
    for v in values {
        if v.chars().nth(col).unwrap() == '0' {
            zeros += 1;
        } else {
            ones += 1;
        }
    }

    if zeros == ones {
        (tie, true)
    } else if zeros > ones {
        ('0', false)
    } else {
        ('1', false)
    }
}

fn most_common(values: &[&str], col: usize, tie: char) -> (char, bool) {
    let common = find_common(values, col, tie);
    common
}

fn least_common(values: &[&str], col: usize, tie: char) -> (char, bool) {
    let (common, t) = find_common(values, col, tie);
    if common == '0' {
        ('1', t)
    } else {
        ('0', t)
    }
}

fn part2(input: &str) -> Result<()> {
    let mut values: Vec<&str> = input.lines().map(|l| l.trim()).collect();
    let first = values[0].to_string();

    let mut oxygen = String::new();
    let mut co2 = String::new();
    let mut o_values = values.clone();
    for col in 0..first.len() {
        let (common, tie) = most_common(&o_values, col, '1');
        let common = if tie { '1' } else { common };
        o_values = o_values
            .iter()
            .copied()
            .filter(|v| v.chars().nth(col).unwrap() == common)
            .collect();

        if o_values.len() == 1 {
            oxygen = o_values[0].to_string();
        }
    }

    println!();
    println!();

    let mut c_values = values;
    for col in 0..first.len() {
        let (common, tie) = least_common(&c_values, col, '0');
        let common = if tie { '0' } else { common };
        c_values = c_values
            .iter()
            .copied()
            .filter(|v| v.chars().nth(col).unwrap() == common)
            .collect();

        if c_values.len() == 1 {
            co2 = c_values[0].to_string();
        }
    }

    println!("{}", oxygen);
    println!("{}", co2);

    let o = usize::from_str_radix(&oxygen.trim(), 2).unwrap();
    let c = usize::from_str_radix(&co2.trim(), 2).unwrap();

    println!("{}", o);
    println!("{}", c);

    let result = o * c;
    println!("{}", result);

    Ok(())
}
