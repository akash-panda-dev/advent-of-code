#[derive(Debug, Copy, Clone)]
struct CPU {
    r_a: i64,
    r_b: i64,
    r_c: i64,
}

impl CPU {
    fn get_combo_operand(&self, operand: u8) -> i64 {
        match operand {
            0..=3 => operand as i64,
            4 => self.r_a,
            5 => self.r_b,
            6 => self.r_c,
            _ => panic!("Invalid operand. No combo exists"),
        }
    }

    fn execute(&mut self, instrs: Vec<Instr>) -> Vec<u8> {
        let mut stdout: Vec<u8> = vec![];
        let mut pointer: usize = 0;

        while pointer + 1 < instrs.len() {
            let opcode = instrs[pointer].0;
            let operand = instrs[pointer + 1].0;
            let mut jump: bool = false;
            match opcode {
                0 => {
                    let result =
                        self.r_a as f64 / 2_i64.pow(self.get_combo_operand(operand) as u32) as f64;
                    self.r_a = result.trunc() as i64;
                }
                1 => {
                    self.r_b = self.r_b ^ instrs[pointer + 1].0 as i64;
                }
                2 => {
                    self.r_b = self.get_combo_operand(operand) % 8;
                }
                3 => {
                    if self.r_a != 0 {
                        jump = true;
                    }
                }
                4 => {
                    self.r_b = self.r_b ^ self.r_c;
                }
                5 => stdout.push((self.get_combo_operand(operand) % 8) as u8),
                6 => {
                    let result =
                        self.r_a as f64 / 2_i64.pow(self.get_combo_operand(operand) as u32) as f64;
                    self.r_b = result.trunc() as i64;
                }
                7 => {
                    let result =
                        self.r_a as f64 / 2_i64.pow(self.get_combo_operand(operand) as u32) as f64;
                    self.r_c = result.trunc() as i64;
                }
                _ => unreachable!("Impossible instr"),
            }

            if jump {
                pointer = operand as usize;
            } else {
                pointer += 2;
            }

            // println!("CPU r_a: {}", self.r_a);
        }

        stdout
    }
}

#[derive(Debug, Copy, Clone)]
struct Instr(u8);

impl Instr {
    fn new(value: u8) -> Self {
        if value < 8 {
            Instr(value)
        } else {
            panic!("Instr cannot be more than a 3 bit number");
        }
    }
}

fn parse(input: &str) -> (CPU, Vec<Instr>) {
    let mut parts = input.split("\n\n");
    let registers: Vec<i64> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .trim()
                .parse::<i64>()
                .unwrap()
        })
        .collect();

    let instrs: Vec<Instr> = parts
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(',')
        .map(|i| Instr::new(i.parse::<u8>().unwrap()))
        .collect();

    (
        CPU {
            r_a: registers[0],
            r_b: registers[1],
            r_c: registers[2],
        },
        instrs,
    )
}

pub mod part1 {
    use std::usize;

    use super::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<usize> {
        // Min is a number when divided by 8 gives  5 digits

        let (mut cpu, program) = parse(&input);
        for a in 0..=2000 {
            cpu.r_a = a;
            let stdout = cpu.execute(program.clone());
            let result = stdout
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
            println!("r_a: {}, binary r_ad: {:b}, stdout: {}", a, a, result);
        }

        // cpu.r_a = a;
        // let stdout = cpu.execute(program.clone());
        // let result = stdout
        //     .iter()
        //     .map(|x| x.to_string())
        //     .collect::<Vec<String>>()
        //     .join(",");
        // println!("r_a: {}, cpu_ra: {}, stdout: {}", a, cpu.r_a, result);

        Ok(program
            .iter()
            .enumerate()
            .map(|(i, instr)| instr.0 as i32 * 8_i32.pow(i as u32 + 1))
            .sum::<i32>() as usize)

        // Ok(stdout
        //     .iter()
        //     .map(|x| x.to_string())
        //     .collect::<Vec<String>>()
        //     .join(","))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        //         #[test]
        //         fn test_process() -> Result<()> {
        //             let input = "\
        // Register A: 729
        // Register B: 0
        // Register C: 0
        //
        // Program: 0,1,5,4,3,0";
        //             assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input)?);
        //             Ok(())
        //         }
        //         #[test]
        //         fn test_process2() -> Result<()> {
        //             let input = "\
        // Register A: 2024
        // Register B: 0
        // Register C: 0
        //
        // Program: 0,3,5,4,3,0";
        //
        //             assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input)?);
        //             Ok(())
        //         }
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
