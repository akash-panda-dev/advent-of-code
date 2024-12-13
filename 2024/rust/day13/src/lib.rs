#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use miette::Result;
fn calculate_determinant(a: i64, b: i64, c: i64, d: i64) -> i64 {
    a * d - b * c
}

fn solve_linear_equation(coeffs: (CoeffPair, CoeffPair, CoeffPair)) -> Option<(i64, i64)> {
    // a1x + b1y = c1
    // a2x + b2y = c2
    let ((a1, a2), (b1, b2), (c1, c2)) = coeffs;
    let determinant = calculate_determinant(a1, b1, a2, b2);

    if determinant == 0 {
        return None;
    }

    let dx = calculate_determinant(c1, b1, c2, b2);
    let dy = calculate_determinant(a1, c1, a2, c2);

    // Check if the division would result in a fractional number
    if dx % determinant != 0 || dy % determinant != 0 {
        return None;
    }
    let x = dx / determinant;
    let y = dy / determinant;

    Some((x, y))
}

type CoeffPair = (i64, i64);

pub mod part1 {
    use super::*;
    use miette::Result;
    use regex::Regex;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<i64> {
        let re = Regex::new(r"X[+=](\d+).*Y[+=](\d+)").unwrap();
        let coeffs: Vec<(CoeffPair, CoeffPair, CoeffPair)> = input
            .split("\n\n")
            .map(|eqs| {
                let nums: Vec<CoeffPair> = eqs
                    .lines()
                    .map(|l| re.captures(l).unwrap())
                    .map(|c| (c[1].parse::<i64>().unwrap(), c[2].parse::<i64>().unwrap()))
                    .collect();
                (nums[0], nums[1], nums[2])
            })
            .collect();

        let tokens = coeffs
            .iter()
            .filter_map(|&c| solve_linear_equation(c))
            .map(|(a, b)| a * 3 + b)
            .sum();

        Ok(tokens)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_process() -> Result<()> {
            let input = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

            assert_eq!(480, process(input)?);
            Ok(())
        }
    }
}

pub mod part2 {
    use super::*;
    use miette::Result;
    use regex::Regex;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<i64> {
        let re = Regex::new(r"X[+=](\d+).*Y[+=](\d+)").unwrap();
        let coeffs: Vec<(CoeffPair, CoeffPair, CoeffPair)> = input
            .split("\n\n")
            .map(|eqs| {
                let nums: Vec<CoeffPair> = eqs
                    .lines()
                    .map(|l| re.captures(l).unwrap())
                    .enumerate()
                    .map(|(i, c)| {
                        let c1 = c[1].parse::<i64>().unwrap();
                        let c2 = c[2].parse::<i64>().unwrap();
                        if i == 2 {
                            return (10000000000000 + c1, 10000000000000 + c2);
                        } else {
                            return (c1, c2);
                        }
                    })
                    .collect();
                (nums[0], nums[1], nums[2])
            })
            .collect();

        let tokens = coeffs
            .iter()
            .filter_map(|&c| solve_linear_equation(c))
            .map(|(a, b)| a * 3 + b)
            .sum();

        Ok(tokens)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    }
}
