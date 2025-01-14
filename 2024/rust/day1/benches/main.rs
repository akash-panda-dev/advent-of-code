use day1::*;
use divan::black_box;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() {
    part1::process(black_box(include_str!("../input.txt"))).unwrap();
}

#[divan::bench]
fn bench_part2() {
    part2::process(black_box(include_str!("../input.txt"))).unwrap();
}

