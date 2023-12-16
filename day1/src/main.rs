use std::fs;

// fn main() {
//     let contents = fs::read_to_string("input.txt").expect("Failed to read file");
//     let mut sum = 0;
//     for line in contents.lines() {
//         let mut s = String::from("");
//         for c in line.chars() {
//             if c.is_digit(10) {
//                 s.push(c);
//                 break;
//             }
//         }

//         for c in line.chars().rev() {
//             if c.is_digit(10) {
//                 s.push(c);
//                 break;
//             }
//         }

//         match s.parse::<i32>() {
//             Ok(n) => sum += n,
//             Err(e) => eprintln!("Failed to parse s: {:?}", e),
//         }
//     }

//     println!("{}", sum);
// }

fn main() {
    const DIGIT_STRINGS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut sum = 0;
    for line in contents.lines() {
        let mut s = String::from("");
        let line_chars: Vec<char> = line.chars().collect();
        for (i, c) in line_chars.iter().enumerate() {
            let mut found = false;
            for d in 0..DIGIT_STRINGS.len() {
                let digit_str = DIGIT_STRINGS[d];
                if i + digit_str.len() > line_chars.len() {
                    continue;
                }
                let substr: String = line_chars[i..(i + digit_str.len())].iter().collect();
                // println!(
                //     "forward: {}, {}, {}",
                //     substr,
                //     digit_str,
                //     substr == digit_str
                // );
                if substr == digit_str {
                    s += &d.to_string();
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }
            if c.is_digit(10) {
                s.push(*c);
                break;
            }
        }

        for (_i, c) in line_chars.iter().rev().enumerate() {
            let i = line_chars.len() - 1 - _i;
            let mut found = false;
            for d in 0..DIGIT_STRINGS.len() {
                let digit_str = DIGIT_STRINGS[d];
                if i + digit_str.len() > line_chars.len() {
                    continue;
                }
                let substr: String = line_chars[i..(i + digit_str.len())].iter().collect();
                // println!("backward: {}", substr);
                if substr == digit_str {
                    s += &d.to_string();
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }

            if c.is_digit(10) {
                s.push(*c);
                break;
            }
        }

        // println!("{}, {}", s, line);

        match s.parse::<i32>() {
            Ok(n) => sum += n,
            Err(e) => eprintln!("Failed to parse s: {:?}", e),
        }
    }

    println!("{}", sum);
}
