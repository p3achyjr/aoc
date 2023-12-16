use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let lines: Vec<Vec<i64>> = contents
        .lines()
        .map(|line| {
            let tokens = line.split_whitespace();
            tokens.map(|token| token.parse::<i64>().unwrap()).collect()
        })
        .collect();

    let all_deltas: Vec<Vec<Vec<i64>>> = lines
        .iter()
        .map(|line| {
            let mut line_deltas = vec![line.clone()];
            loop {
                let mut all_deltas_zero = true;
                let mut delta = vec![];
                let last_line_delta = &line_deltas.last().unwrap();
                for i in 0..(line_deltas.last().unwrap().len() - 1) {
                    let d = last_line_delta[i + 1] - last_line_delta[i];
                    delta.push(d);
                    if d != 0 {
                        all_deltas_zero = false;
                    }
                }

                if !delta.is_empty() {
                    line_deltas.push(delta);
                }
                if all_deltas_zero {
                    break;
                }
            }

            line_deltas
        })
        .collect();

    let sum: i64 = all_deltas
        .iter()
        .map(|deltas| {
            deltas
                .iter()
                .map(|line| -> Vec<&i64> { line.iter().rev().collect() })
                .rev()
                .fold(0, |d, line| *line.last().unwrap() - d)
        })
        .sum();

    println!("Sum: {}", sum);
}
