use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

// #[derive(Clone, Copy)]
// struct Node {
//     ch: char,
//     coord: (i64, i64),
//     edges: Option<((i64, i64), (i64, i64))>,
// }

// fn main() {
//     let contents = fs::read_to_string("input.txt").expect("Failed to read file");
//     let mut start: (i64, i64) = (0, 0);
//     let mut grid: Vec<Vec<Node>> = contents
//         .lines()
//         .enumerate()
//         .map(|(i, line)| {
//             let i = i as i64;
//             line.chars()
//                 .enumerate()
//                 .map(|(j, c)| {
//                     let j = j as i64;
//                     match c {
//                         '.' => Node {
//                             ch: c,
//                             coord: (i, j),
//                             edges: None,
//                         },
//                         'S' => {
//                             start = (i, j);
//                             Node {
//                                 ch: c,
//                                 coord: (i, j),
//                                 edges: Some(((-2, -2), (-2, -2))),
//                             }
//                         }
//                         'F' => Node {
//                             ch: c,
//                             coord: (i, j),
//                             edges: Some(((i + 1, j), (i, j + 1))),
//                         },
//                         'J' => Node {
//                             ch: c,
//                             coord: (i, j),
//                             edges: Some(((i - 1, j), (i, j - 1))),
//                         },
//                         'L' => Node {
//                             ch: c,
//                             coord: (i, j),
//                             edges: Some(((i - 1, j), (i, j + 1))),
//                         },
//                         '7' => Node {
//                             ch: c,
//                             coord: (i, j),
//                             edges: Some(((i + 1, j), (i, j - 1))),
//                         },
//                         '|' => Node {
//                             ch: c,
//                             coord: (i, j),
//                             edges: Some(((i - 1, j), (i + 1, j))),
//                         },
//                         '-' => Node {
//                             ch: c,
//                             coord: (i, j),
//                             edges: Some(((i, j - 1), (i, j + 1))),
//                         },
//                         _ => panic!("Unknown Char: {}", c),
//                     }
//                 })
//                 .collect()
//         })
//         .collect();

//     let m = grid.len() as i64;
//     let n = grid[0].len() as i64;
//     grid = grid
//         .iter()
//         .map(|row| {
//             row.iter()
//                 .map(|node| match node.ch {
//                     'S' => {
//                         let mut neighbors = vec![];
//                         let (i, j) = node.coord;
//                         if i - 1 >= 0
//                             && ['|', 'F', '7'].contains(&grid[(i - 1) as usize][j as usize].ch)
//                         {
//                             neighbors.push((i - 1, j));
//                         }

//                         if i + 1 < m
//                             && ['|', 'J', 'L'].contains(&grid[(i + 1) as usize][j as usize].ch)
//                         {
//                             neighbors.push((i + 1, j));
//                         }

//                         if j - 1 >= 0
//                             && ['-', 'F', 'L'].contains(&grid[i as usize][(j - 1) as usize].ch)
//                         {
//                             neighbors.push((i, j - 1));
//                         }

//                         if j + 1 < n
//                             && ['-', '7', 'J'].contains(&grid[i as usize][(j + 1) as usize].ch)
//                         {
//                             neighbors.push((i, j + 1));
//                         }

//                         if neighbors.len() != 2 {
//                             panic!("S has wrong number of neighbors: {}", neighbors.len());
//                         }

//                         Node {
//                             ch: node.ch,
//                             coord: node.coord,
//                             edges: Some((neighbors[0], neighbors[1])),
//                         }
//                     }
//                     _ => node.clone(),
//                 })
//                 .collect()
//         })
//         .collect();

//     let mut bfs_queue = VecDeque::new();
//     let mut cycle = HashSet::new();
//     let mut max_dist = 0;

//     bfs_queue.push_back((0, start.clone()));
//     while !bfs_queue.is_empty() {
//         let (dist, (i, j)) = bfs_queue.pop_front().unwrap();
//         if i < 0 || i >= m || j < 0 || j >= n {
//             continue;
//         }

//         if cycle.contains(&(i, j)) {
//             continue;
//         }

//         max_dist = cmp::max(dist, max_dist);
//         cycle.insert((i, j));
//         match grid[i as usize][j as usize].edges {
//             None => {}
//             Some(((i0, j0), (i1, j1))) => {
//                 bfs_queue.push_back((dist + 1, (i0, j0)));
//                 bfs_queue.push_back((dist + 1, (i1, j1)));
//             }
//         }
//     }

//     println!("Max Distance: {}", max_dist);
// }

