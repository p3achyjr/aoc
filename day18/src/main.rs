use std::cmp;
use std::collections::HashSet;
use std::fs;

fn part1(insts: &Vec<(char, i64, String)>) {
    let mut boundary: HashSet<(i64, i64)> = HashSet::from([(0, 0)]);
    let mut cur_coord = (0, 0);
    let mut r0 = 0;
    let mut c0 = 0;
    let mut r1 = 0;
    let mut c1 = 0;
    for (dir, len, _) in insts {
        let (i, j) = cur_coord;
        match dir {
            'U' => {
                for off in 1..(len + 1) {
                    boundary.insert((i - off, j));
                }

                r0 = cmp::min(r0, i - len);
                cur_coord = (i - len, j);
            }
            'D' => {
                for off in 1..(len + 1) {
                    boundary.insert((i + off, j));
                }

                r1 = cmp::max(r1, i + len);
                cur_coord = (i + len, j);
            }
            'L' => {
                for off in 1..(len + 1) {
                    boundary.insert((i, j - off));
                }

                c0 = cmp::min(c0, j - len);
                cur_coord = (i, j - len);
            }
            'R' => {
                for off in 1..(len + 1) {
                    boundary.insert((i, j + off));
                }

                c1 = cmp::max(c1, j + len);
                cur_coord = (i, j + len);
            }
            _ => panic!("Unknown Direction: {}", dir),
        }
    }

    let mut region_size = 0;
    for (i, j) in [(1, 1), (-1, 1), (-1, -1), (-1, 1)] {
        let mut local_region_size = 0;
        let mut stack = vec![(i, j)];
        let mut seen = HashSet::new();
        let mut is_oob = false;
        while let Some((i, j)) = stack.pop() {
            if i < r0 || i > r1 || j < c0 || j > c1 {
                is_oob = true;
                break;
            }

            if boundary.contains(&(i, j)) || seen.contains(&(i, j)) {
                continue;
            }

            local_region_size += 1;
            seen.insert((i, j));
            stack.push((i - 1, j));
            stack.push((i + 1, j));
            stack.push((i, j - 1));
            stack.push((i, j + 1));
        }

        if is_oob {
            continue;
        } else {
            region_size = local_region_size;
            break;
        }
    }

    println!("Region Size: {}", region_size + boundary.len());
}

fn part2(insts: &Vec<(char, i64)>) {
    // I was originally thinking of an algorithm like this:
    //
    // 1. Find all line segments.
    // 2. Sort horizontal line segments from top to bottom.
    // 3. For each horizontal line segment, compare it to all line segments above it,
    //    adding in the area of the partial rectangle they make, subtracting already
    //    counted area.
    //
    // This was too hard so now we just have shoelace :)
    let mut coords: Vec<(i64, i64)> = vec![];
    let mut coord: (i64, i64) = (0, 0);
    let mut boundary_size = 0;
    for (dir, len) in insts {
        boundary_size += len;
        coords.push(coord);
        let (x, y) = coord;
        match dir {
            'U' => coord = (x, y + len),
            'D' => coord = (x, y - len),
            'L' => coord = (x - len, y),
            'R' => coord = (x + len, y),
            _ => panic!("Unknown Direction: {}", dir),
        }
    }

    println!("Coords: {:?}", coords);

    let mut xy_sum = 0;
    let mut yx_sum = 0;
    for i in 0..coords.len() {
        let (x0, y0) = coords[i];
        let (x1, y1) = coords[(i + 1) % coords.len()];
        xy_sum += x0 * y1;
        yx_sum += y0 * x1;
    }

    let area0 = (xy_sum - yx_sum).abs() / 2 + (boundary_size / 2 + 1);
    let area1 = (yx_sum - xy_sum).abs() / 2 + (boundary_size / 2 + 1);
    println!("Area? {} {}", area0, area1);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    // let insts: Vec<(char, i64, String)> = contents
    //     .lines()
    //     .map(|line| {
    //         let parts: Vec<&str> = line.split_whitespace().collect();
    //         (
    //             parts[0].chars().next().unwrap(),
    //             parts[1].parse::<i64>().unwrap(),
    //             String::from(parts[2]),
    //         )
    //     })
    //     .collect();
    let insts: Vec<(char, i64)> = contents
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let hex = &parts[2][2..8];
            let num = i64::from_str_radix(&hex[0..5], 16).unwrap();
            let code = match hex.chars().nth(5).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!("Unknown Inst: {}", hex.chars().nth(5).unwrap()),
            };
            (code, num)
        })
        .collect();

    println!("{:?}", insts);

    part2(&insts);
}
