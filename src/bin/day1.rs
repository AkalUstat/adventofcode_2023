use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn part1() -> i32 {
    let mut calibration_total = 0i32;
    let nums = Regex::new(r"[0-9]{1,1}").unwrap();

    let file = File::open("./inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_val = &line.expect("line");
        let mut matches = nums.find_iter(line_val);
        let first_digit = match matches.next() {
            None => "0",
            Some(x) => x.as_str(),
        };
        let last_digit = match matches.last() {
            None => first_digit,
            Some(x) => x.as_str(),
        };

        calibration_total = calibration_total + format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();

    }
    calibration_total

}

pub fn part2() -> i32 {
    let mut calibration_total = 0i32;
    let nums = Regex::new(r"\d|on|tw|thre|fou|fiv|si|seve|eigh|nin{1,1}").unwrap();

    let file = File::open("./inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_val = &line.expect("line");
        let mut matches = nums.find_iter(line_val);
        let first = match matches.next() {
            None => "0",
            Some(x) => x.as_str(),
        };
        let last = match matches.last() {
            None => first,
            Some(x) => x.as_str(),
        };

        let first_digit = match first {
            "on" => "1",
            "tw" => "2",
            "thre" => "3",
            "fou" => "4",
            "fiv" => "5",
            "si" => "6",
            "seve" => "7",
            "eigh" => "8",
            "nin" => "9",
            _ => first
        };
        let last_digit = match last {
            "on" => "1",
            "tw" => "2",
            "thre" => "3",
            "fou" => "4",
            "fiv" => "5",
            "si" => "6",
            "seve" => "7",
            "eigh" => "8",
            "nin" => "9",
            _ => last
        };
        calibration_total = calibration_total + first_digit.parse::<i32>().unwrap() * 10 + last_digit.parse::<i32>().unwrap();

    }
    calibration_total


}

fn main(){
    // println!("{}", part1());
    println!("{}", part2());
}
