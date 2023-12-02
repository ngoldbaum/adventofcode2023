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
        .map(|line| line.split(": ").collect::<Vec<&str>>())
        .map(|line_vec| {
            let mut iter = line_vec.iter();
            Game {
                id: {
                    iter.next()
                        .unwrap()
                        .strip_prefix("Game ")
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                },
                pulls: iter
                    .next()
                    .unwrap()
                    .split("; ")
                    .map(|pull| {
                        pull.split(", ")
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
                            .collect::<Vec<(usize, Color)>>()
                    })
                    .collect::<Vec<Vec<(usize, Color)>>>(),
            }
        })
        .collect::<Vec<Game>>();

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
        let power: usize = maxcolor
            .values()
            .collect::<Vec<&usize>>()
            .into_iter()
            .product();
        power_sum += power;
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
