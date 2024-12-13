use itertools::Itertools;
use rayon::prelude::*;

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
fn is_feasible(target: u64, curr: u64, nums: &[u64], ops: &[Op]) -> bool {
    match nums {
        [] if target == curr => true,
        [head, rest @ ..] => ops.iter().any(|op| match op {
            Op::Mul => is_feasible(target, curr * head, rest, ops),
            Op::Add => is_feasible(target, curr + head, rest, ops),
            Op::Concat => is_feasible(target, format!("{curr}{head}").parse().unwrap(), rest, ops),
            _ => panic!("No other op supported by fn"),
        }),
        _ => false,
    }
}

fn is_feasible_cartesian(target: u64, nums: &[u64], ops: &[Op]) -> bool {
    (0..nums.len() - 1)
        .map(|_| ops)
        .multi_cartesian_product()
        .any(|seq| {
            let mut s = seq.iter();

            let result = nums
                .iter()
                .copied()
                .reduce(|acc, next| match s.next().unwrap() {
                    Op::Mul => acc * next,
                    Op::Add => acc + next,
                    _ => panic!("not supported"),
                })
                .unwrap();

            result == target
        })
}

/// Efficiently checks if target can be reached using the given numbers and operations,
/// processing right-to-left for better performance.
///
/// More efficient than left-to-right because:
/// - Multiplication: Immediately reject if target isn't divisible by current number
/// - Addition: Immediately reject if target is smaller than current number
/// - Masking (string concat): Immediately reject if target's last digits don't match
fn is_feasible_rev(target: u64, nums: &[u64], ops: &[Op]) -> bool {
    match nums {
        [curr] if target == *curr => true,
        [rest @ .., tail] => ops.iter().any(|op match op {
            Op::Mul => target % tail == 0 && is_feasible_rev(target / tail, rest, ops),
            Op::Add => target >= *tail && is_feasible_rev(target - tail, rest, ops),
            Op::Mask => {
                // Example:
                // 178 : 17 || 8
                // We need to first check if 178 ends with 8 (aka last)
                // To do that we're masking it with a 10 power
                // Then we divide by the mask to set the new target as the first half i.e 17
                let mask = 10_u64.pow(tail.ilog10() + 1);
                target % mask == *tail && is_feasible_rev(target / mask, rest, ops)
            }
            _ => panic!("No other op supported for this"),
        }),
        _ => false,
    }
}

#[derive(Clone)]
enum Op {
    Mul,
    Add,
    Concat,
    Mask,
}

pub mod part1 {
    use anyhow::Result;

    use crate::*;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        let input_iter = parse(input);
        let ops = &[Op::Add, Op::Mul];

        // let total_calib = input_iter
        //     .filter_map(|line| is_feasible(line[0], line[1], &line[2..], ops).then_some(line[0]))
        //     .sum();
        let total_calib = input_iter
            .filter_map(|line| is_feasible_cartesian(line[0], &line[1..], ops).then_some(line[0]))
            .sum();
        Ok(total_calib)
    }

    #[tracing::instrument]
    pub fn process_right_left(input: &str) -> Result<u64> {
        let input_iter = parse(input);
        let ops = &[Op::Add, Op::Mul];

        let total_calib = input_iter
            .filter_map(|line| is_feasible_rev(line[0], &line[1..], ops).then_some(line[0]))
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
    use crate::*;
    use anyhow::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        let input_iter = parse(input);
        let ops = &[Op::Add, Op::Mul, Op::Concat];

        let total_calib = input_iter
            .filter_map(|line| {
                is_feasible(
                    line[0],    // calibration
                    line[1],    // first operand
                    &line[2..], // rest of the opeerands
                    ops,
                )
                .then_some(line[0])
            })
            .sum();

        Ok(total_calib)
    }

    #[tracing::instrument]
    pub fn process_right_left(input: &str) -> Result<u64> {
        let input_iter = parse(input);
        let ops = &[Op::Add, Op::Mul, Op::Mask];

        let total_calib = input_iter
            .filter_map(|line| is_feasible_rev(line[0], &line[1..], ops).then_some(line[0]))
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
            assert_eq!(11387, process(input)?);
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
            assert_eq!(11387, process_right_left(input)?);
            Ok(())
        }
    }
}
