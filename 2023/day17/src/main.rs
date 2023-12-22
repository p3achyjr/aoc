use std::cmp;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn straight(
    i: i32,
    j: i32,
    num_straight: usize,
    direction: Direction,
) -> (i32, i32, usize, Direction) {
    match direction {
        Direction::Up => (i - 1, j, num_straight + 1, Direction::Up),
        Direction::Down => (i + 1, j, num_straight + 1, Direction::Down),
        Direction::Right => (i, j + 1, num_straight + 1, Direction::Right),
        Direction::Left => (i, j - 1, num_straight + 1, Direction::Left),
    }
}

fn left_turn(i: i32, j: i32, _: usize, direction: Direction) -> (i32, i32, usize, Direction) {
    match direction {
        Direction::Up => (i, j - 1, 1, Direction::Left),
        Direction::Down => (i, j + 1, 1, Direction::Right),
        Direction::Right => (i - 1, j, 1, Direction::Up),
        Direction::Left => (i + 1, j, 1, Direction::Down),
    }
}

fn right_turn(i: i32, j: i32, _: usize, direction: Direction) -> (i32, i32, usize, Direction) {
    match direction {
        Direction::Up => (i, j + 1, 1, Direction::Right),
        Direction::Down => (i, j - 1, 1, Direction::Left),
        Direction::Right => (i + 1, j, 1, Direction::Down),
        Direction::Left => (i - 1, j, 1, Direction::Up),
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Task {
    heat_loss: usize,
    node: (i32, i32, usize, Direction), // (i, j, num_horizontal, dir)
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heat_loss.cmp(&other.heat_loss)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn next_tasks(
    grid: &Vec<Vec<usize>>,
    heat_loss: usize,
    i: i32,
    j: i32,
    num_straight: usize,
    dir: Direction,
) -> Vec<Task> {
    let next_nodes = if num_straight < 4 {
        vec![straight(i, j, num_straight, dir)]
    } else {
        vec![
            straight(i, j, num_straight, dir),
            left_turn(i, j, num_straight, dir),
            right_turn(i, j, num_straight, dir),
        ]
    };

    next_nodes
        .iter()
        .filter_map(|node| {
            let (i, j, num_straight, _) = node;
            if i < &0 || i >= &(grid.len() as i32) || j < &0 || j >= &(grid[0].len() as i32) {
                None
            } else if num_straight > &10 {
                None
            } else {
                Some(Task {
                    heat_loss: heat_loss + grid[*i as usize][*j as usize],
                    node: *node,
                })
            }
        })
        .collect()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let grid: Vec<Vec<usize>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut pq = BinaryHeap::new();
    pq.push(Reverse(Task {
        heat_loss: 0,
        node: (0, 0, 1, Direction::Down),
    }));
    pq.push(Reverse(Task {
        heat_loss: 0,
        node: (0, 0, 1, Direction::Right),
    }));

    let m = grid.len();
    let n = grid[0].len();

    let mut min_heat_loss = usize::MAX;
    let mut seen: HashSet<(i32, i32, usize, Direction)> = HashSet::new();
    while let Some(Reverse(task)) = pq.pop() {
        let heat_loss = task.heat_loss;
        let (i, j, num_straight, dir) = task.node;
        if i as usize == m - 1 && j as usize == n - 1 && num_straight >= 4 {
            // bottom right.
            min_heat_loss = cmp::min(min_heat_loss, heat_loss);
            break;
        }

        if seen.contains(&task.node) {
            continue;
        }

        seen.insert((i, j, num_straight, dir));
        pq.extend(
            next_tasks(&grid, heat_loss, i, j, num_straight, dir)
                .into_iter()
                .map(Reverse),
        );
    }

    println!("Min Heat Loss: {}", min_heat_loss);
}
