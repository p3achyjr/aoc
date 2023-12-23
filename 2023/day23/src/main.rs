use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn find_longest_path(
    seen: &mut HashSet<(i64, i64)>,
    grid: &Vec<Vec<char>>,
    pos @ (i, j): (i64, i64),
) -> i64 {
    if i < 0 || i >= grid.len() as i64 || j < 0 || j >= grid[0].len() as i64 {
        return i64::MIN;
    }

    if i as usize == grid.len() - 1 {
        return 0;
    }

    if seen.contains(&pos) {
        return i64::MIN;
    }

    seen.insert(pos);
    let longest = match grid[i as usize][j as usize] {
        '#' => i64::MIN,
        _ => {
            1 + *[
                find_longest_path(seen, grid, (i - 1, j)),
                find_longest_path(seen, grid, (i + 1, j)),
                find_longest_path(seen, grid, (i, j - 1)),
                find_longest_path(seen, grid, (i, j + 1)),
            ]
            .iter()
            .max()
            .unwrap()
        }
    };
    seen.remove(&pos);

    // if longest > 0 {
    //     println!("{:?}: {:?}", pos, longest);
    // }
    longest
}

fn find_longest_strided(
    seen: &mut HashSet<(i64, i64)>,
    grid: &Vec<Vec<char>>,
    distances_between_open_coords: &HashMap<(i64, i64), HashMap<(i64, i64), i64>>,
    coord @ (i, j): (i64, i64),
) -> i64 {
    if seen.contains(&coord) {
        return i64::MIN;
    }

    if i as usize == grid.len() - 1 {
        return 0;
    }

    seen.insert(coord);
    let mut max_distance = i64::MIN;
    if let Some(distances) = distances_between_open_coords.get(&coord) {
        for (next_coord, distance) in distances {
            max_distance = cmp::max(
                max_distance,
                distance
                    + find_longest_strided(seen, grid, distances_between_open_coords, *next_coord),
            );
        }
    }

    // println!("{:?}, {:?}", coord, max_distance);
    seen.remove(&coord);

    max_distance
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let start_pos = (0, 1);
    let end_pos = {
        let mut pos = (0, 0);
        for j in 0..grid[0].len() {
            if grid[grid.len() - 1][j] == '.' {
                pos = ((grid.len() - 1) as i64, j as i64);
            }
        }

        pos
    };

    let mut open_coords = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                continue;
            }

            let (i, j) = (i as i64, j as i64);
            let mut num_open = 0;
            for (i0, j0) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                if i0 < 0 || i0 as usize >= grid.len() || j0 < 0 || j0 as usize >= grid[0].len() {
                    continue;
                }

                if grid[i0 as usize][j0 as usize] != '#' {
                    num_open += 1;
                }
            }

            if num_open > 2 {
                open_coords.insert((i, j));
            }
        }
    }

    open_coords.insert(start_pos);
    open_coords.insert(end_pos);

    println!("Open Coords: {:?}", open_coords);
    let mut distances_between_open_coords: HashMap<(i64, i64), HashMap<(i64, i64), i64>> =
        HashMap::new();

    for coord @ (i, j) in &open_coords {
        // dfs to neighboring open coords.
        distances_between_open_coords.insert(*coord, HashMap::new());
        let mut seen = HashSet::new();
        let mut stack = vec![(0, (*i, *j))];
        while let Some((d, (i, j))) = stack.pop() {
            if &(i, j) != coord && open_coords.contains(&(i, j)) {
                // reached next open coord.
                if distances_between_open_coords[coord].contains_key(&(i, j)) {
                    let cur_distance = distances_between_open_coords[coord][&(i, j)];
                    distances_between_open_coords
                        .get_mut(coord)
                        .unwrap()
                        .insert((i, j), cmp::max(cur_distance, d));
                } else {
                    distances_between_open_coords
                        .get_mut(coord)
                        .unwrap()
                        .insert((i, j), d);
                }

                continue;
            }

            seen.insert((i, j));
            for (ni, nj) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                if ni < 0 || ni >= grid.len() as i64 || nj < 0 || nj > grid.len() as i64 {
                    continue;
                }

                if grid[ni as usize][nj as usize] == '#' {
                    continue;
                }

                if seen.contains(&(ni, nj)) {
                    continue;
                }

                stack.push((d + 1, (ni, nj)));
            }
        }
    }

    for (coord, distances) in &distances_between_open_coords {
        println!("{:?}: {:?}", coord, distances);
    }

    let longest_path = find_longest_strided(
        &mut HashSet::new(),
        &grid,
        &distances_between_open_coords,
        start_pos,
    );

    // let longest_path = find_longest_path(&mut HashSet::new(), &grid, start_pos);

    println!("Longest Path: {}", longest_path);
}
