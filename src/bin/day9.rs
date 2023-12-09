use adventofcode_2023::get_files_lines;
// use core::ops::Range;
// use std::fmt::Display;
// use std::str::FromStr;

fn main() {
    println!("Part One: {}", part_one("./aoc-inputs/2023/day9.txt"));
    println!("Part Two: {}", part_two("./aoc-inputs/2023/day9.txt"));
}

fn differentiate(func: &Vec<isize>) -> Vec<isize> {
    let mut derivative: Vec<isize> = vec![];
    for i in 1..func.len() {
        derivative.push(func[i] - func[i - 1]);
    }
    derivative
}
fn part_one(file_path: &str) -> isize {
    let lines: Vec<_> = get_files_lines(file_path);

    let nums: Vec<Vec<isize>> = lines
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|num| num.parse::<isize>().unwrap())
                .collect()
        })
        .collect();
    let mut collector = 0;

    for num_set in nums.iter() {
        let mut derivates: Vec<Vec<isize>> = vec![];
        derivates.push(num_set.to_vec());
        let mut current_derivative = num_set;

        while !current_derivative.iter().all(|x| x == &0) {
            let next_derivative = differentiate(current_derivative);
            derivates.push(next_derivative);
            current_derivative = &derivates.last().unwrap();
        }
        derivates.reverse();

        let mut adder = 0;
        for derivative_set in derivates.iter_mut() {
            adder += derivative_set.last().unwrap();
            derivative_set.push(adder);
        }

        collector += derivates.last().unwrap().last().unwrap();
    }
    collector
}
fn part_two(file_path: &str) -> isize {
    let lines: Vec<_> = get_files_lines(file_path);

    let nums: Vec<Vec<isize>> = lines
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|num| num.parse::<isize>().unwrap())
                .collect()
        })
        .collect();
    let mut collector = 0;

    for num_set in nums.iter() {
        let mut derivates: Vec<Vec<isize>> = vec![];
        derivates.push(num_set.to_vec());
        let mut current_derivative = num_set;

        while !current_derivative.iter().all(|x| x == &0) {
            let next_derivative = differentiate(current_derivative);
            derivates.push(next_derivative);
            current_derivative = &derivates.last().unwrap();
        }
        derivates.reverse();

        let mut adder = 0;
        for derivative_set in derivates.iter_mut() {
            derivative_set.reverse();
            let last_val = derivative_set.last().unwrap();
            adder = last_val - adder;
            derivative_set.push(adder);
        }

        collector += derivates.last().unwrap().last().unwrap();
    }
    collector
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_is_correct() {
        assert_eq!(114, part_one("./aoc-inputs/2023/day9sample.txt"));
        assert_eq!(2098530125, part_one("./aoc-inputs/2023/day9.txt"));
    }

    #[test]
    fn part_two_is_correct() {
        assert_eq!(2, part_two("./aoc-inputs/2023/day9sample.txt"));
        assert_eq!(1016, part_two("./aoc-inputs/2023/day9.txt"));
    }
}
