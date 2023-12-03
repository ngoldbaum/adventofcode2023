use std::collections::HashMap;
use std::error;
use std::fs::read_to_string;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let max_colors = HashMap::from([(Color::RED, 12), (Color::GREEN, 13), (Color::BLUE, 14)]);

    let contents = read_to_string("input")?;

    let games = contents
        .trim()
        .split("\n")
        .map(|line| line.split(": "))
        .map(|mut iter| -> Result<Game> {
            Ok(Game {
                id: {
                    iter.next()
                        .get_or_insert("missing")
                        .strip_prefix("Game ")
                        .get_or_insert("missing")
                        .parse::<usize>()?
                },
                pulls: iter
                    .next()
                    .ok_or("missing")?
                    .split("; ")
                    .map(|pull| -> Result<Vec<(usize, Color)>> {
                        Ok(pull
                            .split(", ")
                            .map(|num_color| {
                                let mut iter = num_color.split(' ');
                                let num = iter.next().unwrap().parse::<usize>().unwrap();
                                let color = match iter.next().unwrap() {
                                    "red" => Color::RED,
                                    "green" => Color::GREEN,
                                    "blue" => Color::BLUE,
                                    _ => panic!(),
                                };
                                (num, color)
                            })
                            .collect::<Vec<(usize, Color)>>())
                    })
                    .collect::<Result<Vec<Vec<(usize, Color)>>>>()?,
            })
        })
        .collect::<Result<Vec<Game>>>()?;

    let id_sum = games.iter().fold(0, |mut id_sum, game| {
        if game
            .pulls
            .iter()
            .all(|pull| pull.iter().all(|(num, color)| *num <= max_colors[color]))
        {
            id_sum += game.id;
        }
        id_sum
    });

    dbg!(id_sum);

    let power_sum = games.iter().fold(0, |mut acc, game| {
        let maxcolor = game.pulls.iter().fold(HashMap::new(), |acc, pull| {
            pull.iter().fold(acc, |mut acc, (num, color)| {
                if *num > *acc.entry(color).or_insert(0) {
                    acc.insert(color, *num);
                }
                acc
            })
        });
        acc += maxcolor.values().product::<usize>();
        acc
    });

    dbg!(power_sum);

    Ok(())
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

#[derive(Debug)]
struct Game {
    id: usize,
    pulls: Vec<Vec<(usize, Color)>>,
}
