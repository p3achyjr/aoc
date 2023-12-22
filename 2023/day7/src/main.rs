use std::cmp;
use std::collections::HashMap;
use std::fs;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum Kind {
    HighCard,
    Pair,
    TwoPair,
    Triple,
    FullHouse,
    Quad,
    Sweep,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    kind: Kind,
    hand: (usize, usize, usize, usize, usize),
}

fn convert_card(c: char) -> usize {
    if c.is_digit(10) {
        c.to_digit(10).unwrap() as usize
    } else {
        match c {
            'T' => 10,
            'J' => 0,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Unknown card. {}", c),
        }
    }
}

fn hand_of((c0, c1, c2, c3, c4): (usize, usize, usize, usize, usize)) -> Kind {
    let mut counts = HashMap::new();
    *counts.entry(c0).or_insert(0) += 1;
    *counts.entry(c1).or_insert(0) += 1;
    *counts.entry(c2).or_insert(0) += 1;
    *counts.entry(c3).or_insert(0) += 1;
    *counts.entry(c4).or_insert(0) += 1;

    let mut num_pairs = 0;
    let mut num_trips = 0;
    let mut is_quad = false;
    let mut is_sweep = false;

    for (card, count) in counts.iter() {
        if *card == 0 {
            continue;
        }

        if *count == 5 {
            is_sweep = true;
        } else if *count == 4 {
            is_quad = true;
        } else if *count == 3 {
            num_trips += 1;
        } else if *count == 2 {
            num_pairs += 1;
        }
    }

    // handle jokers.
    let num_jokers = counts.get(&0).unwrap_or(&0).clone();
    if num_jokers >= 4 {
        is_sweep = true;
    } else if num_jokers == 3 && num_pairs > 0 {
        is_sweep = true;
    } else if num_jokers == 3 {
        // other two cards are different.
        is_quad = true;
    } else if num_jokers == 2 && num_trips > 0 {
        is_sweep = true;
    } else if num_jokers == 2 && num_pairs > 0 {
        is_quad = true;
    } else if num_jokers == 2 {
        // everything else is disjoint.
        num_trips = 1;
    } else if num_jokers == 1 && is_quad {
        is_sweep = true;
    } else if num_jokers == 1 && num_trips > 0 {
        is_quad = true;
    } else if num_jokers == 1 && num_pairs == 2 {
        num_trips = 1;
        num_pairs = 1;
    } else if num_jokers == 1 && num_pairs == 1 {
        num_trips = 1;
        num_pairs = 0;
    } else if num_jokers == 1 {
        // high card.
        num_pairs = 1;
    }

    if is_sweep {
        Kind::Sweep
    } else if is_quad {
        Kind::Quad
    } else if num_trips == 1 && num_pairs == 1 {
        Kind::FullHouse
    } else if num_trips == 1 {
        Kind::Triple
    } else if num_pairs == 2 {
        Kind::TwoPair
    } else if num_pairs == 1 {
        Kind::Pair
    } else {
        Kind::HighCard
    }
}

fn parse_hand(s: &str) -> Hand {
    let cards_vec: Vec<usize> = s.chars().map(|c| convert_card(c)).collect();
    let cards = (
        cards_vec[0],
        cards_vec[1],
        cards_vec[2],
        cards_vec[3],
        cards_vec[4],
    );

    Hand {
        kind: hand_of(cards),
        hand: cards,
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut hand_bid_pairs: Vec<(Hand, usize)> = contents
        .lines()
        .map(|line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let hand = parse_hand(tokens[0]);
            let bid = tokens[1].parse::<usize>().unwrap();
            (hand, bid)
        })
        .collect();

    hand_bid_pairs.sort_by(|(hand0, _), (hand1, _)| hand0.cmp(hand1));

    let mut winnings = 0;
    for (i, (_, bid)) in hand_bid_pairs.iter().enumerate() {
        winnings += (i + 1) * bid;
    }

    println!("Winnings: {}", winnings);
}
