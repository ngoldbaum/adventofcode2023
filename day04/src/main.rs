use regex::Regex;
use std::collections::HashSet;
use std::error;
use std::fs::read_to_string;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = read_to_string("input")?;

    let re = Regex::new(r"Card\s+\d*: ((?:\d\d |\s\d )+)\| ((?:\d\d |\s\d )+(?:\d\d|\s\d))")?;

    let mut scoresum = 0;
    let caps_iter = re.captures_iter(&contents);

    let mut card_counts = vec![1; caps_iter.count()];

    let caps_iter = re.captures_iter(&contents);

    for (card_id, caps) in caps_iter.enumerate() {
        let winning_numbers: HashSet<usize> = HashSet::from_iter(
            (&caps[1])
                .split_whitespace()
                .map(|d| d.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .into_iter(),
        );
        let card_numbers = &caps[2]
            .split_whitespace()
            .map(|d| d.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let num_wins = card_numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .collect::<Vec<&usize>>()
            .len();
        if num_wins != 0 {
            scoresum += 2_usize.pow(num_wins as u32 - 1);
        }
        for i in (card_id + 1)..(card_id + num_wins + 1) {
            card_counts[i] += card_counts[card_id];
        }
    }

    dbg!(&card_counts);

    dbg!(scoresum);
    dbg!(card_counts.iter().sum::<usize>());

    Ok(())
}
