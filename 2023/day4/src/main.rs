use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut cards: Vec<(HashSet<u32>, Vec<u32>)> = vec![];

    for line in contents.lines() {
        let tokens = line.split_whitespace().skip(2);
        let mut winning_nums = HashSet::new();
        let mut nums = vec![];
        let mut did_cross = false;
        for s in tokens {
            if s == "|" {
                did_cross = true;
            } else if did_cross {
                nums.push(s.parse::<u32>().unwrap());
            } else {
                winning_nums.insert(s.parse::<u32>().unwrap());
            }
        }

        cards.push((winning_nums, nums));
    }

    let mut card_multipliers: Vec<u32> = vec![1; cards.len()];

    for (i, (winning, nums)) in cards.iter().enumerate() {
        let num_matched = nums.iter().fold(0, |score, num| {
            if winning.contains(num) {
                score + 1
            } else {
                score
            }
        });

        for j in (i + 1)..(i + num_matched + 1) {
            card_multipliers[j] += card_multipliers[i];
        }
    }

    // let mut total_score = 0;
    // for (winning, nums) in cards {
    //     let score = nums.iter().fold(1, |score, num| {
    //         if winning.contains(num) {
    //             score * 2
    //         } else {
    //             score
    //         }
    //     }) / 2;

    //     total_score += score;
    // }

    println!("Num Cards Won: {}", card_multipliers.iter().sum::<u32>());
}
