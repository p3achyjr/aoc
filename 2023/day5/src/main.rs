use std::cmp;
use std::fmt;
use std::fs;

struct Mapping {
    dst_start: u64,
    src_start: u64,
    range: u64,
}

impl fmt::Debug for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.dst_start, self.src_start, self.range,
        )
    }
}

// fn in_range(mapping: &Mapping, s: u64) -> bool {
//     s >= mapping.src_start && s < mapping.src_start + mapping.range
// }

// fn find_location(maps: &Vec<Vec<Mapping>>, seed: u64) -> u64 {
//     let mut s = seed;
//     for map in maps {
//         for mapping in map {
//             if in_range(mapping, s) {
//                 let offset = s - mapping.src_start;
//                 s = mapping.dst_start + offset;
//                 break;
//             }
//         }
//     }

//     s
// }

fn min_in_range(maps: &Vec<Vec<Mapping>>, i: usize, (seed, range): (u64, u64)) -> u64 {
    if i == maps.len() {
        // println!("RETURNING SEED: {}", seed);
        return seed;
    }

    // println!("i: {}, range: ({}, {})", i, seed, range);

    let map = &maps[i];
    // find ranges that [seed, seed + range)
    let lo = match map.binary_search_by(|mapping| mapping.src_start.cmp(&seed)) {
        Ok(i) => i,
        Err(i) => i.saturating_sub(1),
    };
    let hi = match map.binary_search_by(|mapping| mapping.src_start.cmp(&(seed + range - 1))) {
        Ok(i) => i,
        Err(i) => i.saturating_sub(1),
    };

    let mut min_mapped_index = u64::MAX;
    let mut last_end = seed;
    for map_index in lo..(hi + 1) {
        let mapping = &map[map_index];
        // println!("mapping: {:?} {:?} {:?}", map_index, map.len(), mapping);
        if last_end < mapping.src_start {
            // unaccounted-for range.
            // println!(
            //     "next range (leftover): {:?}",
            //     (last_end, mapping.src_start - last_end)
            // );
            min_mapped_index = cmp::min(
                min_mapped_index,
                min_in_range(maps, i + 1, (last_end, mapping.src_start - last_end)),
            );
        }

        // past the end.
        if mapping.src_start > seed + range {
            continue;
        }

        // before the beginning.
        if mapping.src_start + mapping.range <= seed {
            continue;
        }

        let mut dst_start = mapping.dst_start;
        if seed > mapping.src_start {
            dst_start += seed - mapping.src_start;
        }

        let mut dst_end = mapping.dst_start + mapping.range;
        if seed + range < mapping.src_start + mapping.range {
            dst_end -= (mapping.src_start + mapping.range) - (seed + range);
        }
        // println!("next range (incl): {:?}", (dst_start, dst_end - dst_start));
        min_mapped_index = cmp::min(
            min_mapped_index,
            min_in_range(maps, i + 1, (dst_start, dst_end - dst_start)),
        );
        last_end = (dst_end - mapping.dst_start) + mapping.src_start;
    }

    if last_end < seed + range {
        // unaccounted-for range.
        // println!(
        //     "next range (end): {:?}",
        //     (last_end, seed + range - last_end)
        // );
        min_mapped_index = cmp::min(
            min_mapped_index,
            min_in_range(maps, i + 1, (last_end, seed + range - last_end)),
        );
    }

    min_mapped_index
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    // let mut seeds: Vec<u64> = vec![];
    let mut seeds: Vec<(u64, u64)> = vec![];
    let mut maps: Vec<Vec<Mapping>> = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    let mut current_map_index = 0;
    for line in contents.lines() {
        if line.starts_with("seeds") {
            let tokens: Vec<u64> = line
                .split_whitespace()
                .skip(1)
                .map(|token| token.parse::<u64>())
                .filter_map(|result| match result {
                    Ok(value) => Some(value),
                    Err(_) => None,
                })
                .collect();
            seeds = tokens
                .chunks(2)
                .map(|window| (window[0], window[1]))
                .collect();
        } else if line.starts_with("seed-to-soil") {
            current_map_index = 0;
        } else if line.starts_with("soil-to-fertilizer") {
            current_map_index = 1;
        } else if line.starts_with("fertilizer-to-water") {
            current_map_index = 2;
        } else if line.starts_with("water-to-light") {
            current_map_index = 3;
        } else if line.starts_with("light-to-temperature") {
            current_map_index = 4;
        } else if line.starts_with("temperature-to-humidity") {
            current_map_index = 5;
        } else if line.starts_with("humidity-to-location") {
            current_map_index = 6;
        } else if line.len() > 0 && line.chars().nth(0).unwrap().is_digit(10) {
            let tokens: Vec<u64> = line
                .split_whitespace()
                .map(|token| token.parse::<u64>())
                .filter_map(|result| match result {
                    Ok(value) => Some(value),
                    Err(_) => None,
                })
                .collect();
            maps[current_map_index].push(Mapping {
                dst_start: tokens[0],
                src_start: tokens[1],
                range: tokens[2],
            });
        }
    }

    // let num_seeds: u64 = seeds.iter().map(|(_, y)| y).sum();
    // println!("Num Seeds: {}", num_seeds);

    // sort all maps by src start.
    for map in &mut maps {
        map.sort_by(|x, y| x.src_start.cmp(&y.src_start));
    }

    // println!("Maps: {:?}", maps);
    let min_location_per_range: Vec<u64> = seeds
        .iter()
        .map(|(seed, range)| min_in_range(&maps, 0, (seed.clone(), range.clone())))
        .collect();

    println!("Ranges: {:?}", seeds);
    println!("Min Locations: {:?}", min_location_per_range);
    println!("Min Location: {:?}", min_location_per_range.iter().min());

    // let locations: Vec<u64> = seeds
    //     .iter()
    //     .map(|seed| find_location(&maps, seed.clone()))
    //     .collect();
    // let min_location: Option<&u64> = locations.iter().min();
    // println!("Min: {:?}", locations);
    // println!("Min: {:?}", min_location);
}
