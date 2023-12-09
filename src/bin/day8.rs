use adventofcode_2023::get_files_lines;
use core::ops::Range;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
enum Movement {
    Right,
    Left,
}

impl FromStr for Movement {
    type Err = ();
    fn from_str(s: &str) -> Result<Movement, ()> {
        match s {
            "R" => Ok(Movement::Right),
            "L" => Ok(Movement::Left),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Location {
    position: String,
    left_path: String,
    right_path: String,
}

impl FromStr for Location {
    type Err = ();
    fn from_str(s: &str) -> Result<Location, ()> {
        if let Some((position, path)) = s.split_once(" = ") {
            if let Some((left, right)) = path.split_once(",") {
                Ok(Location {
                    position: position.to_string(),
                    left_path: left[1..].trim().to_string(),
                    right_path: right[..right.len() - 1].trim().to_string(),
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Location {} goes to
            {} ----|     |---- {}!",
            self.position, self.left_path, self.right_path
        )
    }
}
fn main() {
    println!("Part One: {}", part_one("./aoc-inputs/2023/day8.txt"));
    println!("Part Two: {}", part_two("./aoc-inputs/2023/day8.txt"));
}

fn part_one(file_path: &str) -> usize {
    let lines: Vec<_> = get_files_lines(file_path);

    let path: &Vec<_> = &get_path(&lines);
    let map: &Vec<Location> = &get_map(&lines);

    let starting_location: &Location = &map.iter().find(|pos| pos.position == "AAA").unwrap();

    calculate_path(starting_location, "ZZZ", 0..3, path, map)
}

fn part_two(file_path: &str) -> usize {
    let lines: Vec<_> = get_files_lines(file_path);

    let path: &Vec<_> = &get_path(&lines);
    let map: &Vec<Location> = &get_map(&lines);

    let starting_locations = &map
        .iter()
        .filter(|loc| &loc.position[2..3] == "A")
        .collect::<Vec<_>>();
    let mut path_val_to_z: Vec<usize> = vec![];

    for loc in starting_locations {
        path_val_to_z.push(calculate_path(loc, "Z", 2..3, path, map))
    }

    while path_val_to_z.len() > 1 {
        path_val_to_z = path_val_to_z
            .chunks(2)
            .map(|values| {
                if values.len() == 1 {
                    values[0]
                } else {
                    lcm(values[0], values[1])
                }
            })
            .collect::<Vec<_>>();
    }

    path_val_to_z[0]
}
fn get_path(lines: &Vec<String>) -> Vec<Movement> {
    lines[0]
        .split("")
        .filter(|s| s != &"")
        .map(|s| s.parse::<Movement>().unwrap())
        .collect()
}

fn get_map(lines: &Vec<String>) -> Vec<Location> {
    lines[1..]
        .iter()
        .map(|s| s.parse::<Location>().unwrap())
        .collect()
}

fn calculate_path(
    starting_loc: &Location,
    end_str: &str,
    find_pos: Range<usize>,
    path: &Vec<Movement>,
    map: &Vec<Location>,
) -> usize {
    let mut position_num = 0;
    let mut current_position = starting_loc;

    while &current_position.position[find_pos.clone()] != end_str {
        let next_direction = &path[position_num % path.len()];
        position_num += 1;

        let next_position;

        match next_direction {
            Movement::Right => {
                next_position = &current_position.right_path;
                current_position = &map
                    .iter()
                    .find(|pos| pos.position == *next_position)
                    .unwrap();
            }
            Movement::Left => {
                next_position = &current_position.left_path;
                current_position = &map
                    .iter()
                    .find(|pos| pos.position == *next_position)
                    .unwrap();
            }
        }
    }
    position_num
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    // using euclidean algo: https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclid's_algorithm
    let mut num1;
    let mut num2;

    if a > b {
        num1 = a;
        num2 = b;
    } else {
        num1 = b;
        num2 = a;
    }

    let mut remainder;
    while (num1 % num2) > 0 {
        remainder = num1 % num2;
        num1 = num2;
        num2 = remainder;
    }

    return num2;
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_is_correct() {
        assert_eq!(2, part_one("./aoc-inputs/2023/day8sample.txt"));
        assert_eq!(6, part_one("./aoc-inputs/2023/day8sample2.txt"));
        assert_eq!(22357, part_one("./aoc-inputs/2023/day8.txt"));
    }

    #[test]
    fn part_two_is_correct() {
        assert_eq!(6, part_two("./aoc-inputs/2023/day8samplepart2.txt"));
        assert_eq!(10371555451871, part_two("./aoc-inputs/2023/day8.txt"));
    }
}
