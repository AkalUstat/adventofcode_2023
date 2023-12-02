use std::fs::File;
use std::io::{BufRead, BufReader};

// use regex::Regex;
use aho_corasick::{AhoCorasick, PatternID};

/* fn part1() -> i32 {
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

} */

pub fn part2() -> i32 {
    let mut calibration_total = 0i32;
    //let nums = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine{1,1}))").unwrap();

    let file = File::open("./inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);

    let patterns = &["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
                        "six", "seven", "eight", "nine"];
    let ac = AhoCorasick::new(patterns).unwrap();

    let mut lines_iter = reader.lines().map(|l| l.unwrap());

    for line in lines_iter {
        let line_val = &line;
        let mut matches = ac.find_iter(line_val);
        // let mut matches = nums.find_iter(line_val);
        //println!("{:?} {:?}", matches.next().unwrap().unwrap().as_str(), matches.last().unwrap().unwrap().as_str());
        let first = match matches.next() {
            None => "0",
            Some(x) => patterns[x.pattern().as_usize()],
        };
        let last = match matches.last() {
            None => "0",
            Some(x) => patterns[x.pattern().as_usize()],
        };
        let first_digit = match first {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => first
        }.parse::<i32>().unwrap() * 10;
        let last_digit = match last {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => last
        }.parse::<i32>().unwrap();
       // println!(" {} => {}, {} => {}", first, first_digit, last, last_digit);
         calibration_total = calibration_total + first_digit + last_digit;

    }
    calibration_total


}
 
fn main() {
    // println!("{}", part1());
    println!("{}", part_two2(INPUT));
}
