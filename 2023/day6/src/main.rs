use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let lines: Vec<&str> = contents.lines().collect();
    let times: Vec<&str> = lines[0].split_whitespace().skip(1).collect();
    let dists: Vec<&str> = lines[1].split_whitespace().skip(1).collect();
    let time = times.join("").parse::<i64>().unwrap();
    let dist = dists.join("").parse::<i64>().unwrap();

    let mut begin = 0;
    for t in 0..(time as usize) {
        let t = t as i64;
        let d = t * (time - t);
        if d > dist {
            begin = t;
            break;
        }
    }

    let mut end = 0;
    for t in (0..(time as usize)).rev() {
        let t = t as i64;
        let d = t * (time - t);
        if d > dist {
            end = t;
            break;
        }
    }

    // println!("Time: {} Dist: {} Range: ({}, {})", time, dist, begin, end);

    let ways = end - begin + 1;

    println!("Ways: {}", ways);

    // let times: Vec<i32> = lines[0]
    //     .split_whitespace()
    //     .skip(1)
    //     .map(|s| s.parse::<i32>().unwrap())
    //     .collect();
    // let dists: Vec<i32> = lines[1]
    //     .split_whitespace()
    //     .skip(1)
    //     .map(|s| s.parse::<i32>().unwrap())
    //     .collect();

    // let mut num_ways = 1;
    // for (time, dist) in times.iter().zip(dists.iter()) {
    //     let mut begin = 0;
    //     for t in 0..(*time as usize) {
    //         let t = t as i32;
    //         let d = t * (time - t);
    //         if d > *dist {
    //             begin = t;
    //             break;
    //         }
    //     }

    //     let mut end = 0;
    //     for t in (0..(*time as usize)).rev() {
    //         let t = t as i32;
    //         let d = t * (time - t);
    //         if d > *dist {
    //             end = t;
    //             break;
    //         }
    //     }

    //     // println!("Time: {} Dist: {} Range: ({}, {})", time, dist, begin, end);

    //     let ways = end - begin + 1;
    //     num_ways *= ways;
    // }

    // println!("Ways: {}", num_ways);
}
