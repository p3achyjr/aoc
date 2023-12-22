use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Beam {
    Low,
    High,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum ModKind {
    Broad,
    Flip,
    Conj,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Mod {
    kind: ModKind,
    name: String,
}

fn part1(
    mod_map: &HashMap<&str, Mod>,
    graph: &HashMap<&str, Vec<String>>,
    flip_states: &mut HashMap<&str, bool>,
    conj_memory: &mut HashMap<&str, HashMap<&str, Beam>>,
) {
    let mut total_low_beams = 0;
    let mut total_high_beams = 0;
    for _ in 0..1000 {
        let mut worklist = VecDeque::new();
        total_low_beams += 1;
        for init_node in &graph["broadcaster"] {
            worklist.push_back((Beam::Low, "broadcaster", init_node.as_str()));
        }

        while let Some((in_beam, last_node_name, node_name)) = worklist.pop_front() {
            match in_beam {
                Beam::Low => total_low_beams += 1,
                Beam::High => total_high_beams += 1,
            }

            let module = mod_map.get(node_name).unwrap();
            let neighbors = graph.get(node_name).unwrap();
            match module.kind {
                ModKind::Broad => {
                    for neighbor in neighbors {
                        worklist.push_back((in_beam, node_name, neighbor.as_str()));
                    }
                }
                ModKind::Flip => match in_beam {
                    Beam::Low => {
                        let out_beam = if flip_states[node_name] {
                            Beam::Low
                        } else {
                            Beam::High
                        };
                        let flip_state: &mut bool = flip_states.get_mut(node_name).unwrap();
                        *flip_state = !(*flip_state);
                        for neighbor in neighbors {
                            worklist.push_back((out_beam, node_name, neighbor.as_str()));
                        }
                    }
                    Beam::High => {}
                },
                ModKind::Conj => {
                    let memory = conj_memory.get_mut(node_name).unwrap();
                    let cell = memory.get_mut(last_node_name).unwrap();
                    *cell = in_beam;

                    let out_beam = if memory.iter().all(|(_, m)| m == &Beam::High) {
                        Beam::Low
                    } else {
                        Beam::High
                    };

                    for neighbor in neighbors {
                        worklist.push_back((out_beam, node_name, neighbor.as_str()));
                    }
                }
            }
        }
    }

    println!(
        "Low Beams: {}, High Beams: {}, Score: {}",
        total_low_beams,
        total_high_beams,
        total_high_beams * total_low_beams
    );
}

fn dfs<'a>(seen: &mut HashMap<&'a str, i32>, graph: &'a HashMap<&str, Vec<String>>, node: &'a str) {
    match seen.get(node) {
        Some(&0) => {
            println!("Contains Cycle: {}", node);
            return;
        }
        Some(&1) => return,
        _ => (),
    }

    seen.insert(node, 0);
    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            dfs(seen, graph, neighbor);
        }
    }
    seen.insert(node, 1);
}

fn run_sim(
    num_presses: u32,
    saturating_regions: &HashMap<&str, HashSet<&str>>,
    mod_map: &HashMap<&str, Mod>,
    graph: &HashMap<&str, Vec<String>>,
    flip_states: &mut HashMap<&str, bool>,
    conj_memory: &mut HashMap<&str, HashMap<&str, Beam>>,
) {
    let mut worklist = VecDeque::new();
    for init_node in &graph["broadcaster"] {
        worklist.push_back((Beam::Low, "broadcaster", init_node.as_str()));
    }

    while let Some((in_beam, last_node_name, node_name)) = worklist.pop_front() {
        let module = mod_map.get(node_name).unwrap();
        let neighbors = graph.get(node_name).unwrap();
        match module.kind {
            ModKind::Broad => {
                for neighbor in neighbors {
                    worklist.push_back((in_beam, node_name, neighbor.as_str()));
                }
            }
            ModKind::Flip => match in_beam {
                Beam::Low => {
                    let out_beam = if flip_states[node_name] {
                        Beam::Low
                    } else {
                        Beam::High
                    };
                    let flip_state: &mut bool = flip_states.get_mut(node_name).unwrap();
                    *flip_state = !(*flip_state);
                    for neighbor in neighbors {
                        worklist.push_back((out_beam, node_name, neighbor.as_str()));
                    }
                }
                Beam::High => {}
            },
            ModKind::Conj => {
                // if ["nh"].contains(&node_name) {
                //     println!("{}: {:?}", node_name, &conj_memory[node_name]);
                // }

                let memory = conj_memory.get_mut(node_name).unwrap();
                let cell = memory.get_mut(last_node_name).unwrap();
                *cell = in_beam;

                let out_beam = if memory.iter().all(|(_, m)| m == &Beam::High) {
                    Beam::Low
                } else {
                    Beam::High
                };

                // if ["nh"].contains(&node_name) {
                //     println!(
                //         "{}: {:?}, {:?}",
                //         node_name, &conj_memory[node_name], out_beam,
                //     );
                // }

                if ["nh", "mf", "fd", "kb"].contains(&node_name) && out_beam == Beam::Low {
                    println!("{}, {}", num_presses, node_name,);
                }

                for neighbor in neighbors {
                    worklist.push_back((out_beam, node_name, neighbor.as_str()));
                }
            }
        }
    }
}

