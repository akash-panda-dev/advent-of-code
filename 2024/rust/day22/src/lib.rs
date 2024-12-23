use miette::Result;

#[derive(Debug, Clone, Copy)]
struct SecretNumber(usize);

impl SecretNumber {
    fn next(self) -> Self {
        let step1 = self.mix(self.0 * 64).prune();
        let step2 = step1.mix(step1.0 / 32).prune();
        let step3 = step2.mix(step2.0 * 2048).prune();
        step3
    }

    fn mix(self, num: usize) -> Self {
        Self(self.0 ^ num)
    }

    fn prune(self) -> Self {
        Self(self.0 % 16777216)
    }
}

pub mod part1 {
    use super::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<usize> {
        let temp: usize = input
            .lines()
            .map(|line| {
                let init = SecretNumber(line.parse::<usize>().unwrap());
                dbg!(init);

                std::iter::successors(Some(init), |&s| Some(s.next()))
                    .nth(2000)
                    .unwrap()
                    .0
            })
            .sum();

        dbg!(temp);

        Ok(temp)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        //         #[test]
        //         fn test_process() -> Result<()> {
        //             let input = "\
        // 123";
        //             assert_eq!(37327623, process(input)?);
        //             Ok(())
        //         }

        #[test]
        fn test_process() -> Result<()> {
            let input = "\
1
10
100
2024";
            assert_eq!(37327623, process(input)?);
            Ok(())
        }
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
