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
            "7" => Ok(Pipe::SWBend),
            "F" => Ok(Pipe::SEBend),
            "S" => Ok(Pipe::Start),
            "." => Ok(Pipe::Ground),
            _ => Err(()),
        }
    }
}
impl Pipe {
    pub fn get_type_of_s(pipe: &LoopElement, grid: &Vec<Vec<char>>) -> Pipe {
        let LoopElement { x, y, .. } = pipe;
        let [west, east, north, south] = get_around_elements(*x, *y, grid);
        println!(
            "North: {:#?}, South: {:#?}, West: {:#?}, East: {:#?}",
            north, south, west, east
        );
        let mut pipe_type = HashSet::from([
            Pipe::Vertical,
            Pipe::Horizontal,
            Pipe::NEBend,
            Pipe::NWBend,
            Pipe::SEBend,
            Pipe::SWBend,
        ]);

        if let Some(west_elem) = west {
            if !west_elem.pipe.get_self_connections().contains(&Direction::E) {
                pipe_type.remove(&Pipe::Horizontal);
                pipe_type.remove(&Pipe::NWBend);
                pipe_type.remove(&Pipe::SWBend);
            }
        } else {
            pipe_type.remove(&Pipe::Horizontal);
            pipe_type.remove(&Pipe::NWBend);
            pipe_type.remove(&Pipe::SWBend);
        }
        if let Some(east_elem) = east {
            if !east_elem.pipe.get_self_connections().contains(&Direction::W) {
                pipe_type.remove(&Pipe::Horizontal);
                pipe_type.remove(&Pipe::NEBend);
                pipe_type.remove(&Pipe::SEBend);
            }
        } else {
            pipe_type.remove(&Pipe::Horizontal);
            pipe_type.remove(&Pipe::NEBend);
            pipe_type.remove(&Pipe::SEBend);
        }
        if let Some(north_elem) = north {
            if !north_elem.pipe.get_self_connections().contains(&Direction::S) {
                pipe_type.remove(&Pipe::Vertical);
                pipe_type.remove(&Pipe::NEBend);
                pipe_type.remove(&Pipe::NWBend);
            }
        } else {
            pipe_type.remove(&Pipe::Vertical);
            pipe_type.remove(&Pipe::NEBend);
            pipe_type.remove(&Pipe::NWBend);
        }
        if let Some(south_elem) = south {
            if !south_elem.pipe.get_self_connections().contains(&Direction::N) {
                pipe_type.remove(&Pipe::Vertical);
                pipe_type.remove(&Pipe::SWBend);
                pipe_type.remove(&Pipe::SEBend);
            }
        } else {
            pipe_type.remove(&Pipe::Vertical);
            pipe_type.remove(&Pipe::SWBend);
            pipe_type.remove(&Pipe::SEBend);
        }
        println!("{:?}", pipe_type);

        *pipe_type.iter().next().unwrap()
    }
    fn get_self_connections(&self) -> [Direction; 2] {
        match self {
            Pipe::Vertical => [Direction::N, Direction::S],
            Pipe::Horizontal => [Direction::E, Direction::W],
            Pipe::NEBend => [Direction::N, Direction::E],
            Pipe::NWBend => [Direction::N, Direction::W],
            Pipe::SEBend => [Direction::S, Direction::E],
            Pipe::SWBend => [Direction::S, Direction::W],
            Pipe::Ground | Pipe::Start => [Direction::I, Direction::I],
        }
    }
    fn get_needed_connections(&self) -> [Direction; 2] {
        match self {
            Pipe::Vertical => [Direction::N, Direction::S],
            Pipe::Horizontal => [Direction::E, Direction::W],
            Pipe::NEBend => [Direction::S, Direction::W],
            Pipe::NWBend => [Direction::S, Direction::E],
            Pipe::SEBend => [Direction::N, Direction::W],
            Pipe::SWBend => [Direction::N, Direction::E],
            Pipe::Ground | Pipe::Start => [Direction::I, Direction::I],
        }
    }
}

// Returns an [Option<LoopElement>; 4] => 0 -> West, 1 -> East, 2 -> North, 3 -> South
fn get_around_elements(x: usize, y: usize, grid: &Vec<Vec<char>>) -> [Option<LoopElement>; 4] {
    let mut surrounding_elems: [Option<LoopElement>; 4] = [None; 4];
    // looks at (0, -1)W, (0, 1)E, (-1, 0)N , (1, 0)S
    for (indx, (dy, dx)) in [(1, 0), (1, 2), (0, 1), (2, 1)].iter().enumerate() {
        let cx = (x + dx).wrapping_sub(1);
        let cy = (y + dy).wrapping_sub(1);

        // make sure the difference is 1; abandons negative indeces
        if (x.abs_diff(cx) <= 1 && y.abs_diff(cy) <= 1)
            // make sure it is within the max range as well
            && (cx < grid[0].len() && cy < grid.len())
        {
            let other_pipe = grid[cy][cx].to_string().parse::<Pipe>().unwrap();
            let element = LoopElement {
                pipe: other_pipe,
                x: cx,
                y: cy,
            };
            surrounding_elems[indx] = Some(element);
        }
    }
    // println!("{:?}", surrounding_elems);
    surrounding_elems
}

