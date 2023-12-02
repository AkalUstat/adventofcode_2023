use adventofcode_2023::file_reader;

use std::io::{BufRead};
use regex::Regex;
fn main() {
    println!("{}", part_one());
}

fn part_one() -> i32{
    let filer = file_reader("./inputs/day2.txt");
    let id_regex = Regex::new(r"(\d+)").unwrap();
    let _values_regex = Regex::new(r"Game (\d+)").unwrap();

    let (max_red, max_green, max_blue) = (12i32, 13i32, 14i32);
    let mut id_collector = 0i32;

    'outer: for line in filer.lines().map(|l| l.unwrap()) {
        let line_borrow = &line;
        let mut separate_id = line_borrow.split(":");
        let id_str = separate_id.next().unwrap();
        let id = id_regex.captures(id_str).unwrap().get(0).unwrap().as_str();
        let value_str = separate_id.last().unwrap();
        for value_set in value_str.split(";") {
            let set = &value_set;
            for color_set in set.split(", ") {
                let vec = color_set.split(" ").filter(|c| c != &"").collect::<Vec<&str>>();
                match vec[1] {
                    "red" => {
                        if vec[0].parse::<i32>().unwrap() > max_red {
                            continue 'outer;
                        }
                    },
                    "blue" => {
                        if vec[0].parse::<i32>().unwrap() > max_blue {
                            continue 'outer;
                        }
                    },
                    "green" => {
                        if vec[0].parse::<i32>().unwrap() > max_green {
                            continue 'outer;
                        }
                    },
                    _ => {
                    }
                }
            }

        }

        id_collector += id.parse::<i32>().unwrap();






    }
    id_collector

}

fn part_two(){}