/*
 * This code does not solve the problem! I stumbled upon the answer by accident.
 *
 * I was going to do the following:
 * - For each sink conjunction node, find its saturating region (i.e. the subgraph it uniquely
 *   depends on). Then, find its lead in + period.
 * - Do some logic to try to solve a system of equations corresponding to when each subgraph
 *   sends a "high" signal.
 *
 * I ended up printing when the end conjunction gates sent "HIGH", and noticed they had a
 * regular period. I plugged it into a LCM calculator online, and it gave me the answer :)
 *
 * I don't really think that it's robust enough--there can be cases where the end gates
 * send multiple signals within one button press, in interleaved order. There's also
 * nothing to suggest that each should have a regular period. But if it works, it works.
 */
fn part2(
    mod_map: &HashMap<&str, Mod>,
    graph: &HashMap<&str, Vec<String>>,
    inv_graph: &HashMap<&str, Vec<String>>,
    flip_states: &mut HashMap<&str, bool>,
    conj_memory: &mut HashMap<&str, HashMap<&str, Beam>>,
) {
    let term_conjs = ["nh", "mf", "fd", "kb"];
    let mut saturating_regions = HashMap::new();
    for conj in term_conjs {
        // find saturating region.
        let mut stack: Vec<&str> = inv_graph[conj].iter().map(|u| u.as_str()).collect();
        let mut saturating_region = HashSet::new();
        while let Some(u) = stack.pop() {
            if saturating_region.contains(&u) {
                continue;
            }

            saturating_region.insert(u);
            if let Some(neighbors) = inv_graph.get(&u) {
                for v in neighbors {
                    stack.push(v);
                }
            }
        }

        saturating_regions.insert(conj, saturating_region);
    }

    for (conj, region) in &saturating_regions {
        println!("{:?}: {:?}, {:?}", conj, region, region.len());
    }

    let mut nh_history: HashMap<(HashMap<&str, bool>, Beam), usize> = HashMap::new();
    let mut mf_history: HashMap<(HashMap<&str, bool>, Beam), usize> = HashMap::new();
    let mut fd_history: HashMap<(HashMap<&str, bool>, Beam), usize> = HashMap::new();
    let mut kb_history: HashMap<(HashMap<&str, bool>, Beam), usize> = HashMap::new();

    let mut num_presses = 0;
    // loop {
    //     num_presses += 1;
    //     run_sim(mod_map, graph, flip_states, conj_memory);

    //     let mut nh_state = HashMap::new();
    //     for node in &saturating_regions["nh"] {

    //     }
    // }

    let mut num_presses = 0;
    loop {
        num_presses += 1;
        // println!("Round {}", num_presses);
        run_sim(
            num_presses,
            &saturating_regions,
            mod_map,
            graph,
            flip_states,
            conj_memory,
        );
        // println!();
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let rules: Vec<(Mod, Vec<String>)> = contents
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split("->").collect();
            let mod_name = parts[0].trim();
            let module = match mod_name.chars().nth(0).unwrap() {
                '&' => Mod {
                    kind: ModKind::Conj,
                    name: String::from(&mod_name[1..]),
                },
                '%' => Mod {
                    kind: ModKind::Flip,
                    name: String::from(&mod_name[1..]),
                },
                _ => Mod {
                    kind: ModKind::Broad,
                    name: String::from("broadcaster"),
                },
            };

            let part1_trimmed = parts[1].trim();
            let neighbors = if part1_trimmed.is_empty() {
                vec![]
            } else {
                part1_trimmed
                    .trim()
                    .split(",")
                    .map(|s| String::from(s.trim()))
                    .collect()
            };

            (module, neighbors)
        })
        .collect();

    let mut mod_map: HashMap<&str, Mod> = HashMap::new();
    let mut graph: HashMap<&str, Vec<String>> = HashMap::new();
    let mut inv_graph: HashMap<&str, Vec<String>> = HashMap::new();
    let mut flip_states: HashMap<&str, bool> = HashMap::new();
    let mut conj_memory: HashMap<&str, HashMap<&str, Beam>> = HashMap::new();

    for (module, neighbors) in &rules {
        mod_map.insert(module.name.as_str(), module.clone());
        graph.insert(module.name.as_str(), neighbors.clone());
        for neighbor in neighbors {
            inv_graph
                .entry(neighbor.as_str())
                .and_modify(|v| v.push(module.name.clone()))
                .or_insert(vec![module.name.clone()]);
        }
        if module.kind == ModKind::Flip {
            flip_states.insert(module.name.as_str(), false);
        }
    }

    for (module, neighbors) in &rules {
        for neighbor in neighbors {
            if !(mod_map.get(neighbor.as_str()).unwrap().kind == ModKind::Conj) {
                continue;
            }

            if !(conj_memory.contains_key(neighbor.as_str())) {
                conj_memory.insert(neighbor.as_str(), HashMap::new());
            }

            let in_nodes: &mut HashMap<&str, Beam> =
                conj_memory.get_mut(neighbor.as_str()).unwrap();
            in_nodes.insert(module.name.as_str(), Beam::Low);
        }
    }

    // part1(&mod_map, &graph, &mut flip_states, &mut conj_memory);
    part2(
        &mod_map,
        &graph,
        &inv_graph,
        &mut flip_states,
        &mut conj_memory,
    );
}
