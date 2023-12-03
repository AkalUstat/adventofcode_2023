use adventofcode_2023::file_reader;

use std::io::{BufRead};
use core::ops::Range;
use std::cmp::{max, min};

use regex::{Regex};

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

#[derive(Debug)]
struct Symbol {
    line_num: usize,
    range: Range<usize>,
    // we're not storing the actual symbol since what it is has no impact
}

#[derive(Debug)]
struct Numbah {
    line_num: usize,
    range: Range<usize>,
    number: usize,
}

fn parse_lines() -> (Vec<Numbah>, Vec<Symbol>) {
    let file_r = file_reader("./inputs/day3.txt");
    let num_regex = Regex::new(r"(\d+)").unwrap();
    let symbol_regex = Regex::new(r"([^a-zA-z\d.\n])").unwrap();

   let mut nums: Vec<Numbah> = Vec::new();
   let mut syms: Vec<Symbol> = Vec::new();


    // map into readable data
    for (indx, ln) in file_r.lines().map(|l| l.unwrap()).enumerate() {
        let line = &ln;

        nums.append(&mut num_regex.find_iter(line).map(|m| {
            Numbah {
                line_num: indx,
                range: m.range(),
                number: m.as_str().parse::<usize>().unwrap(),
            }
        }).collect::<Vec<_>>());
        syms.append(&mut symbol_regex.find_iter(line).map(|sym| {
            Symbol {
                line_num: indx,
                range: sym.range(),
            }
        }).collect::<Vec<_>>());
    }

    (nums, syms)

}

fn part_one() -> usize {
    let mut part_collector = 0;
    let (nums, syms) = parse_lines();
    // i'll try spinning
    for sym in syms.iter() {
        
        let range_min = sym.range.start - 1;
        let range_max = sym.range.end + 1;

        let possible_nums: Vec<_>  = nums.iter().filter(|num| {
            num.line_num == sym.line_num + 1
                || num.line_num == sym.line_num
                || num.line_num == sym.line_num - 1
        }).collect();
        for num in possible_nums.iter() {
            let num_start = num.range.start;
            let num_end = num.range.end;
            if max(range_max, num_end) - min(range_min, num_start) < (range_max - range_min) + (num_end - num_start) {
                part_collector += num.number;
            }
        }
    }
    part_collector
}

fn part_two() -> usize {

    let mut gear_collector = 0;

    let (nums, syms) = parse_lines();
    // i'll try spinning
    for sym in syms.iter() {
        
        let range_min = sym.range.start - 1;
        let range_max = sym.range.end + 1;

        let mut gear_nums: Vec<_> = Vec::new();
        let possible_nums: Vec<_>  = nums.iter().filter(|num| {
            num.line_num == sym.line_num + 1
                || num.line_num == sym.line_num
                || num.line_num == sym.line_num - 1
        }).collect();

        for num in possible_nums.iter() {
            let num_start = num.range.start;
            let num_end = num.range.end;
            // if the ranges overlap: https://stackoverflow.com/a/25369187
            if max(range_max, num_end) - min(range_min, num_start) < (range_max - range_min) + (num_end - num_start) {
                gear_nums.push(num);

            }
        }
        if gear_nums.len() == 2 {
            gear_collector += gear_nums[0].number * gear_nums[1].number;
        }
    }
    gear_collector
}
