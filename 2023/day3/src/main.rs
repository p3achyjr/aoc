use std::collections::HashMap;
use std::fs;

fn is_adjacent(grid: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    for oi in -1..2 {
        for oj in -1..2 {
            if i + oi < 0
                || i + oi >= (grid.len() as i32)
                || j + oj < 0
                || j + oj >= (grid[0].len() as i32)
            {
                continue;
            }

            let ii: usize = (i + oi) as usize;
            let jj: usize = (j + oj) as usize;
            if grid[ii][jj] != '.' && !grid[ii][jj].is_digit(10) {
                return true;
            }
        }
    }

    return false;
}

fn adj_nums(map: &HashMap<(usize, usize), u32>, grid: &Vec<Vec<char>>, i: i32, j: i32) -> Vec<u32> {
    let mut adj = vec![];
    for oi in -1..2 {
        for oj in -1..2 {
            if i + oi < 0
                || i + oi >= (grid.len() as i32)
                || j + oj < 0
                || j + oj >= (grid[0].len() as i32)
            {
                continue;
            }

            let ii: usize = (i + oi) as usize;
            let jj: usize = (j + oj) as usize;
            if grid[ii][jj].is_digit(10) && !adj.contains(&map[&(ii, jj)]) {
                adj.push(map[&(ii, jj)]);
            }
        }
    }

    return adj;
}

// fn main() {
//     let contents = fs::read_to_string("input.txt").expect("Failed to read file");
//     let lines: Vec<&str> = contents.lines().collect();
//     let grid: Vec<Vec<char>> = lines.iter().map(|&line| line.chars().collect()).collect();

//     let mut sum = 0;
//     for i in 0..grid.len() {
//         let mut j = 0;
//         while j < grid[0].len() {
//             if !grid[i][j].is_digit(10) {
//                 j += 1;
//                 continue;
//             }

//             let mut num = String::from("");
//             let mut is_adj = false;
//             while j < grid[0].len() && grid[i][j].is_digit(10) {
//                 num.push(grid[i][j]);
//                 is_adj = is_adj || is_adjacent(&grid, i as i32, j as i32);
//                 j += 1;
//             }

//             if is_adj {
//                 sum += num.parse::<u32>().unwrap();
//             }
//         }
//     }

//     println!("Sum: {}", sum);
// }

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let lines: Vec<&str> = contents.lines().collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|&line| line.chars().collect()).collect();

    let mut sum = 0;
    let mut num_map = HashMap::new();
    for i in 0..grid.len() {
        let mut j = 0;
        while j < grid[0].len() {
            if !grid[i][j].is_digit(10) {
                j += 1;
                continue;
            }

            let mut js = vec![];
            let mut num = String::from("");
            while j < grid[0].len() && grid[i][j].is_digit(10) {
                num.push(grid[i][j]);
                js.push(j);
                j += 1;
            }

            for j in js {
                num_map.insert((i, j), num.parse::<u32>().unwrap());
            }
        }
    }

    // println!("{:?}", num_map);

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !(grid[i][j] == '*') {
                continue;
            }

            let adj = adj_nums(&num_map, &grid, i as i32, j as i32);
            // println!("{} {} {:?}", i, j, adj);
            if adj.len() != 2 {
                continue;
            }

            let gear_ratio = adj[0] * adj[1];
            sum += gear_ratio;
        }
    }

    println!("Sum: {}", sum);
}
