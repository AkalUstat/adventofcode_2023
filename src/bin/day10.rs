use adventofcode_2023::get_files_lines;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    // I for invalid and T for start
    N,
    E,
    S,
    W,
    I,
    All,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    NEBend,
    NWBend,
    SEBend,
    SWBend,
    Ground,
    Start,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct LoopElement {
    pipe: Pipe,
    x: usize,
    y: usize,
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
            "." => Ok(Pipe::Ground),
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
            Pipe::SWBend => [Direction::S, Direction::E],
            Pipe::Ground => [Direction::I, Direction::I],
            Pipe::Start => [Direction::All, Direction::All],
        }
    }

    fn connects(&self, other: &Pipe) -> bool {
        let self_maps = self.get_connections();
        let other_maps = other.get_connections();
        if self_maps[0] == Direction::All || self_maps[1] == Direction::All {
            return true;
        }

        if self_maps.iter().any(|x| x == &Direction::I)
            || other_maps.iter().any(|x| x == &Direction::I)
        {
            return false;
        }

        self_maps[0] == other_maps[0]
            || self_maps[1] == other_maps[1]
            || self_maps[0] == other_maps[1]
            || self_maps[1] == other_maps[0]
    }
}

fn main() {
    // println!("{}", part_one("./aoc-inputs/2023/day10sample.txt"));
    println!("{}", part_one("./testsample.txt"));
}

fn part_one(file_path: &str) -> usize {
    let lines = get_files_lines(file_path);
    let characters: Vec<Vec<_>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut start = LoopElement {
        pipe: Pipe::Start,
        x: 0,
        y: 0,
    };

    'outer: for (y, line) in characters.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == &'S' {
                start = LoopElement { x, y, ..start };
                break 'outer;
            }
        }
    }

    let mut found_elements: HashSet<LoopElement> = HashSet::new();
    found_elements.insert(start);

    let mut checker_queue: VecDeque<LoopElement> = VecDeque::new();
    checker_queue.push_back(start);

    while checker_queue.len() > 0 {
        let LoopElement { pipe, x, y } = checker_queue.pop_front().unwrap();

        for dy in [0, 1, 2] {
            for dx in [0, 1, 2] {
                let cx: isize = (x + dx) as isize - 1;
                let cy: isize = (y + dy) as isize - 1;
                if !(cx == x as isize && cy == y as isize)
                    && (cx >= 0 && cy >= 0)
                    && ((cx as usize) < characters.len() && (cy as usize) < characters.len())
                {
                    let other_pipe = characters[cy as usize][cx as usize]
                        .to_string()
                        .parse::<Pipe>()
                        .unwrap();
                    let element = LoopElement {
                        pipe: other_pipe,
                        x: cx as usize,
                        y: cy as usize,
                    };
                    if !((cx as usize) == start.x && (cy as usize) == start.y) {
                        let contains = pipe.connects(&other_pipe);
                        println!(
                            "comparing {:?} at ({}, {}) to {:?} at ({}, {}): {}",
                            pipe,
                            x,
                            y,
                            other_pipe,
                            cx,
                            cy,
                            contains,
                        );
                        if contains && !found_elements.contains(&element) {
                            checker_queue.push_back(element);
                            found_elements.insert(element);
                        }
                    }
                }
            }
        }
    }
    found_elements.len() / 2
}

fn find_loop() {}

fn part_two() {}
