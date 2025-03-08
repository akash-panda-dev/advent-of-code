pub mod part1 {

    use anyhow::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        let sum = input
            .lines()
            .try_fold(0u32, |acc, l| -> Result<u32> {
                let mass = l.trim().parse::<u32>()?;
                Ok(acc + ((mass / 3) - 2))
            })?;
        
        Ok(sum as u64)
    }
}

pub mod part2 {
    use anyhow::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        let sum = input
            .lines()
            .try_fold(0u32, |acc, l| -> Result<u32> {
                let mass = l.trim().parse::<i32>()?;
                Ok(acc + calculate_fuel(mass))
            })?;
        
        Ok(sum as u64)
    }

    fn calculate_fuel(mass: i32) -> u32 {
        let fuel = (mass / 3) - 2;

        if fuel <= 0 {
            0
        } else {
            fuel as u32 + calculate_fuel(fuel)
        }
    }
}
