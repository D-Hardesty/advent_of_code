use std::collections::HashMap;
use crate::custom_error::AocError;

#[derive(Debug)]
enum PokerHand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    // sort cards into hand types
    let mut five_ok: Vec<(String, u64)> = Vec::new();
    let mut four_ok: Vec<(String, u64)> = Vec::new();
    let mut full_house: Vec<(String, u64)> = Vec::new();
    let mut three_ok: Vec<(String, u64)> = Vec::new();
    let mut two_pair: Vec<(String, u64)> = Vec::new();
    let mut one_pair: Vec<(String, u64)> = Vec::new();
    let mut high_card: Vec<(String, u64)> = Vec::new();


    for line in _input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let hand = cards_to_ascii(&parts);
        let hand_type = determine_hand(&hand.0);
        // println!("parts: {:?}", parts);
        // println!("hand: {:?}", hand);
        // println!("hand type: {:?}", hand_type);

        match hand_type {
            PokerHand::FiveOfAKind => five_ok.push(hand),
            PokerHand::FourOfAKind => four_ok.push(hand),
            PokerHand::ThreeOfAKind => three_ok.push(hand),
            PokerHand::TwoPair => two_pair.push(hand),
            PokerHand::FullHouse => full_house.push(hand),
            PokerHand::HighCard => high_card.push(hand),
            PokerHand::OnePair => one_pair.push(hand),
        }
    }
    // println!("{:?}", five_ok);
    // println!("{:?}", four_ok);
    // println!("{:?}", high_card);

    let mut scores: Vec<u64> = Vec::new();

    // sort hand types into ranks
    sort_by_value(high_card, &mut scores);
    sort_by_value(one_pair, &mut scores);
    sort_by_value(two_pair, &mut scores);
    sort_by_value(three_ok, &mut scores);
    sort_by_value(full_house, &mut scores);
    sort_by_value(four_ok, &mut scores);
    sort_by_value(five_ok, &mut scores);
    // println!("scores: {:?}", scores);

    // println!("{:?}", scores);
    // count cards and assign ranks
    let mut result = 0;
    let mut total = 1;

    for bid in scores.iter() {
        result += bid * total;
        // println!("total: {} * bid: {} = result: {}", total, bid, result);
        total += 1;
    }
    // get score

    Ok(result.to_string())
}

fn sort_by_value(mut vec: Vec<(String, u64)>, scores: &mut Vec<u64>) {
    vec.sort();
    // println!("{:?}", vec);
    for score in vec.iter() {
        scores.push(score.1);
    }
    // println!("{:?}", scores);
}

fn cards_to_ascii(cards: &Vec<&str>) -> (String, u64) {
    let mut card = cards[0].to_string();

    card = card
        .replace('A', "?")
        .replace('K', ">")
        .replace('Q', "=")
        .replace('J', "<")
        .replace('T', ";");

    (card, cards[1].parse().unwrap())
}

fn determine_hand(cards: &String) -> PokerHand {
    let mut char_count = HashMap::new();

    for ch in cards.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }

    let mut counts: Vec<usize> = char_count.values().cloned().collect();
    counts.sort_by(|a, b| b.cmp(a));

    // hand_type = first * second + 1
    // 5ok = 5, 4ok = 8, fh = 9, 3ok = 6, 2p = 6, 1p = 4, hc = 2
    let hand_type = counts[0] * (counts.get(1).unwrap_or(&0) + 1);
    if hand_type == 6 {
        return if counts[0] == 3 {
            PokerHand::ThreeOfAKind
        } else {
            PokerHand::TwoPair
        };
    }

    match hand_type {
        4 => PokerHand::OnePair,
        9 => PokerHand::FullHouse,
        8 => PokerHand::FourOfAKind,
        5 => PokerHand::FiveOfAKind,
        _ => PokerHand::HighCard,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("6440", process(input)?);
        Ok(())
    }
}