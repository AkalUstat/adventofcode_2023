use adventofcode_2023::get_files_lines;
use core::ops::Range;
use std::cmp::{max, min};

fn main() {
    println!("{:?}", algo());
    // part_one();
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Mapping {
    dest_start: usize,
    src_start: usize,
    range: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Block {
    range: Vec<Mapping>,
}

fn algo() -> (usize, usize) {
    let lines: Vec<_> = get_files_lines("./aoc-inputs/2023/day5.txt");
    let total_lines = lines.len();

    let mut lowest_seed = std::usize::MAX;
    let mut lowest_seed_range = lowest_seed;

    let seeds: Vec<_> = lines[0]
        .split(":")
        .last()
        .unwrap()
        .split(" ")
        .filter(|s| s != &"")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut seed_ranges: Vec<_> = seeds
        .chunks(2)
        .map(|val| val[0]..(val[0] + val[1]))
        .collect();

    // println!("{:?}", &lines[1..]);
    let data_lines = &lines[1..];
    let map_indeces: &Vec<_> = &data_lines
        .iter()
        .enumerate()
        .filter(|(_, value)| value.contains(":"))
        .map(|(indx, _)| indx)
        .collect();

    let mut blocks: Vec<Block> = vec![];
    for i in 0..=(map_indeces.len() - 1) {
        let range = if i == map_indeces.len() - 1 {
            map_indeces[i]..=(data_lines.len() - 1)
        } else {
            map_indeces[i]..=(map_indeces[i + 1] - 1)
        };
        let lines = &data_lines[range];
        let ranges: Vec<Mapping> = lines
            .iter()
            .filter(|line| !line.contains(":"))
            .map(|line| {
                let values: Vec<_> = line
                    .split(" ")
                    .filter(|s| s != &"")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                Mapping {
                    dest_start: values[0],
                    src_start: values[1],
                    range: values[2],
                }
            })
            .collect();
        blocks.push(Block { range: ranges })
    }
    // part one
    for seed in seeds.iter() {
        let final_loc = seed_to_loc_mapper(&seed, &blocks);
        if final_loc < lowest_seed {
            lowest_seed = final_loc;
        }
    }

    // part two
    // I really thank https://youtu.be/NmxHw_bHhGM?si=s6jIEr_zd_rDDoBi for explaining the logic
    // behind this.
    // I got to learn a new algo!
    // However, this uses a completely different logic to what I used for part two, so the TODO
    // is to somehow reconcile to two.
    for block in blocks.iter() {
        let mut updated_values: Vec<Range<usize>> = vec![];
        while seed_ranges.len() > 0 {
            let Range { start, end } = seed_ranges.pop().unwrap();
            let mut found_overlap = false;
            for Mapping { dest_start, src_start, range } in block.range.iter() {
                let overlap_start = max(start, *src_start);
                let overlap_end = min(end, *src_start + *range);

                // if we have an overlap
                if overlap_end > overlap_start {
                    found_overlap = true;
                    updated_values.push((dest_start + (overlap_start - src_start))..(dest_start + (overlap_end - src_start)));

                    // covers values maybe not caught before the overlap
                    if overlap_start > start {
                        seed_ranges.push(start..overlap_start);
                    }
                    // covers values maybe not caught after the overlap
                    if end > overlap_end {
                        seed_ranges.push(overlap_end..end);
                    }
                    break;
                }
            }
            // if there were no overlaps here
            if !found_overlap {
                updated_values.push(start..end);
            }
        }
        seed_ranges = updated_values;
    }

    for range in seed_ranges.iter() {
        if range.start < lowest_seed_range {
            lowest_seed_range = range.start;
        }
    }
    (lowest_seed, lowest_seed_range)
}

fn seed_to_loc_mapper(seed: &usize, mappings: &Vec<Block>) -> usize {
    let mut calculated_value: usize = *seed;
    for block in mappings.iter() {
        let found_val = block
            .range
            .iter()
            .find(|s| (s.src_start..(s.src_start + s.range)).contains(&calculated_value));
        match found_val {
            Some(x) => {
                let dest_value = x.dest_start + (calculated_value - x.src_start);
                calculated_value = dest_value;
            }
            None => {}
        }
    }
    calculated_value
}
