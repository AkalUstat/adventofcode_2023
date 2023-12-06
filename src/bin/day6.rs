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

/*
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
 * x1 = (-T +- sqrt( (T)^2 - 4(-1)(-R) )) / 2
 * x2 = (-T +- sqrt( (T)^2 - 4(-1)(-R) )) / 2
 *
 *
 */

fn part_one() -> usize {
    let lines: Vec<_> = get_files_lines("./aoc-inputs/2023/day6sample.txt");
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
        let (x1, x2) = quadratic_formula(&-1, &(race.total_time as isize), &(race.record as isize));
        println!("{}, {}", x1, x2);
        record_beating_collector *= x2.floor() as usize - (x1+1.0).ceil() as usize + 1;
        // record_beating_collector *= total;
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
    println!("time: {}, record: {}", time, distance_record);

    let race = Race {
        total_time: time.parse::<usize>().unwrap(),
        record: distance_record.parse::<usize>().unwrap(),
    };

    let (x1, x2) = quadratic_formula(&-1, &(race.total_time as isize), &(race.record as isize));
    x2.ceil() as usize - x1.ceil() as usize
}

fn quadratic_formula(a: &isize, b: &isize, c: &isize) -> (f32, f32) {
    let discriminant = b.pow(2) as f32 - (4 * a * (c * -1)) as f32;

    let x1: f32 = ((*b * -1) as f32 + discriminant.sqrt()) / (2.0 * (*a as f32));
    let x2: f32 = ((*b * -1) as f32 - discriminant.sqrt()) / (2.0 * (*a as f32));

    (x1, x2)
}

