use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn part1(grid: &Vec<Vec<char>>, start_pos: (i64, i64)) {
    let m = grid.len() as i64;
    let n = grid[0].len() as i64;
    let mut seen = HashSet::new();
    let mut reachable = HashSet::new();
    let mut queue = VecDeque::from([(0, start_pos)]);
    while let Some((depth, coord @ (i, j))) = queue.pop_front() {
        let i_norm = if i >= 0 {
            i % m
        } else {
            let imod = i % m;
            if imod == 0 {
                0
            } else {
                imod + m
            }
        };
        let j_norm = if j >= 0 {
            j % n
        } else {
            let jmod = j % n;
            if jmod == 0 {
                0
            } else {
                jmod + n
            }
        };

        if grid[i_norm as usize][j_norm as usize] == '#' {
            continue;
        }

        if seen.contains(&coord) || depth > 64 {
            continue;
        }

        seen.insert(coord);
        if depth % 2 == 0 {
            reachable.insert(coord);
        }

        queue.push_back((depth + 1, (i - 1, j)));
        queue.push_back((depth + 1, (i + 1, j)));
        queue.push_back((depth + 1, (i, j - 1)));
        queue.push_back((depth + 1, (i, j + 1)));
    }

    println!("Reachable Size: {}", reachable.len());
}

fn opp_parity(parity: i64) -> i64 {
    if parity == 1 {
        0
    } else {
        1
    }
}

