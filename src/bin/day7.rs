use adventofcode_2023::get_files_lines;
use std::collections::HashMap;
use std::str::FromStr;

// so we can iterate over enum values (just makes things easier for me)
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
fn main() {
    part_one();
}

#[derive(Debug, PartialEq, Eq, EnumIter, Hash, Copy, Clone, PartialOrd, Ord)]
enum Card {
    Two = 13,
    Three = 12,
    Four = 11,
    Five = 10,
    Six = 9,
    Seven = 8,
    Eight = 7,
    Nine = 6,
    T = 5,
    J = 4,
    Q = 3,
    K = 2,
    A = 1,
}

// from https://www.reddit.com/r/rust/comments/2vqama/comment/cojzafn/?utm_source=share&utm_medium=web2x&context=3
impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Card, ()> {
        match s {
            "A" => Ok(Card::A),
            "K" => Ok(Card::K),
            "Q" => Ok(Card::Q),
            "J" => Ok(Card::J),
            "T" => Ok(Card::T),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum Hand {
    FiveKind(Card),
    FourKind(Card, Card),
    FullHouse(Card, Card),
    ThreeKind(Card, Card, Card),
    TwoPair(Card, Card, Card),
    OnePair(Card, Card, Card, Card),
    HighCard(Card, Card, Card, Card, Card),
}

impl From<[Card; 5]> for Hand {
    fn from(card_arr: [Card; 5]) -> Self {
        let mut collections = HashMap::new();
        for card in Card::iter() {
            let num_card = &card_arr.into_iter().filter(|&val| val == card).count();
            if num_card >= &1 {
                collections
                    .entry(card)
                    .and_modify(|count| *count += 1)
                    .or_insert(num_card);
            }
        }

        let number_of_cards: Vec<(Card, usize)> = collections.into_iter().collect();
        println!("{:?}", number_of_cards);

        let fives = &number_of_cards
            .iter()
            .filter(|(_, value)| value == &5)
            .collect::<Vec<_>>();
        let fours = &number_of_cards
            .iter()
            .filter(|(_, value)| value == &4)
            .collect::<Vec<_>>();
        let threes = &number_of_cards
            .iter()
            .filter(|(_, value)| value == &3)
            .collect::<Vec<_>>();
        let twos = &mut number_of_cards
            .iter()
            .filter(|(_, value)| value == &2)
            .collect::<Vec<_>>();
        let ones = &mut number_of_cards
            .iter()
            .filter(|(_, value)| value == &1)
            .collect::<Vec<_>>();

        // Case: Five of a Kind - All same
        if fives.len() == 1 {
            Hand::FiveKind(fives[0].0)
        // Case: Four of a kind - 4 same, 1 diff
        } else if fours.len() == 1 {
            Hand::FourKind(fours[0].0, ones[0].0)
        // Case: Full House - 3 same, 2 same
        } else if threes.len() == 1 && twos.len() == 1 {
            Hand::FullHouse(threes[0].0, twos[0].0)
        // Case: Three of a kind - 3 same, 2 diff
        } else if threes.len() == 1 && twos.len() == 2 {
            twos.sort();
            Hand::ThreeKind(threes[0].0, twos[0].0, twos[1].0)
        // Case: Two pair - 2 same, 2 same, 1 diff
        } else if twos.len() == 2 && ones.len() == 1 {
            twos.sort();
            Hand::TwoPair(twos[0].0, twos[1].0, ones[0].0)
        // Case: 1 pair - 2 same, 3 diff
        } else if twos.len() == 1 && ones.len() == 3 {
            ones.sort();
            Hand::OnePair(twos[0].0, ones[0].0, ones[1].0, ones[2].0)
        }
        // case: High card - all diff
        else {
            ones.sort();
            println!("{}", ones.len());
            Hand::HighCard(ones[0].0, ones[1].0, ones[2].0, ones[3].0, ones[4].0)
        }
    }
}

fn part_one() -> usize {
    let lines: Vec<_> = get_files_lines("./aoc-inputs/2023/day7sample.txt");

    let map: Vec<([Card; 5], usize)> = lines
        .iter()
        .map(|s| {
            if let Some((cards, bet)) = s.split_once(" ") {
                let mut cards_arr: [Card; 5] = [Card::Two; 5];
                let parsed_cards = cards
                    .split("")
                    .filter(|s| s != &"")
                    .map(|card| card.parse::<Card>().unwrap());

                for (i, card) in parsed_cards.enumerate() {
                    cards_arr[i] = card;
                }

                (cards_arr, bet.parse::<usize>().unwrap())
            } else {
                panic!("Could not parse line");
            }
        })
        .collect::<Vec<_>>();
    println!("{:?}", map);

    let hands: Vec<(Hand, usize)> = map
        .iter()
        .map(|(cards, bet)| {
            let hand = Hand::from(*cards);
            (hand, *bet)
        })
        .collect::<Vec<_>>();
    0
}
