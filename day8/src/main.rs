use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::fs;

fn gcd(x: usize, y: usize) -> usize {
    let mut gcd = 1;
    for d in 1..cmp::min(x, y) {
        if x % d == 0 && y % d == 0 {
            gcd = d;
        }
    }

    gcd
}

fn lcm(x: usize, y: usize) -> usize {
    (x * y) / gcd(x, y)
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut lines = contents.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let graph_lines = lines.skip(1);
    let graph_line_re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    let mut cur_keys = vec![];

    for line in graph_lines {
        match graph_line_re.captures(line) {
            Some(groups) => {
                let key = groups.get(1).unwrap().as_str();
                let l = groups.get(2).unwrap().as_str();
                let r = groups.get(3).unwrap().as_str();
                graph.insert(String::from(key), (String::from(l), String::from(r)));

                if key.chars().last().unwrap() == 'A' {
                    cur_keys.push(key);
                }
            }
            None => {}
        }
    }

    for (key, (lkey, rkey)) in &graph {
        if lkey.chars().last().unwrap() == 'Z' {
            println!("L prev: {}", key);
        }

        if rkey.chars().last().unwrap() == 'Z' {
            println!("R prev: {}", key);
        }
    }

    let min_hops: Vec<usize> = cur_keys
        .iter()
        .map(|k| {
            let mut key = k.clone();
            let mut num_steps = 0;
            let mut i = 0;
            while !(key.chars().last().unwrap() == 'Z') {
                let (lkey, rkey) = graph.get(key).unwrap();
                key = match instructions[i] {
                    'L' => lkey.as_str(),
                    'R' => rkey.as_str(),
                    _ => panic!("Unknown Instruction: {}", instructions[i]),
                };
                num_steps += 1;
                i = (i + 1) % instructions.len();
            }

            num_steps
        })
        .collect();

    let num_steps = min_hops
        .iter()
        .fold(1, |min_steps, hops| lcm(min_steps, *hops));

    // let mut seen = HashMap::new();
    // while !cur_keys.iter().all(|k| k.chars().last().unwrap() == 'Z') {
    //     // println!("{} {} Keys: {:?}", num_steps, instructions[i], cur_keys);
    //     // if seen.contains_key(&cur_keys) {
    //     //     println!(
    //     //         "Cycle: {:?}, Len: {}",
    //     //         cur_keys,
    //     //         i - seen.get(&cur_keys).unwrap_or(&0)
    //     //     );
    //     // }

    //     // seen.insert(cur_keys.clone(), i);
    //     cur_keys = cur_keys
    //         .iter()
    //         .map(|&cur_key| {
    //             let (lkey, rkey) = graph.get(cur_key).unwrap();
    //             match instructions[i] {
    //                 'L' => lkey.as_str(),
    //                 'R' => rkey.as_str(),
    //                 _ => panic!("Unknown Instruction: {}", instructions[i]),
    //             }
    //         })
    //         .collect();
    //     num_steps += 1;
    //     i = (i + 1) % instructions.len();
    // }

    println!("Num Steps: {}", num_steps);
}
