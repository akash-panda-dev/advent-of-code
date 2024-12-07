fn parse(input: &str) -> impl Iterator<Item = Vec<u64>> + '_ {
    input.lines().map(|line| {
        line.split(|c: char| !c.is_ascii_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(|v| v.trim().parse().unwrap())
            .collect()
    })
}

// Not efficient
// Check from left to right
fn is_feasible(target: u64, curr: u64, nums: &[u64]) -> bool {
    match nums {
        [] if target == curr => true,
        [head, rest @ ..] => {
            is_feasible(target, curr * head, rest) || is_feasible(target, curr + head, rest)
        }
        _ => false,
    }
}

// Start from the end
// If the last is divisible then try going the * route
// if the target - last > 0 then try the + route too
fn is_feasible_rev(target: u64, nums: &[u64]) -> bool {
    match nums {
        [curr] if target == *curr => true,
        [rest @ .., tail] => {
            target % tail == 0 && is_feasible_rev(target / tail, rest)
                || target >= *tail && is_feasible_rev(target - tail, rest)
        }
        _ => false,
    }
}

pub mod part1 {
    use anyhow::Result;

    use crate::*;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        let input_iter = parse(input);

        let total_calib = input_iter
            .filter_map(|line| is_feasible(line[0], line[1], &line[2..]).then_some(line[0]))
            .sum();
        Ok(total_calib)
    }

    #[tracing::instrument]
    pub fn process_right_left(input: &str) -> Result<u64> {
        let input_iter = parse(input);

        let total_calib = input_iter
            .filter_map(|line| is_feasible_rev(line[0], &line[1..]).then_some(line[0]))
            .sum();
        Ok(total_calib)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use anyhow::Result;

        #[test]
        fn test_process() -> Result<()> {
            let input = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
            assert_eq!(3749, process(input)?);
            Ok(())
        }

        #[test]
        fn test_process_right_left() -> Result<()> {
            let input = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
            assert_eq!(3749, process_right_left(input)?);
            Ok(())
        }
    }
}

pub mod part2 {
    use anyhow::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        todo!("part 2");
    }
}
