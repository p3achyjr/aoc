use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn grid_str(grid: &Vec<Vec<char>>) -> String {
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn north(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| if *c == '#' { '#' } else { '.' })
                .collect()
        })
        .collect();

    for j in 0..grid[0].len() {
        let mut stone_positions: Vec<usize> = vec![];
        let mut pos_queue: VecDeque<usize> = VecDeque::new();
        for i in 0..grid.len() {
            match grid[i][j] {
                'O' => match pos_queue.pop_front() {
                    None => stone_positions.push(i),
                    Some(pos) => {
                        stone_positions.push(pos);
                        pos_queue.push_back(i);
                    }
                },
                '.' => pos_queue.push_back(i),
                '#' => pos_queue = VecDeque::new(),
                _ => panic!("Unknown Char. {}", grid[i][j]),
            }
        }

        for pos in stone_positions {
            new_grid[pos][j] = 'O';
        }
    }

    new_grid
}

fn south(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| if *c == '#' { '#' } else { '.' })
                .collect()
        })
        .collect();

    for j in 0..grid[0].len() {
        let mut stone_positions: Vec<usize> = vec![];
        let mut pos_queue: VecDeque<usize> = VecDeque::new();
        for i in (0..grid.len()).rev() {
            match grid[i][j] {
                'O' => match pos_queue.pop_front() {
                    None => stone_positions.push(i),
                    Some(pos) => {
                        stone_positions.push(pos);
                        pos_queue.push_back(i);
                    }
                },
                '.' => pos_queue.push_back(i),
                '#' => pos_queue = VecDeque::new(),
                _ => panic!("Unknown Char. {}", grid[i][j]),
            }
        }

        for pos in stone_positions {
            new_grid[pos][j] = 'O';
        }
    }

    new_grid
}

fn west(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| if *c == '#' { '#' } else { '.' })
                .collect()
        })
        .collect();

    for i in 0..grid.len() {
        let mut stone_positions: Vec<usize> = vec![];
        let mut pos_queue: VecDeque<usize> = VecDeque::new();
        for j in 0..grid[0].len() {
            match grid[i][j] {
                'O' => match pos_queue.pop_front() {
                    None => stone_positions.push(j),
                    Some(pos) => {
                        stone_positions.push(pos);
                        pos_queue.push_back(j);
                    }
                },
                '.' => pos_queue.push_back(j),
                '#' => pos_queue = VecDeque::new(),
                _ => panic!("Unknown Char. {}", grid[i][j]),
            }
        }

        for pos in stone_positions {
            new_grid[i][pos] = 'O';
        }
    }

    new_grid
}

fn east(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| if *c == '#' { '#' } else { '.' })
                .collect()
        })
        .collect();

    for i in 0..grid.len() {
        let mut stone_positions: Vec<usize> = vec![];
        let mut pos_queue: VecDeque<usize> = VecDeque::new();
        for j in (0..grid[0].len()).rev() {
            match grid[i][j] {
                'O' => match pos_queue.pop_front() {
                    None => stone_positions.push(j),
                    Some(pos) => {
                        stone_positions.push(pos);
                        pos_queue.push_back(j);
                    }
                },
                '.' => pos_queue.push_back(j),
                '#' => pos_queue = VecDeque::new(),
                _ => panic!("Unknown Char. {}", grid[i][j]),
            }
        }

        for pos in stone_positions {
            new_grid[i][pos] = 'O';
        }
    }

    new_grid
}

fn cycle(grid: &Vec<Vec<char>>) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    // println!("FIRST:\n{}\n", grid_str(grid));
    let grid = north(grid);
    // println!("NORTH:\n{}\n", grid_str(&grid));
    let grid = west(&grid);
    // println!("WEST:\n{}\n", grid_str(&grid));
    let grid = south(&grid);
    // println!("SOUTH:\n{}\n", grid_str(&grid));
    let grid = east(&grid);
    // println!("EAST:\n{}\n", grid_str(&grid));

    let mut stone_positions = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                stone_positions.push((i, j));
            }
        }
    }

    (grid, stone_positions)
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut seen: HashMap<Vec<(usize, usize)>, usize> = HashMap::new();
    let mut pos_history: Vec<Vec<(usize, usize)>> = vec![];
    for i in 0..1000000000 {
        // println!("{}\n", grid_str(&grid));
        let (new_grid, positions) = cycle(&grid);
        match seen.get(&positions) {
            None => {
                grid = new_grid;
                seen.insert(positions.clone(), i);
                pos_history.push(positions);
            }
            Some(last_pos) => {
                // we have found a cycle.
                let cycle_len = i - last_pos;
                let num_steps_remaining = 1000000000 - last_pos;
                let cycle_index = num_steps_remaining % cycle_len;
                let positions = &pos_history[last_pos + cycle_index - 1];
                let load = positions.iter().map(|(i, _)| grid.len() - i).sum::<usize>();
                println!(
                    "Cycle Found: {} <-> {}, |C|: {}, cycle_index: {}, index: {}",
                    last_pos,
                    i,
                    cycle_len,
                    cycle_index,
                    last_pos + cycle_index
                );
                println!("Load After 1000000000: {}", load);
                break;
            }
        }
    }

    // println!("Load: {}", load);
}
