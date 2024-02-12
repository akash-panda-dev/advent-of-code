use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    process,
};

fn main() {
    let depths = read_input_lines("input/input.txt").unwrap_or_else(|err| {
        eprintln!("Failed to read the input file: {err}");
        process::exit(1);
    });

    let depths: Vec<usize> = depths
        .map(|depth| depth.unwrap().parse::<usize>().unwrap())
        .collect();

    println!(
        "Part 1 answer: {}",
        depths.windows(2).filter(|w| w[1] > w[0]).count()
    );

    println!("Part 2 answer: {}", analyze_windowed_depths(depths));
}

fn analyze_windowed_depths(depths: Vec<usize>) -> usize {
    let depth_groups: Vec<usize> = depths.windows(3).map(|w| w.iter().sum()).collect();
    depth_groups.windows(2).filter(|w| w[1] > w[0]).count()
}

fn read_input_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
