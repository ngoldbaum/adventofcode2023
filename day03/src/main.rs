use std::collections::HashMap;
use std::error;
use std::fs::read_to_string;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = read_to_string("input")?;

    let map: Vec<Vec<char>> = contents
        .trim()
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let xmin = 0;
    let xmax = map[0].len();
    let ymin = 0;
    let ymax = map.len();

    let mut value_coords: HashMap<(usize, usize), usize> = HashMap::new();
    let mut part_coords: HashMap<(usize, usize), Part> = HashMap::new();
    let mut values: Vec<Value> = Vec::new();
    let mut parts: Vec<Part> = Vec::new();
    let mut part_values: HashMap<Part, Vec<usize>> = HashMap::new();

    for (y, line) in map.iter().enumerate() {
        for (x, symbol) in line.iter().enumerate() {
            if value_coords.contains_key(&(x, y)) {
                continue;
            } else if symbol.is_digit(10) {
                // handle new part number
                let (coords, value) = read_part_number(line, x);
                for coord in coords.iter() {
                    value_coords.insert((*coord, y), value);
                }
                let coords = coords
                    .iter()
                    .map(|c| (*c, y))
                    .collect::<Vec<(usize, usize)>>();
                values.push(Value {
                    value: value,
                    coords: coords,
                    part: None,
                });
            } else if *symbol != '.' {
                let p: Part = Part {
                    coord: (x, y),
                    symbol: *symbol,
                };
                part_coords.insert((x, y), p.clone());
                part_values.insert(p.clone(), Vec::new());
                parts.push(p)
            }
        }
    }

    for value in &mut values {
        'outer: for (xv, yv) in &value.coords {
            for x in clamp(*xv as i64 - 1, xmin, xmax)..clamp(*xv as i64 + 2, xmin, xmax) {
                for y in clamp(*yv as i64 - 1, ymin, ymax)..clamp(*yv as i64 + 2, ymin, ymax) {
                    match part_coords.get(&(x, y)) {
                        Some(part) => {
                            match part_values.get_mut(part) {
                                Some(pv) => {
                                    pv.push(value.value);
                                }
                                None => {}
                            }
                            value.part = Some(part.clone());
                            break 'outer;
                        }
                        None => {}
                    }
                }
            }
        }
    }

    let mut part_number_sum = 0;

    for value in &values {
        match value.part {
            Some(_) => part_number_sum += value.value,
            None => {}
        }
    }
    dbg!(part_number_sum);

    dbg!(parts
        .into_iter()
        .filter(|p| p.symbol == '*' && part_values[p].len() == 2)
        .map(|p| part_values[&p].iter().product::<usize>())
        .sum::<usize>());

    Ok(())
}

#[derive(Debug)]
struct Value {
    value: usize,
    coords: Vec<(usize, usize)>,
    part: Option<Part>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Part {
    symbol: char,
    coord: (usize, usize),
}

fn read_part_number(line: &Vec<char>, index: usize) -> (Vec<usize>, usize) {
    let mut coords: Vec<usize> = Vec::new();
    for i in index..line.len() {
        if !line[i].is_digit(10) {
            break;
        }
        coords.push(i);
    }

    let value: usize = coords
        .iter()
        .map(|c| line[*c])
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    (coords, value)
}

fn clamp(v: i64, min: usize, max: usize) -> usize {
    if v < min as i64 {
        return min;
    } else if v > max as i64 {
        return max;
    }
    v as usize
}
