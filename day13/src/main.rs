use std::cmp;
use std::fs;

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

fn reflection_score(grid: &Vec<Vec<char>>) -> usize {
    let mut horizontal_axes: Vec<usize> = vec![];
    for i in 0..(grid.len() - 1) {
        let num_rows_to_check = cmp::min(i + 1, grid.len() - i - 1);
        let mut is_match = true;
        for off in 0..num_rows_to_check {
            if grid[i - off] != grid[i + off + 1] {
                is_match = false;
                break;
            }
        }

        if is_match {
            horizontal_axes.push(i + 1);
        }
    }

    let mut vertical_axes: Vec<usize> = vec![];
    for j in 0..(grid[0].len() - 1) {
        let num_cols_to_check = cmp::min(j + 1, grid[0].len() - j - 1);
        let mut is_match = true;
        for off in 0..num_cols_to_check {
            // loop through entire column.
            for i in 0..grid.len() {
                if grid[i][j - off] != grid[i][j + off + 1] {
                    is_match = false;
                    break;
                }
            }

            if !is_match {
                break;
            }
        }

        if is_match {
            vertical_axes.push(j + 1);
        }
    }

    vertical_axes.iter().sum::<usize>() + 100 * horizontal_axes.iter().sum::<usize>()
}

fn smudge_score(grid: &Vec<Vec<char>>) -> usize {
    // print_grid(grid);
    for i in 0..(grid.len() - 1) {
        let num_rows_to_check = cmp::min(i + 1, grid.len() - i - 1);
        let mut diffs = 0;
        for off in 0..num_rows_to_check {
            for j in 0..(grid[0].len()) {
                if grid[i - off][j] != grid[i + off + 1][j] {
                    diffs += 1;
                    if diffs > 1 {
                        break;
                    }
                }
            }

            if diffs > 1 {
                break;
            }
        }

        if diffs == 1 {
            return 100 * (i + 1);
        }
    }

    for j in 0..(grid[0].len() - 1) {
        let num_cols_to_check = cmp::min(j + 1, grid[0].len() - j - 1);
        let mut diffs = 0;
        for off in 0..num_cols_to_check {
            // loop through entire column.
            for i in 0..grid.len() {
                if grid[i][j - off] != grid[i][j + off + 1] {
                    diffs += 1;
                    if diffs > 1 {
                        break;
                    }
                }
            }

            if diffs > 1 {
                break;
            }
        }

        if diffs == 1 {
            return j + 1;
        }
    }

    // println!(
    //     "Horizontal: {:?}, Vertical: {:?}",
    //     horizontal_axes, vertical_axes
    // );
    print_grid(grid);
    panic!("Did not find a smudge.");
}

fn main() {
    let contents = fs::read_to_string("test_input.txt").expect("Failed to read file");
    let grids: Vec<Vec<Vec<char>>> = contents
        .split("\n\n")
        .map(|block| block.lines().map(|line| line.chars().collect()).collect())
        .collect();

    let sum: usize = grids.iter().map(|grid| smudge_score(&grid)).sum();

    println!("Sum: {}", sum);
}
