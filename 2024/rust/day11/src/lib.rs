use std::collections::HashMap;

use miette::Result;

fn count_digits(n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    n.ilog10() + 1
}

// What if I just noted the number of even digit stones.
fn blink(stone_freq: HashMap<u64, u64>) -> HashMap<u64, u64> {
    stone_freq
        .iter()
        .fold(HashMap::new(), |mut acc, (&stone, freq)| match stone {
            0 => {
                *acc.entry(1).or_default() += freq;
                acc
            }
            s if count_digits(s) % 2 == 0 => {
                let c = count_digits(s);
                let mask = 10u64.pow(c / 2);
                [s / mask, s % mask]
                    .iter()
                    .for_each(|s| *acc.entry(*s).or_default() += freq);

                acc
            }
            _ => {
                *acc.entry(stone * 2024).or_default() += freq;
                acc
            }
        })
}

pub mod part1 {
    use std::collections::HashSet;

    use crate::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        let mut stones: Vec<u64> = input
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let mut stones_freq: HashMap<u64, u64> =
            stones.iter().fold(HashMap::new(), |mut acc, stone| {
                *acc.entry(*stone).or_default() += 1;
                acc
            });

        for i in 0..75 {
            stones_freq = blink(stones_freq);
        }

        Ok(stones_freq.values().sum())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    }
}

pub mod part2 {
    use super::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        todo!("part 2");
    }
}
