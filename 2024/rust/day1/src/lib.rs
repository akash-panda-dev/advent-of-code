use miette::Result;

pub mod part1 {
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<String> {
        Ok(input.lines().count().to_string())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_process() -> miette::Result<()> {
            let input = "\
97924   12015
50267   32019
98415   10716";
            assert_eq!("3", process(input)?);
            Ok(())
        }
    }
}

pub mod part2 {
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<String> {
        todo!("part 2");
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic(expected = "part 2")]
        fn test_process() {
            let input = "\
97924   12015
50267   32019
98415   10716";
            let _ = process(input);
        }
    }
}
