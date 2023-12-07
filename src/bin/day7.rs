use adventofcode_2023::get_files_lines;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::str::FromStr;

// so we can iterate over enum values (just makes things easier for me)
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
fn main() {
    part_one();
}

#[derive(Debug, PartialEq, Eq, EnumIter, Hash, Copy, Clone, PartialOrd, Ord)]
enum Card {
    Two = 1,
    Three = 2,
    Four = 3, 
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    T = 9,
    J = 10,
    Q = 11,
    K = 12,
    A = 13,
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
enum Hand {
    HighCard { value: [Card; 5] },
    OnePair { value: [Card; 5] },
    TwoPair { value: [Card; 5] },
    ThreeKind { value: [Card; 5] },
    FullHouse { value: [Card; 5] },
    FourKind { value: [Card; 5] },
    FiveKind { value: [Card; 5] },
}

impl From<[Card; 5]> for Hand {
    fn from(card_arr: [Card; 5]) -> Self {
        let mut times = HashMap::new();
        for card in Card::iter() {
            let num_card = card_arr.into_iter().filter(|&val| val == card).count();
            if num_card >= 1 {
                times
                    .entry(card)
                    .and_modify(|count| *count += &1)
                    .or_insert(num_card);
            }
        }

        let number_of_cards: Vec<_> = times.into_iter().collect();

        let fives = &number_of_cards
            .iter()
            .filter(|(_, value)| value == &5)
            .count();
        let fours = &number_of_cards
            .iter()
            .filter(|(_, value)| value == &4)
            .count();
        let threes = &number_of_cards
            .iter()
            .filter(|(_, value)| value == &3)
            .count();
        let twos = &number_of_cards
            .iter()
            .filter(|(_, value)| value == &2)
            .count();
        let ones = &number_of_cards
            .iter()
            .filter(|(_, value)| value == &1)
            .count();

        // Case: Five of a Kind - All same
        if fives == &1 {
            Hand::FiveKind { value: card_arr }
        // Case: Four of a kind - 4 same, 1 diff
        } else if fours == &1 {
            Hand::FourKind { value: card_arr }
        // Case: Full House - 3 same, 2 same
        } else if threes == &1 && twos == &1 {
            Hand::FullHouse { value: card_arr }
        // Case: Three of a kind - 3 same, 2 diff
        } else if threes == &1 && ones == &2 {
            Hand::ThreeKind { value: card_arr }
        // Case: Two pair - 2 same, 2 same, 1 diff
        } else if twos == &2 && ones == &1 {
            Hand::TwoPair { value: card_arr }
        // Case: 1 pair - 2 same, 3 diff
        } else if twos == &1 && ones == &3 {
            Hand::OnePair  { value: card_arr }
        }
        // case: High card - all diff
        else {
            Hand::HighCard { value: card_arr }
        }
    }
}

// using https://users.rust-lang.org/t/accessing-enum-variant-trait-method-without-matching/59505/2
impl Hand {
    fn as_value(&self) -> &[Card; 5] {
        match self {
            Hand::FiveKind {value} => value,
            Hand::FourKind {value} => value,
            Hand::FullHouse {value} => value,
            Hand::ThreeKind {value} => value,
            Hand::TwoPair {value} => value,
            Hand::OnePair {value} => value,
            Hand::HighCard {value} => value,
        }
    }
}
// using https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
/*impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self < other {
           return Some(Ordering::Greater);
        } else if other < self {
            return Some(Ordering::Less);
        }  

        // if let
        for (i, card) in self.as_value().iter().enumerate() {
            if card > &other.as_value()[i] {
                return Some(Ordering::Greater);
            } else if &other.as_value()[i] > card {
                return Some(Ordering::Less);
            }

        }
        return Some(Ordering::Equal);
    }

}
// use partial ord to impl ord as noted here https://www.reddit.com/r/rust/comments/11gm19h/why_eq_partialeq_ord_and_partialord_especially/
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
       self.partial_cmp(other).unwrap()
    }
} */

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

    let mut hands: Vec<(Hand, usize)> = map
        .iter()
        .map(|(cards, bet)| {
            let hand = Hand::from(*cards);
            (hand, *bet)
        })
        .collect::<Vec<_>>();
    hands.sort_by(|(hand1, _), (hand2, _)| {
        if hand1 > hand2 {
            return Ordering::Greater;
        } else if hand2 > hand1 {
            return Ordering::Less;
        } else {
            let hand1_vals = &hand1.as_value();
            let hand2_vals = &hand2.as_value();
            for (i, card) in &hand1_vals.iter().enurmerate() {
                if card > hand2_vals[i] {
                    return Ordering::Greater;
                } else if hand2_vals[i] > card {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        }
    });
    println!("{:?}", hands);
    0
}
