use adventofcode_2023::get_files_lines;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;


#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Element {
}

fn main() {
    println!("{}", part_one("./aoc-inputs/2023/day10sample.txt"));
    println!("{}", part_one("./aoc-inputs/2023/day10sample2.txt"));
    // println!("{}", part_one("./aoc-inputs/day10simplesample2.txt"));
    // println!("{}", part_one("./aoc-inputs/day10simplesample.txt"));
}


fn part_one(file_path: &str) -> usize {
    let lines = get_files_lines(file_path);
    let grid: Vec<Vec<_>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    0
}


fn part_two() {}
