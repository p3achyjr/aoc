use std::collections::HashMap;
use std::fs;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Cond {
    U,
    G,
    B,
}

fn num_arrangements(
    memo: &mut HashMap<(usize, usize), usize>,
    springs: &Vec<(Cond, usize, usize)>,
    groups: &Vec<usize>,
    i: usize,
    j: usize,
) -> usize {
    if i >= springs.len() && j == groups.len() {
        return 1;
    } else if i >= springs.len() {
        return 0;
    } else if j == groups.len() && springs[i].0 == Cond::B {
        return 0;
    }

    match memo.get(&(i, j)) {
        Some(&x) => {
            return x;
        }
        None => {}
    }

    if j == groups.len() {
        let arrangements = num_arrangements(memo, springs, groups, i + 1, j);
        memo.insert((i, j), arrangements);
        return arrangements;
    }

    let (cond, broken_before, _) = springs[i];
    let group_size = groups[j];
    if i + group_size > springs.len() {
        return 0;
    }

    if cond == Cond::G {
        let arrangements = num_arrangements(memo, springs, groups, i + 1, j);
        memo.insert((i, j), arrangements);
        return arrangements;
    }

    // can we make a group here?
    let mut can_insert_group_here = true;
    for off in 0..(group_size - broken_before) {
        let (cond, _, broken_after) = springs[i + off];
        let num_contiguous = broken_before + off + broken_after + 1;
        if cond == Cond::G {
            can_insert_group_here = false;
            break;
        } else if num_contiguous > group_size {
            can_insert_group_here = false;
            break;
        }
    }

    if !can_insert_group_here {
        // cannot make the desired group by putting a broken spring here.
        let arrangements = if cond == Cond::U {
            num_arrangements(memo, springs, groups, i + 1, j)
        } else {
            0
        };
        memo.insert((i, j), arrangements);
        return arrangements;
    }

    let arrangements = if cond == Cond::U {
        num_arrangements(memo, springs, groups, i + 1, j)
            + num_arrangements(memo, springs, groups, i + group_size + 1, j + 1)
    } else {
        num_arrangements(memo, springs, groups, i + group_size + 1, j + 1)
    };

    memo.insert((i, j), arrangements);
    arrangements
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let records: Vec<(Vec<Cond>, Vec<usize>)> = contents
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                panic!("Wrong number of parts. {}", parts.len());
            }

            let springs: Vec<Cond> = parts[0]
                .chars()
                .map(|c| match c {
                    '?' => Cond::U,
                    '.' => Cond::G,
                    '#' => Cond::B,
                    _ => panic!("Unknown Condition: {}", c),
                })
                .collect();
            let groups: Vec<usize> = parts[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let mut springs_repeated: Vec<Cond> = vec![];
            let mut groups_repeated: Vec<usize> = vec![];
            for i in 0..5 {
                springs_repeated.extend(&springs);
                groups_repeated.extend(&groups);
                if i != 4 {
                    springs_repeated.push(Cond::U);
                }
            }

            (springs_repeated, groups_repeated)
            // (springs, groups)
        })
        .collect();

    let records: Vec<(Vec<(Cond, usize, usize)>, Vec<usize>)> = records
        .into_iter()
        .map(|(springs, groups)| {
            let mut springs_annotated = vec![];
            for (i, &cond) in springs.iter().enumerate() {
                let mut broken_before = 0;
                let mut broken_after = 0;
                for j in (0..i).rev() {
                    if springs[j] != Cond::B {
                        break;
                    }

                    broken_before += 1;
                }

                for j in (i + 1)..springs.len() {
                    if springs[j] != Cond::B {
                        break;
                    }

                    broken_after += 1;
                }

                springs_annotated.push((cond, broken_before, broken_after));
            }

            (springs_annotated, groups)
        })
        .collect();

    let sum = records
        .iter()
        .map(|(springs, groups)| {
            let mut memo = HashMap::new();
            // println!("{:?}", springs);
            // println!("{:?}", groups);
            let num = num_arrangements(&mut memo, &springs, &groups, 0, 0);
            // println!("{:?}", num);
            num
        })
        .sum::<usize>();

    println!("Sum: {}", sum);
}
