use adventofcode_2023::get_files_lines;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;


#[derive(Debug, PartialEq, Eq)]
enum Direction {
    // I for invalid and T for start
    N, E, S, W, I, T
}

#[derive(Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    NEBend,
    NWBend,
    SEBend,
    SWBend,
    Ground,
    Start
}

#[derive(Debug)]
struct LoopElement {
    pipe: Pipe,
    x: usize,
    y: usize
}

impl FromStr for Pipe {
    type Err = ();

    fn from_str(s: &str) -> Result<Pipe, ()> {
        match s {
            "|" => Ok(Pipe::Vertical),
            "-" => Ok(Pipe::Horizontal),
            "L" => Ok(Pipe::NEBend),
            "J" => Ok(Pipe::NWBend),
            "7" => Ok(Pipe::SEBend),
            "F" => Ok(Pipe::SWBend),
            "S" => Ok(Pipe::Start),
            _ => Err(()),
        }
    }
}

impl Pipe {
    fn get_connections(&self) -> [Direction; 2] {
        match self {
            Pipe::Vertical => [Direction::N, Direction::S],
            Pipe::Horizontal => [Direction::E, Direction::W],
            Pipe::NEBend => [Direction::N, Direction::E],
            Pipe::NWBend => [Direction::N, Direction::W],
            Pipe::SEBend => [Direction::S, Direction::W],
            Pipe::SWBend => [Direction:: S, Direction::E],
            Pipe::Ground => [Direction::I, Direction:: I],
            Pipe::Start => [Direction::T, Direction::T],
        }
    }

    fn connects(&self, other: &Pipe) -> bool {
        let mut this_vec = self.get_connections().into_iter();
        let other_vec = other.get_connections().into_iter().collect::<Vec<_>>();
        this_vec.any(|val| other_vec.contains(&val))
    }
}

fn main(){
    part_one("./aoc-inputs/2023/day10sample.txt");
}

fn part_one(file_path: &str) {
    let lines = get_files_lines(file_path);
    let characters: Vec<Vec<_>> = lines.iter().map(|line| line.chars().collect::<Vec<_>>()).collect();

    let mut start =  LoopElement {
        pipe: Pipe::Start,
        x: 0,
        y: 0
    };

    'outer: for (y, line) in characters.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == &'S' {
                start = LoopElement {
                    x,
                    y,
                    ..start
                };
                break 'outer;
            }
        }
    }

    
    println!("{:#?}", start);
}

fn find_loop() {

}


fn part_two() {}
