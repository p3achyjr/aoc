use std::cmp;
use std::collections::HashSet;
use std::fs;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

fn left_turn(direction: Dir) -> Dir {
    match direction {
        Dir::Up => Dir::Left,
        Dir::Down => Dir::Right,
        Dir::Right => Dir::Up,
        Dir::Left => Dir::Down,
    }
}

fn right_turn(direction: Dir) -> Dir {
    match direction {
        Dir::Up => Dir::Right,
        Dir::Down => Dir::Left,
        Dir::Right => Dir::Down,
        Dir::Left => Dir::Up,
    }
}

fn next_coord(i: i64, j: i64, direction: Dir) -> (i64, i64, Dir) {
    match direction {
        Dir::Up => (i - 1, j, direction),
        Dir::Down => (i + 1, j, direction),
        Dir::Right => (i, j + 1, direction),
        Dir::Left => (i, j - 1, direction),
    }
}

fn dfs_from(grid: &Vec<Vec<char>>, i: i64, j: i64, direction: Dir) -> usize {
    let mut seen_coords: HashSet<(i64, i64)> = HashSet::new();
    let mut seen: HashSet<(i64, i64, Dir)> = HashSet::new();

    let m = grid.len();
    let n = grid[0].len();
    let mut stack: Vec<(i64, i64, Dir)> = vec![(i, j, direction)];
    while !stack.is_empty() {
        let (i, j, direction) = stack.pop().unwrap();
        if i < 0 || i >= (m as i64) || j < 0 || j >= (n as i64) {
            continue;
        }

        if seen.contains(&(i, j, direction)) {
            continue;
        }

        seen_coords.insert((i, j));
        seen.insert((i, j, direction));

        let ch = grid[i as usize][j as usize];
        match ch {
            '.' => stack.push(next_coord(i, j, direction)),
            '|' => {
                if [Dir::Left, Dir::Right].contains(&direction) {
                    stack.push(next_coord(i, j, left_turn(direction)));
                    stack.push(next_coord(i, j, right_turn(direction)));
                } else {
                    stack.push(next_coord(i, j, direction));
                }
            }
            '-' => {
                if [Dir::Up, Dir::Down].contains(&direction) {
                    stack.push(next_coord(i, j, left_turn(direction)));
                    stack.push(next_coord(i, j, right_turn(direction)));
                } else {
                    stack.push(next_coord(i, j, direction));
                }
            }
            '/' => {
                if [Dir::Left, Dir::Right].contains(&direction) {
                    stack.push(next_coord(i, j, left_turn(direction)));
                } else {
                    stack.push(next_coord(i, j, right_turn(direction)));
                }
            }
            '\\' => {
                if [Dir::Up, Dir::Down].contains(&direction) {
                    stack.push(next_coord(i, j, left_turn(direction)));
                } else {
                    stack.push(next_coord(i, j, right_turn(direction)));
                }
            }
            _ => panic!("Unknown Char: {}", ch),
        }
    }

    seen_coords.len()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut max_energized = 0;
    let m = grid.len();
    let n = grid[0].len();
    for i in 0..m {
        max_energized = cmp::max(max_energized, dfs_from(&grid, i as i64, 0, Dir::Right));
        max_energized = cmp::max(
            max_energized,
            dfs_from(&grid, i as i64, (n - 1) as i64, Dir::Left),
        );
    }

    for j in 0..n {
        max_energized = cmp::max(max_energized, dfs_from(&grid, 0, j as i64, Dir::Down));
        max_energized = cmp::max(max_energized, dfs_from(&grid, (m - 1) as i64, 0, Dir::Up));
    }

    println!("Num Coords: {}", max_energized);
}
