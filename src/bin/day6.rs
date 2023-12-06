use adventofcode_2023::get_files_lines;
fn main() {
    println!("{:?}", part_one());
}

#[derive(Debug)]
struct Race {
    total_time: usize,
    record: usize,
}

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

    let mut record_beating_collector = 0;
    for race in data.iter() {
        let vertex_x: f32 = race.total_time as f32 / 2.0;
        let vertex_y: f32 = {
            let floor = vertex_x.floor();
            let ceil = vertex_x.ceil();

            let floor_y = (race.total_time as f32 - floor) * floor;
            let ceil_y = (race.total_time as f32 - ceil) * ceil;

            println!("({:?}, {:?})", floor_y, ceil_y);
            (floor_y + ceil_y) / 2.0
        };
        println!("({:?}, {:?})", vertex_x, vertex_y);
    }

    record_beating_collector
}
