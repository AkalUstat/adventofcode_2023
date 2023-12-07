use adventofcode_2023::get_files_lines;
fn main() {
    println!("{:?}", part_one());
    println!("{:?}", part_two());
}

#[derive(Debug)]
struct Race {
    total_time: usize,
    record: usize,
}

fn part_one() -> usize {
    let lines: Vec<_> = get_files_lines("./aoc-inputs/2023/day6.txt");
    let data_strs: Vec<_> = lines
        .iter()
        .map(|s| s.split(":").last().unwrap().trim())
        .collect();

    let times = data_strs[0].trim().split(" ").filter(|s| s != &"");
    let mut distance_records = data_strs[1].trim().split(" ").filter(|s| s != &"");

    let mut data: Vec<Race> = vec![];

    for value in times.into_iter() {
        data.push(Race {
            total_time: value.parse::<usize>().unwrap(),
            record: distance_records.next().unwrap().parse::<usize>().unwrap(),
        });
    }

    let mut record_beating_collector = 1;
    for race in data.iter() {
        let range = get_range(race.total_time, race.record);
        record_beating_collector *= range.count()
    }

    record_beating_collector
}

fn part_two() -> usize {
    let lines: Vec<_> = get_files_lines("./aoc-inputs/2023/day6.txt");
    let data_strs: Vec<_> = lines
        .iter()
        .map(|s| s.split(":").last().unwrap().trim())
        .collect();

    let time = data_strs[0]
        .trim()
        .split(" ")
        .filter(|s| s != &"")
        .collect::<Vec<_>>()
        .join("");
    let distance_record = data_strs[1]
        .trim()
        .split(" ")
        .filter(|s| s != &"")
        .collect::<Vec<_>>()
        .join("");

    let race = Race {
        total_time: time.parse::<usize>().unwrap(),
        record: distance_record.parse::<usize>().unwrap(),
    };

    get_range(race.total_time, race.record).count()
}

fn get_range(b: usize, c: usize) -> core::ops::RangeInclusive<usize> {
    let b_sq = (b.pow(2) as f32).floor();
    let second_part = 4.0 * c as f32;
    let discriminant: f32 = (b_sq - second_part).sqrt();

    let exact_root_1 = ((1.0 * b as f32) - discriminant) / 2.0;
    let exact_root_2 = ((1.0 * b as f32) + discriminant) / 2.0;

    let mut root_1 = exact_root_1.ceil() as usize;
    let mut root_2 = exact_root_2.floor() as usize;

    while test_value(&root_1, &b, &c) {
        root_1 += 1;
    }

    while test_value(&root_2, &b, &c) {
        root_2 -= 1;
    }

    (root_1)..=(root_2)
}

fn test_value(value: &usize, time: &usize, record: &usize) -> bool {
    (((*time as f32) - *value as f32) * *value as f32) - *record as f32 == 0.0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn correct_outputs() {
        assert_eq!(625968, part_one());
        assert_eq!(43663326, part_two());
    }
}

/* Math
 * Given that the total distance traveled, d = (total_time, T, - hold_time, H) * H
 * We want the distances such that d > record, R
 *
 * Thus, given that, we have
 *
 * R > TH - H^2 (quadratic formula)
 *
 *
 * To find this range, we need to know
 * the roots of the quadratic, d, where the axis
 * is y = R.
 *
 * -H^2 + TH = R
 * -H^2 + TH - R = 0
 * thus, a = -1, b = T, c = -R
 *
 * To find the zeroes, we can use the quadratic formula:
 *
 * x = (-b +- sqrt( b^2 - 4ac)) / 2
 * x = (-T +- sqrt( (T)^2 - 4(-1)(-R) )) / 2
 *
 * Thus,
 * x1 = (-T + sqrt( (T)^2 - 4(-1)(-R) )) / 2(-1)
 *     = (-T + sqrt (T^2 - 4R)) / -2
 *     = (T - sqrt(T^2 - 4R)) / 2
 * x2 = (-T - sqrt( (T)^2 - 4(-1)(-R) )) / 2(-1)
 *     = (-T - sqrt (T^2 - 4R)) / -2
 *     = (T + sqrt(T^2 - 4R)) / 2
 *
 * Then, when we combine into x2 - x1 to generate our range:
 *
 *  2final_r = T + sqrt( (T)^2 - 4R) - T + sqrt(T^2 - 4R)
 *  2final_r = 2sqrt(T^2 - 4R)
 *  final_r = sqrt(T^2 - 4R)
 *
 *  Thus, final_r = sqrt(b^2 - 4R)...simply the discriminat!
 *
 *  Oh MY GOD I've forgotten HS algebra *smacks head against wall*.
 *
 *  Given our values, with a = 1, this discriminant will be the range.
 *
 *  See this quora thread: https://math.stackexchange.com/a/1335507
 *
 *
 */