#[derive(Clone, Copy)]
struct Node {
    ch: char,
    coord: (i64, i64),
    edges: Option<((i64, i64), (i64, i64))>,
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut start: (i64, i64) = (0, 0);
    let mut grid: Vec<Vec<Node>> = contents
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let i = i as i64;
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    let j = j as i64;
                    match c {
                        '.' => Node {
                            ch: c,
                            coord: (i, j),
                            edges: None,
                        },
                        'S' => {
                            start = (i, j);
                            Node {
                                ch: c,
                                coord: (i, j),
                                edges: Some(((-2, -2), (-2, -2))),
                            }
                        }
                        'F' => Node {
                            ch: c,
                            coord: (i, j),
                            edges: Some(((i + 1, j), (i, j + 1))),
                        },
                        'J' => Node {
                            ch: c,
                            coord: (i, j),
                            edges: Some(((i - 1, j), (i, j - 1))),
                        },
                        'L' => Node {
                            ch: c,
                            coord: (i, j),
                            edges: Some(((i - 1, j), (i, j + 1))),
                        },
                        '7' => Node {
                            ch: c,
                            coord: (i, j),
                            edges: Some(((i + 1, j), (i, j - 1))),
                        },
                        '|' => Node {
                            ch: c,
                            coord: (i, j),
                            edges: Some(((i - 1, j), (i + 1, j))),
                        },
                        '-' => Node {
                            ch: c,
                            coord: (i, j),
                            edges: Some(((i, j - 1), (i, j + 1))),
                        },
                        _ => panic!("Unknown Char: {}", c),
                    }
                })
                .collect()
        })
        .collect();

    let m = grid.len() as i64;
    let n = grid[0].len() as i64;
    grid = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|node| match node.ch {
                    'S' => {
                        let mut neighbors = vec![];
                        let (i, j) = node.coord;
                        if i - 1 >= 0
                            && ['|', 'F', '7'].contains(&grid[(i - 1) as usize][j as usize].ch)
                        {
                            neighbors.push((i - 1, j));
                        }

                        if i + 1 < m
                            && ['|', 'J', 'L'].contains(&grid[(i + 1) as usize][j as usize].ch)
                        {
                            neighbors.push((i + 1, j));
                        }

                        if j - 1 >= 0
                            && ['-', 'F', 'L'].contains(&grid[i as usize][(j - 1) as usize].ch)
                        {
                            neighbors.push((i, j - 1));
                        }

                        if j + 1 < n
                            && ['-', '7', 'J'].contains(&grid[i as usize][(j + 1) as usize].ch)
                        {
                            neighbors.push((i, j + 1));
                        }

                        if neighbors.len() != 2 {
                            panic!("S has wrong number of neighbors: {}", neighbors.len());
                        }

                        let ((i0, j0), (i1, j1)) = (neighbors[0], neighbors[1]);
                        let converted_ch = if i0 == i - 1 && i1 == i + 1 {
                            '|'
                        } else if i0 == i - 1 && j1 == j - 1 {
                            'J'
                        } else if i0 == i - 1 && j1 == j + 1 {
                            'L'
                        } else if i0 == i + 1 && j1 == j - 1 {
                            '7'
                        } else if i0 == i + 1 && j1 == j + 1 {
                            'F'
                        } else if j0 == j - 1 && j1 == j + 1 {
                            '-'
                        } else {
                            panic!(
                                "Unknown Neighbor Configuration. {:?} {:?} {:?}",
                                (i, j),
                                (i0, j0),
                                (i1, j1)
                            );
                        };

                        Node {
                            ch: converted_ch,
                            coord: node.coord,
                            edges: Some((neighbors[0], neighbors[1])),
                        }
                    }
                    _ => node.clone(),
                })
                .collect()
        })
        .collect();

    let mut bfs_queue = VecDeque::new();
    let mut cycle = HashSet::new();
    let mut max_dist = 0;

    bfs_queue.push_back((0, start.clone()));
    while !bfs_queue.is_empty() {
        let (dist, (i, j)) = bfs_queue.pop_front().unwrap();
        if i < 0 || i >= m || j < 0 || j >= n {
            continue;
        }

        if cycle.contains(&(i, j)) {
            continue;
        }

        max_dist = cmp::max(dist, max_dist);
        cycle.insert((i, j));
        match grid[i as usize][j as usize].edges {
            None => {}
            Some(((i0, j0), (i1, j1))) => {
                bfs_queue.push_back((dist + 1, (i0, j0)));
                bfs_queue.push_back((dist + 1, (i1, j1)));
            }
        }
    }

    // println!("Cycle: {:?}", cycle);

    // now expand graph.
    let expanded_grid: Vec<Vec<[[u8; 3]; 3]>> = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|node| match node.ch {
                    '.' => [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
                    'F' => [[0, 0, 0], [0, 1, 1], [0, 1, 0]],
                    'J' => [[0, 1, 0], [1, 1, 0], [0, 0, 0]],
                    'L' => [[0, 1, 0], [0, 1, 1], [0, 0, 0]],
                    '7' => [[0, 0, 0], [1, 1, 0], [0, 1, 0]],
                    '|' => [[0, 1, 0], [0, 1, 0], [0, 1, 0]],
                    '-' => [[0, 0, 0], [1, 1, 1], [0, 0, 0]],
                    _ => panic!("Unknown Char: {}", node.ch),
                })
                .collect()
        })
        .collect();

    let neighbors = |((i, j), (oi, oj)): ((i64, i64), (i64, i64))| match (oi, oj) {
        (0, 0) => vec![
            ((i, j - 1), (0, 2)),
            ((i - 1, j), (2, 0)),
            ((i, j), (0, 1)),
            ((i, j), (1, 0)),
        ],
        (0, 1) => vec![
            ((i, j), (0, 0)),
            ((i - 1, j), (2, 1)),
            ((i, j), (0, 2)),
            ((i, j), (1, 1)),
        ],
        (0, 2) => vec![
            ((i, j), (0, 1)),
            ((i - 1, j), (2, 2)),
            ((i, j + 1), (0, 0)),
            ((i, j), (1, 2)),
        ],
        (1, 0) => vec![
            ((i, j - 1), (1, 2)),
            ((i, j), (0, 0)),
            ((i, j), (1, 1)),
            ((i, j), (2, 0)),
        ],
        (1, 1) => vec![
            ((i, j), (1, 0)),
            ((i, j), (0, 1)),
            ((i, j), (1, 2)),
            ((i, j), (2, 1)),
        ],
        (1, 2) => vec![
            ((i, j), (1, 1)),
            ((i, j), (0, 2)),
            ((i, j + 1), (1, 0)),
            ((i, j), (2, 1)),
        ],
        (2, 0) => vec![
            ((i, j - 1), (2, 2)),
            ((i, j), (1, 0)),
            ((i, j), (2, 1)),
            ((i + 1, j), (0, 0)),
        ],
        (2, 1) => vec![
            ((i, j), (2, 0)),
            ((i, j), (1, 1)),
            ((i, j), (2, 2)),
            ((i + 1, j), (0, 1)),
        ],
        (2, 2) => vec![
            ((i, j), (2, 1)),
            ((i, j), (1, 2)),
            ((i, j + 1), (2, 0)),
            ((i + 1, j), (1, 2)),
        ],
        _ => panic!("Invalid indices: {:?}", (oi, oj)),
    };
    let mut seen_coords = HashSet::new();
    let mut seen = HashSet::new();
    let mut num_enclosed = 0;

    for i in 0..m {
        for j in 0..n {
            if seen_coords.contains(&(i, j)) || cycle.contains(&(i, j)) {
                continue;
            }

            let mut region_size = 0;
            let mut is_enclosed = true;
            let mut stack = vec![((i, j), (1, 1))];
            // println!("-----Round-----");
            while !stack.is_empty() {
                let ((i, j), (oi, oj)) = stack.pop().unwrap();
                if i < 0 || i >= m || j < 0 || j >= n {
                    // println!("OUT OF BOUNDS");
                    is_enclosed = false;
                    continue;
                }

                if seen.contains(&((i, j), (oi, oj))) {
                    continue;
                }

                if !seen_coords.contains(&(i, j)) {
                    if !cycle.contains(&(i, j)) {
                        region_size += 1;
                    }
                    seen_coords.insert((i, j));
                }

                seen.insert(((i, j), (oi, oj)));
                let mut neighbors = if cycle.contains(&(i, j))
                    && expanded_grid[i as usize][j as usize][oi as usize][oj as usize] == 1
                {
                    vec![]
                } else {
                    neighbors(((i, j), (oi, oj)))
                };
                if cycle.contains(&(i, j)) {
                    neighbors = neighbors
                        .iter()
                        .filter_map(|((i, j), (oi, oj))| {
                            if cycle.contains(&(*i, *j)) {
                                if expanded_grid[*i as usize][*j as usize][*oi as usize]
                                    [*oj as usize]
                                    == 1
                                {
                                    None
                                } else {
                                    Some(((*i, *j), (*oi, *oj)))
                                }
                            } else {
                                Some(((*i, *j), (*oi, *oj)))
                            }
                        })
                        .collect();
                }

                stack.extend(neighbors);
            }

            if is_enclosed {
                num_enclosed += region_size;
            }
        }
    }

    println!("Num Enclosed: {}", num_enclosed);
}
