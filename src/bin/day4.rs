use adventofcode_2023::file_reader;

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Debug)]
struct Card {
    count: usize,
    value: usize,
    num_matches: usize,
    card_number: usize,
}

impl Card {
    fn increment(&mut self, value: usize) {
        (*self).count += value;
    }
}

fn main() {
    println!("{:?}", algo());
}

fn algo() -> (usize, usize) {
    let file_r: BufReader<_> = file_reader("./inputs/day4.txt");
    // map into readable data
    let mut nums_collector = 0;
    let mut many_scratches_collector = 0;
    let mut cards: HashMap<String, RefCell<Card>> = HashMap::new();

    let lines: Vec<_> = file_r.lines().map(|l| l.unwrap()).collect();
    let total_cards = lines.len();

    for (indx, ln) in lines.iter().enumerate() {
        let line = &ln;
        let split: Vec<&str> = line.split(":").collect();
        let strings: Vec<&str> = split[1].split("|").collect();
        let (your_num_str, winning_num_str) = (strings[0].trim(), strings[1].trim());

        let mut your_nums: Vec<_> = your_num_str.split(" ").collect();
        let mut winning_nums: Vec<_> = winning_num_str.split(" ").collect();

        your_nums.append(&mut winning_nums);

        let mut map = HashMap::new();

        for val in your_nums.into_iter().filter(|s| s != &""){
            let count = map.entry(val).or_insert(0);
            *count += 1;
        }
        let total_matches = map.iter()
            .filter(| (_key, &value) | value == 2)
            .map(|(_key, &value)| value);
        let num_matches = total_matches.clone().collect::<Vec<_>>().len();

        let matches_sum = total_matches.fold(0, |acc, _num| if acc < 1 { acc + 1 } else { acc * 2 });
        nums_collector += matches_sum;

        cards.insert((indx+1).to_string(), RefCell::new(Card {
            count: 1,
            value: matches_sum,
            num_matches,
            card_number: indx + 1
        }));
    }

    for i in 1..=total_cards {
        let index = i.to_string();
        let card = cards.get(&index).unwrap();
        for j in (i+1)..=(i + card.borrow().num_matches) {
            let future_indx = j.to_string();
            let future_card = cards.get(&future_indx).unwrap();
            (*future_card).borrow_mut().increment(card.borrow().count * 1);

        }
    }
    many_scratches_collector += cards.iter()
        .map(|(_key, value)| value.borrow().count)
        .fold(0, |acc, value| acc + value);
    (nums_collector, many_scratches_collector)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn correct_outputs() {
        assert_eq!((17803, 5554894), algo());
    }

}
