use std::error;
use std::fs::read_to_string;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = read_to_string("input")?;

    let result: u32 = contents
        .trim()
        .split("\n")
        .map(|lines| find_digits(lines))
        .map(|x| {
            x.into_iter()
                .map(|c| char::from_digit(c, 10).unwrap())
                .collect::<Vec<char>>()
        })
        .map(|x| {[&x[0], x.last().unwrap()].into_iter().collect::<String>()})
        .map(|x| x.parse::<u32>().unwrap())
        .sum();

    dbg!(result);

    Ok(())
}

fn find_digits(line: &str) -> Vec<u32> {
    let mut line_contents: String = line.to_string();
    let mut result: Vec<u32> = Vec::new();
    let digits: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].into_iter().collect();
    while line_contents.len() != 0 {
        let ch = line_contents.chars().next().unwrap();
        let d: u32 = match ch.to_digit(10) {
            Some(d) => d,
            None => {
                let mut ret: u32 = 0;
                for (digit, value) in digits.iter() {
                    if line_contents.starts_with(digit) {
                        ret = *value;
                        break;
                    }
                }
                ret
            }
        };
        if d != 0 {
            result.push(d);
        }
        let mut chars = line_contents.chars();
        chars.next();
        line_contents = chars.as_str().to_string();
    }
    result
}
