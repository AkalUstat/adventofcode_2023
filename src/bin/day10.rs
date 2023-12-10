use adventofcode_2023::get_files_lines;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;


struct Coordinate {
    x: usize,
    y: usize
}
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    // I for invalid and T for start
    N, E, S, W, I, T
}

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

fn main(){}

fn part_one() {}
fn part_two() {}
