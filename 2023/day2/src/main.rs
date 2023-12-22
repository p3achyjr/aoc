use std::cmp;
use std::fs;

#[derive(Clone, Copy)]
struct Round {
    id: u32,
    num_red: u32,
    num_green: u32,
    num_blue: u32,
}

fn parse_line(line: &Vec<char>) -> Vec<Round> {
    let mut rounds: Vec<Round> = vec![];
    let mut j = 5;
    let mut game_id_str = String::from("");
    while j < line.len() && line[j].is_digit(10) {
        game_id_str.push(line[j]);
        j += 1;
    }

    // j is now the colon
    j += 2;

    let game_id = game_id_str.parse::<u32>().unwrap();
    let mut round = Round {
        id: game_id,
        num_red: 0,
        num_green: 0,
        num_blue: 0,
    };

    while j < line.len() {
        // scan number.
        let mut num_str = String::from("");
        while j < line.len() && line[j].is_digit(10) {
            num_str.push(line[j]);
            j += 1;
        }

        j += 1;

        let mut color_str = String::from("");
        while j < line.len() && line[j] != ',' && line[j] != ';' {
            color_str.push(line[j]);
            j += 1;
        }

        match color_str.as_str() {
            "red" => round.num_red = num_str.parse::<u32>().unwrap(),
            "green" => round.num_green = num_str.parse::<u32>().unwrap(),
            "blue" => round.num_blue = num_str.parse::<u32>().unwrap(),
            _ => panic!("Unknown Color: `{}`\n", color_str),
        }

        if j == line.len() || line[j] == ';' {
            rounds.push(round.clone());
            round = Round {
                id: game_id,
                num_red: 0,
                num_green: 0,
                num_blue: 0,
            };
        }

        j += 2;
    }

    rounds
}

// fn main() {
//     let contents = fs::read_to_string("input.txt").expect("Failed to read file");
//     let mut sum = 0;
//     for line_str in contents.lines() {
//         let line: Vec<char> = line_str.chars().collect();
//         let game = parse_line(&line);
//         if game
//             .iter()
//             .any(|&round| round.num_red > 12 || round.num_green > 13 || round.num_blue > 14)
//         {
//             continue;
//         }

//         sum += game[0].id;
//     }

//     println!("Sum: {}", sum);
// }

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut sum = 0;
    for line_str in contents.lines() {
        let line: Vec<char> = line_str.chars().collect();
        let game = parse_line(&line);
        let min_cubes = game.iter().fold(
            Round {
                id: 0,
                num_red: 0,
                num_green: 0,
                num_blue: 0,
            },
            |min_cubes, &round| Round {
                id: 0,
                num_red: cmp::max(round.num_red, min_cubes.num_red),
                num_green: cmp::max(round.num_green, min_cubes.num_green),
                num_blue: cmp::max(round.num_blue, min_cubes.num_blue),
            },
        );

        sum += min_cubes.num_red * min_cubes.num_green * min_cubes.num_blue;
    }

    println!("Sum: {}", sum);
}
