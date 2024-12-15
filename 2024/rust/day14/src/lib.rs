#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use nom::{
    bytes::complete::tag,
    character::complete::{char, i32},
    multi::separated_list0,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

const MAX_X: i32 = 101;
const MAX_Y: i32 = 103;

enum Quadrant {
    Q1,
    Q2,
    Q3,
    Q4,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

impl Robot {
    fn move_next(self) -> Self {
        let pos = Vec2 {
            x: (self.pos.x + self.vel.x).rem_euclid(MAX_X),
            y: (self.pos.y + self.vel.y).rem_euclid(MAX_Y),
        };
        Robot { pos, vel: self.vel }
    }

    fn get_quadrant(&self) -> Option<Quadrant> {
        match (self.pos.x, self.pos.y) {
            (x, y) if x < MAX_X / 2 && y < MAX_Y / 2 => Some(Quadrant::Q1),
            (x, y) if x > MAX_X / 2 && y < MAX_Y / 2 => Some(Quadrant::Q2),
            (x, y) if x < MAX_X / 2 && y > MAX_Y / 2 => Some(Quadrant::Q3),
            (x, y) if x > MAX_X / 2 && y > MAX_Y / 2 => Some(Quadrant::Q4),
            (_, _) => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

fn robot(input: &str) -> IResult<&str, Robot> {
    let (input, ((px, py), (vx, vy))) = tuple((
        preceded(tag("p="), separated_pair(i32, char(','), i32)),
        preceded(tag(" v="), separated_pair(i32, char(','), i32)),
    ))(input)?;

    Ok((
        input,
        Robot {
            pos: Vec2 { x: px, y: py },
            vel: Vec2 { x: vx, y: vy },
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list0(char('\n'), robot)(input)
}

pub mod part1 {
    use std::iter::successors;

    use super::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        let robots: Vec<Robot> = parse(input).unwrap().1;

        let robots: Vec<Robot> = robots
            .into_iter()
            .map(|robot| {
                successors(Some(robot), |r| Some(r.move_next()))
                    .nth(100)
                    .unwrap()
            })
            .collect();

        let safety_factor = robots.into_iter().filter_map(|r| r.get_quadrant()).fold(
            (0, 0, 0, 0),
            |(q1, q2, q3, q4), q| {
                let (mut q1n, mut q2n, mut q3n, mut q4n) = (q1, q2, q3, q4);
                match q {
                    Quadrant::Q1 => q1n += 1,
                    Quadrant::Q2 => q2n += 1,
                    Quadrant::Q3 => q3n += 1,
                    Quadrant::Q4 => q4n += 1,
                };

                (q1n, q2n, q3n, q4n)
            },
        );

        let safety_factor = safety_factor.0 * safety_factor.1 * safety_factor.2 * safety_factor.3;

        Ok(safety_factor)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        //         #[test]
        //         fn test_process() -> Result<()> {
        //             let input = "\
        // p=0,4 v=3,-3
        // p=6,3 v=-1,-3
        // p=10,3 v=-1,2
        // p=2,0 v=2,-1
        // p=0,0 v=1,3
        // p=3,0 v=-2,-2
        // p=7,6 v=-1,-3
        // p=3,0 v=-1,-2
        // p=9,3 v=2,3
        // p=7,3 v=-1,2
        // p=2,4 v=2,-3
        // p=9,5 v=-3,-3";
        //
        //             assert_eq!(12, process(input)?);
        //             Ok(())
        //         }

        //         #[test]
        //         fn test_process() -> Result<()> {
        //             let input = "\
        // p=2,4 v=2,-3";
        //
        //             assert_eq!(480, process(input)?);
        //             Ok(())
        //         }
    }
}

pub mod part2 {
    use std::io::{self, Read, Write};
    use std::{fmt, iter::successors, thread, time::Duration};

    use super::*;
    use miette::Result;

    fn print_grid(robots: &Vec<Robot>) {
        // print!("\x1B[2J\x1B[1;1H");
        let mut grid = [['.'; MAX_X as usize]; MAX_Y as usize];

        for (i, robot) in robots.iter().enumerate() {
            let x = robot.pos.x as usize;
            let y = robot.pos.y as usize;
            grid[y][x] = '*';
        }

        println!("┌{}┐", "─".repeat(MAX_X as usize));
        for row in grid {
            print!("│");
            for cell in row {
                print!("{}", cell);
            }
            println!("│");
        }
        println!("└{}┘", "─".repeat(MAX_X as usize));
    }

    // #[tracing::instrument]
    // pub fn process(input: &str) -> Result<u64> {
    //     let mut robots: Vec<Robot> = parse(input).unwrap().1;
    //
    //     print_grid(&robots);
    //     let mut second = 0;
    //
    //     loop {
    //         println!("Press anykey to move robots");
    //         io::stdout().flush().unwrap();
    //
    //         io::stdin().read_exact(&mut [0u8; 1]).unwrap();
    //
    //         robots = robots.iter().map(|r| r.move_next()).collect();
    //         second += 1;
    //
    //         println!("{} second", second);
    //         print_grid(&robots);
    //     }
    // }

    pub fn process(input: &str) -> Result<u64> {
        let mut robots = parse(input).unwrap().1;
        println!("{}", robots.len());

        // Display initial state
        print_grid(&robots);
        thread::sleep(Duration::from_millis(500));

        let mut x_display = 23;
        let mut y_display = 89;
        let mut variance = i32::MAX;
        let mut min_var_step = 0;

        for step in 1..=10_000_000 {
            robots = robots.iter().map(|r| r.move_next()).collect();

            if step == 7093 {
                print_grid(&robots);
            }

            if step == x_display || step == y_display {
                // println!("{} second", step);
                // print_grid(&robots);
                // thread::sleep(Duration::from_millis(700));

                if step == x_display {
                    x_display = x_display + MAX_X;
                }
                if step == y_display {
                    y_display = y_display + MAX_Y;
                }

                let var = calculate_variance(&robots);
                if var < variance {
                    variance = var;
                    min_var_step = step;
                }

                // println!("Press anykey to continue");
                // io::stdout().flush().unwrap();
                //
                // io::stdin().read_exact(&mut [0u8; 1]).unwrap();
            }
        }

        dbg!(variance);
        dbg!(min_var_step);

        Ok(0)
    }

    fn calculate_variance(robots: &[Robot]) -> i32 {
        let n: i32 = robots.len() as i32;

        // Calculate means
        let mean_x: i32 = robots.iter().map(|p| p.pos.x).sum::<i32>() / n;
        let mean_y: i32 = robots.iter().map(|p| p.pos.y).sum::<i32>() / n;

        // Calculate variances
        let var_x = robots
            .iter()
            .map(|p| (p.pos.x - mean_x).pow(2))
            .sum::<i32>()
            / n;

        let var_y = robots
            .iter()
            .map(|p| (p.pos.y - mean_y).pow(2))
            .sum::<i32>()
            / n;

        var_x + var_y
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_process() -> Result<()> {
            let input = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

            assert_eq!(12, process(input)?);
            Ok(())
        }
    }
}