fn part2(grid: &Vec<Vec<char>>, start_pos: (i64, i64)) {
    // The grid is open on all axes, and all borders. Thus this reduces to:
    // - Find parity of each reachable square from center.
    // - Find frontier. In our case, the number of steps perfectly ends at the
    //   edge of a square.
    // - For each type of frontier piece, find the number of squares reachable
    //   with the given budget.
    // - Sum them all together.
    const NUM_STEPS: i64 = 26501365;
    let m = grid.len() as i64;
    let n = grid[0].len() as i64;
    let (i0, j0, i1, j1, i2, j2) = (0, 0, m / 2, n / 2, m - 1, n - 1);

    let mut seen = HashSet::new();
    let mut num_odd = 0;
    let mut num_even = 0;
    let mut queue = VecDeque::from([(0, start_pos)]);
    while let Some((depth, coord @ (i, j))) = queue.pop_front() {
        if i < 0 || i >= m || j < 0 || j >= n {
            continue;
        }

        if grid[i as usize][j as usize] == '#' {
            continue;
        }

        if seen.contains(&coord) {
            continue;
        }

        seen.insert(coord);
        if depth % 2 == 0 {
            num_even += 1;
        } else {
            num_odd += 1;
        }

        queue.push_back((depth + 1, (i - 1, j)));
        queue.push_back((depth + 1, (i + 1, j)));
        queue.push_back((depth + 1, (i, j - 1)));
        queue.push_back((depth + 1, (i, j + 1)));
    }

    println!("Parities: {:?}", (num_odd, num_even));

    let man_dist = (NUM_STEPS - (m / 2 + 1)) / m + 1;
    let boundary_parity = (man_dist - 1) % 2;
    println!("Boundary Parity: {:?}", boundary_parity);
    // Parity at straight boundary squares is _odd_. Corner boundary squares
    // is _even_.
    let mut num_boundary: HashMap<(i64, i64), i64> = HashMap::new();
    for coord @ (i, j) in [
        (i0, j0),
        (i0, j1),
        (i0, j2),
        (i1, j0),
        (i1, j2),
        (i2, j0),
        (i2, j1),
        (i2, j2),
    ] {
        let (budget, parity_desired) = if [(i0, j1), (i1, j0), (i1, j2), (i2, j1)].contains(&(i, j))
        {
            // straight line.
            (m - 1, opp_parity(boundary_parity))
        } else {
            (m - 1 + (m / 2), boundary_parity)
        };

        let mut seen = HashSet::new();
        let mut num_reachable = 0;
        let mut queue = VecDeque::from([(0, coord)]);
        while let Some((depth, coord @ (i, j))) = queue.pop_front() {
            if i < 0 || i >= m || j < 0 || j >= n {
                continue;
            }

            if depth > budget {
                continue;
            }

            if grid[i as usize][j as usize] == '#' {
                continue;
            }

            if seen.contains(&coord) {
                continue;
            }

            seen.insert(coord);
            if depth % 2 == parity_desired {
                num_reachable += 1;
            }

            queue.push_back((depth + 1, (i - 1, j)));
            queue.push_back((depth + 1, (i + 1, j)));
            queue.push_back((depth + 1, (i, j - 1)));
            queue.push_back((depth + 1, (i, j + 1)));
        }

        num_boundary.insert(coord, num_reachable);
    }

    println!("Boundary Reachable: {:?}", num_boundary);

    // Parity at one-past boundary is _even_.
    let mut num_sliver: HashMap<(i64, i64), i64> = HashMap::new();
    for coord in [(i0, j0), (i0, j2), (i2, j0), (i2, j2)] {
        let budget = m / 2 - 1;
        let parity_desired = opp_parity(boundary_parity);

        let mut seen = HashSet::new();
        let mut num_reachable = 0;
        let mut queue = VecDeque::from([(0, coord)]);
        while let Some((depth, coord @ (i, j))) = queue.pop_front() {
            if i < 0 || i >= m || j < 0 || j >= n {
                continue;
            }

            if depth > budget {
                continue;
            }

            if grid[i as usize][j as usize] == '#' {
                continue;
            }

            if seen.contains(&coord) {
                continue;
            }

            seen.insert(coord);
            if depth % 2 == parity_desired {
                num_reachable += 1;
            }

            queue.push_back((depth + 1, (i - 1, j)));
            queue.push_back((depth + 1, (i + 1, j)));
            queue.push_back((depth + 1, (i, j - 1)));
            queue.push_back((depth + 1, (i, j + 1)));
        }

        num_sliver.insert(coord, num_reachable);
    }

    println!("Sliver Reachable: {:?}", num_sliver);

    // Figure out out many sliver squares, and fat boundary squares, we have per quadrant.
    // We always have exactly one straight-shot square.
    let num_sliver_squares = man_dist;
    let num_boundary_squares = man_dist - 1;

    // Now figure out how many interior squares we have, and of each parity.
    let mut num_odd_interior_squares: i64 = 1;
    let mut num_even_interior_squares: i64 = 0;
    for d in 0..(man_dist - 1) {
        let is_even = d % 2 == 0;
        let boundary_size = (d + 1) * 4;
        if is_even {
            num_even_interior_squares += boundary_size;
        } else {
            num_odd_interior_squares += boundary_size;
        }
    }

    println!(
        "Odd Squares: {}, Even Squares: {}, Sliver Squares: {}, Boundary Squares: {}, Man Dist: {}",
        num_odd_interior_squares,
        num_even_interior_squares,
        num_sliver_squares,
        num_boundary_squares,
        man_dist,
    );

    // Send it I guess?
    let mut total_plots: i64 = 0;
    total_plots += num_even_interior_squares * num_even;
    total_plots += num_odd_interior_squares * num_odd;
    println!("Total Interior: {}", total_plots);

    // First Quadrant.
    total_plots += num_boundary[&(i2, j1)];
    total_plots += num_boundary[&(i2, j0)] * num_boundary_squares; // lower left
    total_plots += num_sliver[&(i2, j0)] * num_sliver_squares;
    println!("Total First: {}", total_plots);

    // Second Quadrant.
    total_plots += num_boundary[&(i1, j0)];
    total_plots += num_boundary[&(i0, j0)] * num_boundary_squares; // upper left
    total_plots += num_sliver[&(i0, j0)] * num_sliver_squares;
    println!("Total Second: {}", total_plots);

    // Third Quadrant.
    total_plots += num_boundary[&(i0, j1)];
    total_plots += num_boundary[&(i0, j2)] * num_boundary_squares; // upper right
    total_plots += num_sliver[&(i0, j2)] * num_sliver_squares;
    println!("Total Third: {}", total_plots);

    // Fourth Quadrant.
    total_plots += num_boundary[&(i1, j2)];
    total_plots += num_boundary[&(i2, j2)] * num_boundary_squares; // lower right
    total_plots += num_sliver[&(i2, j2)] * num_sliver_squares;

    println!("Total: {}", total_plots);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut start_pos = (-1, -1);
    let grid: Vec<Vec<char>> = contents
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => '.',
                    '#' => '#',
                    'S' => {
                        start_pos = (i as i64, j as i64);
                        '.'
                    }
                    _ => panic!("Invalid Char: {}", c),
                })
                .collect()
        })
        .collect();

    part2(&grid, start_pos);
}
