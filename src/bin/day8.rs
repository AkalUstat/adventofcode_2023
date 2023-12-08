use adventofcode_2023::get_files_lines;
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
    //    println!("Part Two: {}", part_two("./aoc-inputs/2023/day7.txt"));
}

fn part_one(file_path: &str) -> usize {
    let lines: Vec<_> = get_files_lines(file_path);

    let path: &Vec<_> = &lines[0]
        .split("")
        .filter(|s| s != &"")
        .map(|s| s.parse::<Movement>().unwrap())
        .collect();

    let map: &Vec<Location> = &lines[1..]
        .iter()
        .map(|s| s.parse::<Location>().unwrap())
        .collect();

    let mut position_num = 0;
    let mut current_position: &Location = &map.iter().find(|pos| pos.position == "AAA").unwrap();

    while current_position.position != "ZZZ" {
        let next_direction = &path[position_num % path.len()];
        position_num += 1;

        let next_position;

        match next_direction {
            Movement::Right => {
               next_position = &current_position.right_path;
               current_position = &map.iter().find(|pos| pos.position == *next_position).unwrap();
            },
            Movement::Left => {
               next_position = &current_position.left_path;
               current_position = &map.iter().find(|pos| pos.position == *next_position).unwrap();
            },
        }
    }
    position_num
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

    /*#[test]
    fn part_two_is_correct() {
        assert_eq!(5905, part_two("./aoc-inputs/2023/day7sample.txt"));
        assert_eq!(6839, part_two("./aoc-inputs/2023/day7sample2.txt"));
        assert_eq!(250577259, part_two("./aoc-inputs/2023/day7.txt"));
    }*/
}
