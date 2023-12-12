use adventofcode_2023::get_files_lines;
use std::collections::{HashSet, VecDeque};
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Dir {
    T,
    B,
    L,
    R,
    I,
    A,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Connection {
    direction: Dir,
    connected: bool,
}
impl Connection {
    pub fn new(dir: Dir) -> Self {
        Self {
            direction: dir,
            connected: false,
        }
    }
    fn connect(&mut self) {
        self.connected = true;
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum PipeType {
    Vert,
    Horiz,
    NE,
    NW,
    SE,
    SW,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Pipe {
    kind: PipeType,
    connections: [Connection; 2],
    x: usize,
    y: usize,
}

impl FromStr for PipeType {
    type Err = ();

    fn from_str(s: &str) -> Result<PipeType, ()> {
        match s {
            "|" => Ok(PipeType::Vert),
            "-" => Ok(PipeType::Horiz),
            "L" => Ok(PipeType::NE),
            "J" => Ok(PipeType::NW),
            "7" => Ok(PipeType::SW),
            "F" => Ok(PipeType::SE),
            _ => Err(()),
        }
    }
}

impl PipeType {
    fn get_connectors(&self) -> [Connection; 2] {
        match self {
            PipeType::Vert => [Connection::new(Dir::T), Connection::new(Dir::B)],
            PipeType::Horiz => [Connection::new(Dir::L), Connection::new(Dir::R)],
            PipeType::NE => [Connection::new(Dir::T), Connection::new(Dir::R)],
            PipeType::NW => [Connection::new(Dir::T), Connection::new(Dir::L)],
            PipeType::SW => [Connection::new(Dir::B), Connection::new(Dir::L)],
            PipeType::SE => [Connection::new(Dir::B), Connection::new(Dir::R)],
        }
    }
}

impl Pipe {
    pub fn default() -> Self {
        let kind = PipeType::Vert;
        Self {
            x: 0,
            y: 0,
            kind,
            connections: kind.get_connectors(),
        }
    }
    pub fn new(x: usize, y: usize, pipe_type: PipeType) -> Self {
        Self {
            x,
            y,
            kind: pipe_type,
            connections: pipe_type.get_connectors(),
        }
    }
    pub fn get_nearby(x: usize, y: usize, grid: &Vec<Vec<char>>) -> [Option<Pipe>; 4] {
        let mut surrounding_elems: [Option<Pipe>; 4] = [None; 4];
        // looks at (0, -1)W, (0, 1)E, (-1, 0)N , (1, 0)S
        for (indx, (dy, dx)) in [(1, 0), (1, 2), (0, 1), (2, 1)].iter().enumerate() {
            let cx = (x + dx).wrapping_sub(1);
            let cy = (y + dy).wrapping_sub(1);

            // make sure the difference is 1; abandons negative indeces
            if (x.abs_diff(cx) <= 1 && y.abs_diff(cy) <= 1)
                // make sure it is within the max range as well
                && (cx < grid[0].len() && cy < grid.len())
            {
                let character = grid[cy][cx];
                if !(character == '.') && !(character == 'S') {
                    println!("{}", character == '.');
                    let pipe_type = character.to_string().parse::<PipeType>().unwrap();
                    let element = Pipe {
                        kind: pipe_type,
                        connections: pipe_type.get_connectors(),
                        x: cx,
                        y: cy,
                    };
                    surrounding_elems[indx] = Some(element);
                }
            }
        }
        // println!("{:?}", surrounding_elems);
        surrounding_elems
    }
}

fn main() {
    //println!("{}", part_one("./aoc-inputs/2023/day10sample.txt"));
    // println!("{}", part_one("./aoc-inputs/2023/day10sample2.txt"));
    // println!("{}", part_one("./aoc-inputs/day10simplesample2.txt"));
    println!("{}", part_one("./aoc-inputs/day10simplesample.txt"));
}

fn mod_set(
    elem_opt: Option<&Pipe>,
    needed_dir: Dir,
    dependent_types: [PipeType; 3],
    mut set: HashSet<PipeType>,
) -> HashSet<PipeType> {
    match elem_opt {
        Some(elem) => {
            if !elem
                .connections
                .iter()
                .any(|conn| conn.direction == needed_dir)
            {
                for ptype in dependent_types.iter() {
                    set.remove(ptype);
                }
            }
        }
        None => {
            for ptype in dependent_types.iter() {
                set.remove(ptype);
            }
        }
    }

    set
}

fn part_one(file_path: &str) -> usize {
    let lines = get_files_lines(file_path);
    let grid: Vec<Vec<_>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut start: Pipe = Pipe::default();

    let mut found_pipes: HashSet<Pipe> = HashSet::new();
    let mut checker_queue: VecDeque<Pipe> = VecDeque::new();

    'outer: for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == &'S' {
                let [left, right, top, bottom] = Pipe::get_nearby(x, y, &grid);
                let mut start_type = HashSet::from([
                    PipeType::Vert,
                    PipeType::Horiz,
                    PipeType::NE,
                    PipeType::NW,
                    PipeType::SE,
                    PipeType::SW,
                ]);

                start_type = mod_set(
                    left.as_ref(),
                    Dir::R,
                    [PipeType::Horiz, PipeType::NW, PipeType::SW],
                    start_type,
                );
                start_type = mod_set(
                    right.as_ref(),
                    Dir::R,
                    [PipeType::Horiz, PipeType::NE, PipeType::SE],
                    start_type,
                );
                start_type = mod_set(
                    top.as_ref(),
                    Dir::B,
                    [PipeType::Vert, PipeType::NE, PipeType::NW],
                    start_type,
                );

                start_type = mod_set(
                    bottom.as_ref(),
                    Dir::T,
                    [PipeType::Vert, PipeType::SE, PipeType::SW],
                    start_type,
                );

                let kind = *start_type.iter().next().unwrap();
                let mut connections = kind.get_connectors();
                connections[0].connected = true;
                connections[1].connected = true;
                
                start = Pipe {
                    kind,
                    x,
                    y,
                    connections
                };

                found_pipes.insert(start);
                break 'outer;
            }
        }
    }

    println!("{:?}", start);

    0
}

fn part_two() {}
