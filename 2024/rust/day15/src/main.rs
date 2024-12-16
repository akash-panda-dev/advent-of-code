use day15::part1;
use day15::part2;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    // Only initialize if we're not benchmarking
    if std::env::var("BENCH").is_err() {
        tracing_subscriber::fmt::init();
    }

    let part = std::env::args()
        .nth(1)
        .expect("Expected part number as argument");

    match part.as_str() {
        "1" => {
            let file = include_str!("../input.txt");
            let result = part1::process(file).context("process part 1")?;
            println!("{}", result);
        }
        "2" => {
            let file = include_str!("../input.txt");
            let result = part2::process(file).context("process part 2")?;
            println!("{}", result);
        }
        _ => panic!("Invalid part number: {}", part),
    }

    Ok(())
}