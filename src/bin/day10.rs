use adventofcode_2023::get_files_lines;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
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
    prev_connection: Option<Direction>,
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
            Pipe::SEBend => [Direction::S, Direction::E],
            Pipe::SWBend => [Direction::S, Direction::W],
            Pipe::Ground => [Direction::I, Direction::I],
            Pipe::Start => [Direction::All, Direction::All],
        }
    }

    fn connects(&self, other: &Pipe, prev_connection: Option<&Direction>) -> Option<Direction> {
        let mut self_maps = self
            .get_connections()
            .iter()
            .collect::<Vec<_>>();

        if let Some(prev_dir) = prev_connection {
            self_maps
                .iter_mut()
                .filter(|&x| x != &prev_dir)
                .collect::<Vec<_>>();
        }
        let other_maps = other
            .get_connections()
            .iter()
            .collect::<Vec<_>>();
        println!("{:?} to {:?}", self_maps, other_maps);

        if self == &Pipe::Ground || other == &Pipe::Ground {
            return None;
        }

        if self_maps[0] == &Direction::All || self_maps[1] == &Direction::All {
            return None;
        }

        self_maps.iter().find(|x| other_maps.contains(&x))
        
    }
}

fn main() {
    println!("{}", part_one("./aoc-inputs/2023/day10sample.txt"));
    // println!("{}", part_one("./aoc-inputs/2023/day10sample2.txt"));
    // println!("{}", part_one("./aoc-inputs/day10simplesample.txt"));
    // println!("{}", part_one("./aoc-inputs/day10simplesample2.txt"));
}

fn part_one(file_path: &str) -> usize {
    let lines = get_files_lines(file_path);
    let characters: Vec<Vec<_>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut start = LoopElement {
        pipe: Pipe::Start,
        prev_connection: None,
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
        let LoopElement {
            prev_connection,
            pipe,
            x,
            y,
        } = checker_queue.pop_front().unwrap();

        // where x is col, y is row
        for (dy, dx) in [(1, 0), (1, 2), (0, 1), (2, 1)] {
            let cx = (x + dx).wrapping_sub(1);
            let cy = (y + dy).wrapping_sub(1);

            if (x.abs_diff(cx) <= 1 && y.abs_diff(cy) <= 1)
                && (cx < characters[0].len() && cy < characters.len())
            {
                let other_pipe = characters[cy][cx].to_string().parse::<Pipe>().unwrap();
                // println!("{:#?}", element);
                if !(cx == start.x && cy == start.y) {
                    let contains = pipe.connects(&other_pipe, prev_connection);
                    println!(
                        "comparing {:?} at ({}, {}) to {:?} at ({}, {}): {}",
                        pipe, x, y, other_pipe, cx, cy, contains,
                    );
                    // if contains && !found_elements.contains(&element) {
                    //     checker_queue.push_back(element);
                    //     found_elements.insert(element);
                    // }
                }
            }
        }
    }
    match found_elements.len() % 2 {
        0 => found_elements.len() / 2,
        1 => (found_elements.len() / 2) + 1,
        _ => 0,
    }
}

fn find_loop() {}

fn part_two() {}
