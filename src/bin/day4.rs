use adventofcode_2023::file_reader;

use std::collections::HashMap;
use std::io::{BufRead};

fn main() {
    println!("{}", part_one());
}

fn part_one() -> (usize, usize) {
    let file_r = file_reader("./inputs/day4.txt");
    // map into readable data
    let mut nums_collector = 0;
    let mut nested_nums_collector = 0;
    let mut lines = HashMap::new();

    for (indx, ln) in file_r.lines().map(|l| l.unwrap()).enumerate() {
        let line = &ln;
        let split: Vec<&str> = line.split(":").collect();
        let strings: Vec<&str> = split[1].split("|").collect();
        let (your_num_str, winning_num_str) = (strings[0].trim(), strings[1].trim());

        let mut your_nums: Vec<_> = your_num_str.split(" ").collect();
        let mut winning_nums: Vec<_> = winning_num_str.split(" ").collect();

        your_nums.append(&mut winning_nums);

        let mut map = HashMap::new();

        for val in your_nums.into_iter().filter(|s| s != &""){
            let count = map.entry(val).or_insert(0);
            *count += 1;
        }
        let total_matches = map.iter()
            .filter(| (_key, &value) | value == 2)
            .map(|(_key, &value)| value);
        let num_matches = total_matches.clone().collect::<Vec<_>>().len();
        
        let matches_sum = total_matches.fold(0, |acc, _num| if acc < 1 { acc + 1 } else { acc * 2 });
        lines.insert(indx, (num_matches, matches_sum));
        nums_collector += matches_sum
    }
    nums_collector
}