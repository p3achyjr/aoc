use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let grid: Vec<Vec<usize>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Unknown Char: {}", c),
                })
                .collect()
        })
        .collect();

    let m = grid.len();
    let n = grid[0].len();
    let row_empty: Vec<bool> = grid
        .iter()
        .map(|row| row.iter().sum::<usize>() == 0)
        .collect();
    let mut col_empty = vec![];
    for j in 0..n {
        let mut num_galaxies = 0;
        for i in 0..m {
            num_galaxies += grid[i][j];
        }

        col_empty.push(num_galaxies == 0);
    }

    let mut row_indices: Vec<i64> = vec![];
    let mut offset = 0;
    for (i, empty) in row_empty.iter().enumerate() {
        row_indices.push((i + offset) as i64);
        if *empty {
            offset += 1000000 - 1;
        }
    }

    let mut col_indices: Vec<i64> = vec![];
    let mut offset = 0;
    for (i, empty) in col_empty.iter().enumerate() {
        col_indices.push((i + offset) as i64);
        if *empty {
            offset += 1000000 - 1;
        }
    }

    let mut galaxies = vec![];
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                galaxies.push((i, j));
            }
        }
    }

    let mut sum = 0;
    for gi in 0..galaxies.len() {
        for gj in (gi + 1)..galaxies.len() {
            let (i0, j0) = galaxies[gi];
            let (i1, j1) = galaxies[gj];
            let (i0_index, j0_index) = (row_indices[i0], col_indices[j0]);
            let (i1_index, j1_index) = (row_indices[i1], col_indices[j1]);
            let dist = (i0_index - i1_index).abs() + (j0_index - j1_index).abs();
            sum += dist;
        }
    }

    println!("Sum: {}", sum);
}
