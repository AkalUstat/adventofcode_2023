use adventofcode_2023::get_files_lines;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

// so we can iterate over enum values (just makes things easier for me)
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
fn main() {
    //println!("Part One: {}", part_one("./aoc-inputs/2023/day7.txt"));
    println!("Part Two: {}", part_two("./aoc-inputs/2023/day7sample.txt"));
}

// Part One
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
            Hand::OnePair { value: card_arr }
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
            Hand::FiveKind { value } => value,
            Hand::FourKind { value } => value,
            Hand::FullHouse { value } => value,
            Hand::ThreeKind { value } => value,
            Hand::TwoPair { value } => value,
            Hand::OnePair { value } => value,
            Hand::HighCard { value } => value,
        }
    }
}

#[derive(Debug)]
struct Play {
    hand: Hand,
    bet: usize,
}

fn part_one(file_path: &str) -> usize {
    let lines: Vec<_> = get_files_lines(file_path);

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

    let mut hands: Vec<Play> = map
        .iter()
        .map(|(cards, bet)| {
            let hand = Hand::from(*cards);
            Play { hand, bet: *bet }
        })
        .collect::<Vec<_>>();

    // sorts in reverse order; we can then use the index to get the rank
    hands.sort_by(|play1, play2| {
        let Play { hand: hand1, .. } = play1;
        let Play { hand: hand2, .. } = play2;

        if hand1 > hand2 {
            return Ordering::Greater;
        } else if hand2 > hand1 {
            return Ordering::Less;
        } else {
            let hand1_vals = &hand1.as_value();
            let hand2_vals = &hand2.as_value();
            println!("hand 1: {:?}, hand 2: {:?}", hand1_vals, hand2_vals);
            for (i, card) in hand1_vals.iter().enumerate() {
                if *card > hand2_vals[i] {
                    return Ordering::Greater;
                } else if hand2_vals[i] > *card {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        }
    });
    println!("{:?}", hands);
    hands
        .iter()
        .enumerate()
        .map(|(indx, Play { bet, .. })| bet * (indx + 1))
        .fold(0, |acc, val| acc + val)
}

// Part Two
#[derive(Debug, PartialEq, Eq, EnumIter, Hash, Copy, Clone, PartialOrd, Ord)]
enum CardTwo {
    J = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    T = 10,
    Q = 11,
    K = 12,
    A = 13,
}

// from https://www.reddit.com/r/rust/comments/2vqama/comment/cojzafn/?utm_source=share&utm_medium=web2x&context=3
impl FromStr for CardTwo {
    type Err = ();

    fn from_str(s: &str) -> Result<CardTwo, ()> {
        match s {
            "A" => Ok(CardTwo::A),
            "K" => Ok(CardTwo::K),
            "Q" => Ok(CardTwo::Q),
            "J" => Ok(CardTwo::J),
            "T" => Ok(CardTwo::T),
            "9" => Ok(CardTwo::Nine),
            "8" => Ok(CardTwo::Eight),
            "7" => Ok(CardTwo::Seven),
            "6" => Ok(CardTwo::Six),
            "5" => Ok(CardTwo::Five),
            "4" => Ok(CardTwo::Four),
            "3" => Ok(CardTwo::Three),
            "2" => Ok(CardTwo::Two),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
enum HandTwo {
    HighCard { value: [CardTwo; 5] },
    OnePair { value: [CardTwo; 5] },
    TwoPair { value: [CardTwo; 5] },
    ThreeKind { value: [CardTwo; 5] },
    FullHouse { value: [CardTwo; 5] },
    FourKind { value: [CardTwo; 5] },
    FiveKind { value: [CardTwo; 5] },
}

impl From<[CardTwo; 5]> for HandTwo {
    fn from(card_arr: [CardTwo; 5]) -> Self {
        let mut times = HashMap::new();
        for card in CardTwo::iter() {
            let num_card = card_arr.into_iter().filter(|&val| val == card).count();
            if num_card >= 1 {
                times
                    .entry(card)
                    .and_modify(|count| *count += &1)
                    .or_insert(num_card);
            }
        }

        let times_clone = times.clone();
        let mut count_map = times_clone
                .iter()
                .collect::<Vec<_>>();
        count_map
                .sort_by(|(card1, count1), (card2, count2)| {
                    if count1 < count2 {
                        return Ordering::Greater;
                    } else if count2 < count1 {
                        return Ordering::Less;
                    } else {
                        if card1 < card2 {
                            return Ordering::Greater;
                        } else if card2 < card1 {
                            return Ordering::Less;
                        } else {
                            return Ordering::Equal;
                        }
                    }
                });


        if let Some((_, num_js)) = &count_map.iter().find(|(card, _)| *card == &CardTwo::J) {
            if *num_js != &5 {
                let greatest = count_map[0].0;
                times.entry(*greatest).and_modify(|count| *count += *num_js);
                times.remove(&CardTwo::J);
            }
        }

        let number_of_cards: &Vec<_> = &times.into_iter().collect();

        let num_js = &number_of_cards
            .iter()
            .filter(|(key, _)| key == &CardTwo::J)
            .count();

        let fives = &mut number_of_cards
            .iter()
            .filter(|(_, value)| value == &5)
            .count();
        let fours = &mut number_of_cards
            .iter()
            .filter(|(_, value)| value == &4)
            .count();
        let threes = &mut number_of_cards
            .iter()
            .filter(|(_, value)| value == &3)
            .count();
        let  twos = &mut number_of_cards
            .iter()
            .filter(|(_, value)| value == &2)
            .count();
        let ones = &mut number_of_cards
            .iter()
            .filter(|(_, value)| value == &1)
            .count();

        if num_js > &0 {
            if fours == &1 {
                *fours = 0;
                *fives += 1;
            } else if threes == &1 {
                *threes = 0;
                *fours += 1;
            }
        }

        // Case: Five of a Kind - All same
        if fives == &1 {
            HandTwo::FiveKind { value: card_arr }
        // Case: Four of a kind - 4 same, 1 diff
        } else if fours == &1 {
            HandTwo::FourKind { value: card_arr }
        // Case: Full House - 3 same, 2 same
        } else if threes == &1 && twos == &1 {
            HandTwo::FullHouse { value: card_arr }
        // Case: Three of a kind - 3 same, 2 diff
        } else if threes == &1 && ones == &2 {
            HandTwo::ThreeKind { value: card_arr }
        // Case: Two pair - 2 same, 2 same, 1 diff
        } else if twos == &2 && ones == &1 {
            HandTwo::TwoPair { value: card_arr }
        // Case: 1 pair - 2 same, 3 diff
        } else if twos == &1 && ones == &3 {
            HandTwo::OnePair { value: card_arr }
        }
        // case: High card - all diff
        else {
            HandTwo::HighCard { value: card_arr }
        }
    }
}

// using https://users.rust-lang.org/t/accessing-enum-variant-trait-method-without-matching/59505/2
impl HandTwo {
    fn as_value(&self) -> &[CardTwo; 5] {
        match self {
            HandTwo::FiveKind { value } => value,
            HandTwo::FourKind { value } => value,
            HandTwo::FullHouse { value } => value,
            HandTwo::ThreeKind { value } => value,
            HandTwo::TwoPair { value } => value,
            HandTwo::OnePair { value } => value,
            HandTwo::HighCard { value } => value,
        }
    }
}

#[derive(Debug)]
struct PlayTwo {
    hand: HandTwo,
    bet: usize,
}

fn part_two(file_path: &str) -> usize {
    let lines: Vec<_> = get_files_lines(file_path);

    let map: Vec<([CardTwo; 5], usize)> = lines
        .iter()
        .map(|s| {
            if let Some((cards, bet)) = s.split_once(" ") {
                let mut cards_arr: [CardTwo; 5] = [CardTwo::Two; 5];
                let parsed_cards = cards
                    .split("")
                    .filter(|s| s != &"")
                    .map(|card| card.parse::<CardTwo>().unwrap());

                for (i, card) in parsed_cards.enumerate() {
                    cards_arr[i] = card;
                }

                (cards_arr, bet.parse::<usize>().unwrap())
            } else {
                panic!("Could not parse line");
            }
        })
        .collect::<Vec<_>>();

    let mut hands: Vec<PlayTwo> = map
        .iter()
        .map(|(cards, bet)| {
            let hand = HandTwo::from(*cards);
            PlayTwo { hand, bet: *bet }
        })
        .collect::<Vec<_>>();

    // sorts in reverse order; we can then use the index to get the rank
    hands.sort_by(|play1, play2| {
        let PlayTwo { hand: hand1, .. } = play1;
        let PlayTwo { hand: hand2, .. } = play2;

        if hand1 > hand2 {
            return Ordering::Greater;
        } else if hand2 > hand1 {
            return Ordering::Less;
        } else {
            let hand1_vals = &hand1.as_value();
            let hand2_vals = &hand2.as_value();
            println!("hand 1: {:?}, hand 2: {:?}", hand1_vals, hand2_vals);
            for (i, card) in hand1_vals.iter().enumerate() {
                if *card > hand2_vals[i] {
                    return Ordering::Greater;
                } else if hand2_vals[i] > *card {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        }
    });
    println!("{:?}", hands);
    hands
        .iter()
        .enumerate()
        .map(|(indx, PlayTwo { bet, .. })| bet * (indx + 1))
        .fold(0, |acc, val| acc + val)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_is_correct() {
        assert_eq!(6440, part_one("./aoc-inputs/2023/day7sample.txt"));
        assert_eq!(252295678, part_one("./aoc-inputs/2023/day7.txt"));
    }

    fn part_two_is_correct() {}
}
