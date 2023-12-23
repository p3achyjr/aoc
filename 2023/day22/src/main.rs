use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Axis {
    N,
    X,
    Y,
    Z,
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Brick {
    id: i64,
    axis: Axis,
    p0: (u64, u64, u64),
    p1: (u64, u64, u64),
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut bricks: Vec<Brick> = contents
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let parts: Vec<&str> = line.split("~").collect();
            let p0_str = parts[0];
            let p1_str = parts[1];
            let p0_vec: Vec<u64> = p0_str
                .split(",")
                .map(|p| p.parse::<u64>().unwrap())
                .collect();
            let p1_vec: Vec<u64> = p1_str
                .split(",")
                .map(|p| p.parse::<u64>().unwrap())
                .collect();
            let p0 = (p0_vec[0], p0_vec[1], p0_vec[2]);
            let p1 = (p1_vec[0], p1_vec[1], p1_vec[2]);
            let axis = if p0.0 != p1.0 {
                Axis::X
            } else if p0.1 != p1.1 {
                Axis::Y
            } else if p0.2 != p1.2 {
                Axis::Z
            } else {
                Axis::N
            };

            Brick {
                id: i as i64,
                axis,
                p0,
                p1,
            }
        })
        .collect();

    bricks.sort_by(|b0, b1| {
        let (_, _, b0_z0) = b0.p0;
        let (_, _, b1_z0) = b1.p0;

        b0_z0.cmp(&b1_z0)
    });

    let mut x_max = 0;
    let mut y_max = 0;
    for (id, brick) in bricks.iter_mut().enumerate() {
        brick.id = id as i64;
        x_max = cmp::max(x_max, cmp::max(brick.p0.0 + 1, brick.p1.0 + 1));
        y_max = cmp::max(y_max, cmp::max(brick.p0.1 + 1, brick.p1.1 + 1));
    }

    // Part 1.
    let mut grid: Vec<Vec<(u64, i64)>> = vec![vec![(0, -1); y_max as usize]; x_max as usize];
    let mut dep_graph: Vec<HashSet<i64>> = vec![HashSet::new(); bricks.len()];
    let mut inv_dep_graph: Vec<HashSet<i64>> = vec![HashSet::new(); bricks.len()];
    for brick in &bricks {
        let (x0, y0, z0) = brick.p0;
        let (x1, y1, z1) = brick.p1;
        let height = z1 - z0 + 1;

        // what z-coordinate will this brick rest on? It will be the max of the z-coordinates
        // of all the grid sections it intersects with.
        let resting_z = match brick.axis {
            Axis::N => grid[x0 as usize][y0 as usize].0,
            Axis::X => {
                let mut max_z = 0;
                for x in (x0 as usize)..((x1 + 1) as usize) {
                    max_z = cmp::max(max_z, grid[x][y0 as usize].0);
                }

                max_z
            }
            Axis::Y => {
                let mut max_z = 0;
                for y in (y0 as usize)..((y1 + 1) as usize) {
                    max_z = cmp::max(max_z, grid[x0 as usize][y].0);
                }

                max_z
            }
            Axis::Z => grid[x0 as usize][y0 as usize].0,
        };
        // update dependency graph, and heights.
        match brick.axis {
            Axis::X => {
                for x in (x0 as usize)..((x1 + 1) as usize) {
                    let (last_z, last_brick_id) = grid[x][y0 as usize];
                    if last_brick_id != -1 && last_z == resting_z {
                        dep_graph[brick.id as usize].insert(last_brick_id);
                        inv_dep_graph[last_brick_id as usize].insert(brick.id);
                    }
                    grid[x][y0 as usize] = (resting_z + height, brick.id);
                }
            }
            Axis::Y => {
                for y in (y0 as usize)..((y1 + 1) as usize) {
                    let (last_z, last_brick_id) = grid[x0 as usize][y];
                    if last_brick_id != -1 && last_z == resting_z {
                        dep_graph[brick.id as usize].insert(last_brick_id);
                        inv_dep_graph[last_brick_id as usize].insert(brick.id);
                    }
                    grid[x0 as usize][y] = (resting_z + height, brick.id);
                }
            }
            _ => {
                let (last_z, last_brick_id) = grid[x0 as usize][y0 as usize];
                if last_brick_id != -1 && last_z == resting_z {
                    dep_graph[brick.id as usize].insert(last_brick_id);
                    inv_dep_graph[last_brick_id as usize].insert(brick.id);
                }
                grid[x0 as usize][y0 as usize] = (resting_z + height, brick.id);
            }
        };
    }

    // for brick in &bricks {
    //     println!("{:?}", brick);
    // }
    // for brick in &bricks {
    //     println!("{:?}: {:?}", brick.id, inv_dep_graph[brick.id as usize]);
    //     for supported_brick in &inv_dep_graph[brick.id as usize] {
    //         println!(
    //             "\t{:?}: {:?}",
    //             supported_brick, dep_graph[*supported_brick as usize]
    //         );
    //     }
    // }

    let mut num_disintegrable = 0;
    for brick in &bricks {
        let mut all_supported_bricks_sturdy = true;
        for supported_brick in &inv_dep_graph[brick.id as usize] {
            if &dep_graph[*supported_brick as usize].len() == &1 {
                all_supported_bricks_sturdy = false;
            }
        }

        if all_supported_bricks_sturdy {
            // println!("Can Disintegrate: {}", brick.id);
            num_disintegrable += 1;
        }
    }

    println!("Num Disintegratable: {}", num_disintegrable);

    // Part 2.

    // for each brick, add up the subgraphs it will actually cause to fall.
    let mut num_chained_fallen_bricks = 0;
    for brick in &bricks {
        let mut chained_here = HashSet::from([brick.id]);

        // for each brick that depends on this brick...
        for supported_brick in &inv_dep_graph[brick.id as usize] {
            // if this brick is the dependent brick's _sole_ dependency, then
            // removing this brick will cause the dependent brick to fall. Then,
            // from the dependent brick, all bricks depending on _that_ brick,
            // for which that brick is the _sole_ dependency will also fall.
            if &dep_graph[*supported_brick as usize].len() == &1 {
                let mut queue = VecDeque::from([*supported_brick]);
                while let Some(id) = queue.pop_front() {
                    // println!("{}, {:?}", id, chained_here);
                    if chained_here.contains(&id) {
                        continue;
                    }

                    let mut all_deps_removed = true;
                    for dep in &dep_graph[id as usize] {
                        // println!("Dep: {:?}", dep);
                        if !chained_here.contains(dep) {
                            all_deps_removed = false;
                            break;
                        }
                    }

                    if !all_deps_removed {
                        continue;
                    }

                    chained_here.insert(id);
                    for next_id in &inv_dep_graph[id as usize] {
                        queue.push_back(*next_id);
                    }
                }
            }
        }

        num_chained_fallen_bricks += chained_here.len() - 1;
        let mut chained_vec: Vec<_> = chained_here.iter().collect();
        chained_vec.sort();
        println!("Brick ID: {}. Chained: {:?}", brick.id, chained_vec);
        // println!("Brick ID: {}, Chained: {:?}", brick.id, chained_here.len());
    }

    println!("Num Chained: {}", num_chained_fallen_bricks);
}