fn get_valid_elems(
    curr_elem: &LoopElement,
    set: &HashSet<LoopElement>,
    grid: &Vec<Vec<char>>,
) -> Vec<LoopElement> {
    let mut valid_elems: Vec<LoopElement> = vec![];

    let LoopElement { x, y, pipe } = curr_elem;
    println!("{:?}", curr_elem);
    let surrounding_elems = get_around_elements(*x, *y, grid);

    let mut prev_connected_elem = LoopElement {
        pipe: Pipe::Ground,
        x: 0,
        y: 0,
    };

    for elem_opt in surrounding_elems {
        if let Some(elem) = elem_opt {
            if set.contains(&elem) {
                println!("Here! {:?}", elem);
                prev_connected_elem = elem;
            }
        }
    }

    let needs_dir = [Direction::E, Direction::W, Direction::S, Direction::N];

    let connection_pts = pipe
        .get_needed_connections()
        .into_iter()
        .filter(|&x| !prev_connected_elem.pipe.get_self_connections().contains(&x))
        .collect::<Vec<_>>();
    println!("{:?} -> {:?}", prev_connected_elem, connection_pts);

    for (elem_opt, needed_direction) in std::iter::zip(surrounding_elems, needs_dir) {
        // for elem_opt in surrounding_elems {
        if let Some(elem) = elem_opt {
            if !(elem.pipe == Pipe::Start) {
                let elem_connections = elem.pipe.get_self_connections();
                // println!("With Current Pipe {:?} at ({:?}, {:?}) with connections {:?}, comparing to {:?} at ({:?}, {:?}) with connections {:?}. Needs {:?}.", pipe, x, y,pipe_connections, elem, elem.x, elem.y, elem_connections, needed_direction);
                if elem_connections.contains(&needed_direction)
                    && connection_pts.iter().any(|x| elem_connections.contains(&x))
                {
                    valid_elems.push(elem);
                }
            }
        }
    }
    println!("{:?}: {:?}", valid_elems.len(), valid_elems);
    valid_elems
}

fn main() {
    println!("{}", part_one("./aoc-inputs/2023/day10sample.txt"));
    println!("{}", part_one("./aoc-inputs/2023/day10sample2.txt"));
    // println!("{}", part_one("./aoc-inputs/day10simplesample2.txt"));
    // println!("{}", part_one("./aoc-inputs/day10simplesample.txt"));
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
                let pipe_type = Pipe::get_type_of_s(
                    &LoopElement {
                        x,
                        y,
                        pipe: Pipe::Start,
                    },
                    &characters,
                );
                start = LoopElement {
                    x,
                    y,
                    pipe: pipe_type,
                    ..start
                };
                break 'outer;
            }
        }
    }

    let mut found_elements: HashSet<LoopElement> = HashSet::new();
    found_elements.insert(start);

    let mut checker_queue: VecDeque<LoopElement> = VecDeque::new();
    checker_queue.push_back(start);

    let mut prev_elem = LoopElement {
        pipe: Pipe::Start,
        ..start
    };
    while checker_queue.len() > 0 {
        let current_elem = checker_queue.pop_front().unwrap();
        let valid_elems = get_valid_elems(&current_elem, &found_elements, &characters);
        for elem in valid_elems {
            if !found_elements.contains(&elem) {
                checker_queue.push_back(elem);
                found_elements.insert(elem);
            }
        }
    }

    // while checker_queue.len() > 0 {
    //     let LoopElement { pipe, x, y } = checker_queue.pop_front().unwrap();

    //     // where x is col, y is row
    //     for (dy, dx) in [(1, 0), (1, 2), (0, 1), (2, 1)] {
    //         let cx = (x + dx).wrapping_sub(1);
    //         let cy = (y + dy).wrapping_sub(1);

    //         if (x.abs_diff(cx) <= 1 && y.abs_diff(cy) <= 1)
    //             && (cx < characters[0].len() && cy < characters.len())
    //         {
    //             let other_pipe = characters[cy][cx].to_string().parse::<Pipe>().unwrap();
    //             // println!("{:#?}", element);
    //             if !(cx == start.x && cy == start.y) {
    //                 let contains = pipe.connects(&other_pipe, dx as isize - 1, dy as isize - 1);
    //                 println!(
    //                     "comparing {:?} at ({}, {}) to {:?} at ({}, {}): {:?}",
    //                     pipe, x, y, other_pipe, cx, cy, contains,
    //                 );
    //                 let element = LoopElement {
    //                     pipe: other_pipe,
    //                     x: cx,
    //                     y: cy,
    //                 };
    //                 if contains && !found_elements.contains(&element) {
    //                     checker_queue.push_back(element);
    //                     found_elements.insert(element);
    //                 }
    //             }
    //             // if contains && !found_elements.contains(&element) {
    //             //     checker_queue.push_back(element);
    //             //     found_elements.insert(element);
    //             // }
    //         }
    //     }
    // }
    println!("{:#?}", found_elements);
    match found_elements.len() % 2 {
        0 => found_elements.len() / 2,
        1 => (found_elements.len() / 2) + 1,
        _ => 0,
    }
}

fn find_loop() {}

fn part_two() {}
