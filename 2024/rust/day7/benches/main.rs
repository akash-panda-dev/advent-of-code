use day7::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() {
    part1::process(include_str!("../input.txt")).unwrap();
}

#[divan::bench]
fn bench_part1_optimized() {
    part1::process_right_left(include_str!("../input.txt")).unwrap();
}

#[divan::bench]
fn bench_part1_optimized_blackbox() {
    divan::black_box(part1::process_right_left(include_str!("../input.txt")).unwrap());
}

#[divan::bench]
fn bench_part2() {
    part2::process(include_str!("../input.txt")).unwrap();
}
