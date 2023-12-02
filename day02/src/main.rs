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
                        .ok_or("missing")?
                        .strip_prefix("Game ")
                        .ok_or("missing")?
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

    let mut id_sum = 0;

    for game in games.iter() {
        if game
            .pulls
            .iter()
            .all(|pull| pull.iter().all(|(num, color)| *num <= max_colors[color]))
        {
            id_sum += game.id;
        }
    }

    dbg!(id_sum);

    let mut power_sum = 0;

    for game in games.iter() {
        let mut maxcolor: HashMap<Color, usize> = HashMap::new();
        for pull in game.pulls.iter() {
            for (num, color) in pull {
                let cur_max_num = *maxcolor.entry(*color).or_insert(0);
                if *num > cur_max_num {
                    maxcolor.insert(*color, *num);
                }
            }
        }
        power_sum += maxcolor.values().product::<usize>();
    }

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
