use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn tarjan_dfs<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    nums: &mut HashMap<&'a str, (u64, u64)>,
    p: &'a str,
    u: &'a str,
    t: u64,
) {
    if nums.contains_key(&u) {
        return;
    }

    nums.insert(u, (t, t));
    for v in graph.get(&u).unwrap() {
        if *v == p {
            continue;
        }

        tarjan_dfs(graph, nums, u, v, t + 1);
    }

    // update low.
    let mut low = t;
    for v in graph.get(&u).unwrap() {
        if *v == p {
            continue;
        }

        let (v_disc, v_low) = nums.get(v).unwrap();
        if v_disc < &t {
            // back edge.
            low = low.min(*v_disc);
        }

        low = low.min(*v_low);
    }

    nums.insert(u, (t, low));
}

fn tarjan<'a>(
    graph: &'a HashMap<&'a str, Vec<&'a str>>,
    src: &'a str,
) -> HashSet<(&'a str, &'a str)> {
    let mut nums = HashMap::new();
    tarjan_dfs(graph, &mut nums, "", src, 0);

    let mut bridges = HashSet::new();
    for (u, vs) in graph {
        let (u_disc, u_low) = nums.get(u).unwrap();
        for v in vs {
            let (v_disc, v_low) = nums.get(v).unwrap();
            if v_low > u_disc || u_low > v_disc {
                bridges.insert((*u, *v));
            }
        }
    }

    bridges
}

fn total_distance(graph: &HashMap<&str, HashSet<&str>>, src: &str) -> u64 {
    let mut distance = 0;
    let mut queue = VecDeque::from([(0, src)]);
    let mut seen = HashSet::new();
    while let Some((d, u)) = queue.pop_front() {
        if seen.contains(u) {
            continue;
        }

        distance += d;
        seen.insert(u);
        for v in graph.get(&u).unwrap() {
            queue.push_back((d + 1, v));
        }
    }

    distance
}

fn find_component_sizes(
    graph: &HashMap<&str, HashSet<&str>>,
    removed_edges: &Vec<(&str, &str)>,
) -> (u64, u64) {
    let mut seen = HashSet::new();
    let src = graph.iter().next().unwrap().0;
    let mut stack = vec![*src];
    let mut first_size = 0;
    while let Some(u) = stack.pop() {
        if seen.contains(&u) {
            continue;
        }

        first_size += 1;
        seen.insert(u);
        for v in graph.get(u).unwrap() {
            if removed_edges.contains(&(u, v)) || removed_edges.contains(&(v, u)) {
                continue;
            }

            stack.push(v);
        }
    }

    (first_size, graph.len() as u64 - first_size)
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut src = "";

    for line in contents.lines() {
        let parts = line.split(":").collect::<Vec<&str>>();
        let u = parts[0];
        let vs = parts[1].split_whitespace().collect::<Vec<&str>>();

        graph.entry(&u).or_insert(HashSet::new()).extend(&vs);
        for v in &vs {
            graph.entry(v).or_insert(HashSet::new()).insert(&u);
        }

        if src.is_empty() {
            src = u;
        }
    }

    let mut biggest_edges = BinaryHeap::new();
    let mut seen_edges = HashSet::new();
    for (u, vs) in &graph {
        let mut new_graph = graph.clone();
        for v in vs {
            if seen_edges.contains(&(u, v)) || seen_edges.contains(&(v, u)) {
                continue;
            }

            new_graph.entry(&u).and_modify(|vs| {
                vs.remove(v);
            });
            new_graph.entry(&v).and_modify(|us| {
                us.remove(u);
            });

            let distance = total_distance(&graph, u);
            let new_distance = total_distance(&new_graph, u);
            let diff = new_distance - distance;
            // println!(
            //     "({}, {}), Distance: {}, New Distance: {}, Difference: {}",
            //     u, v, distance, new_distance, diff
            // );

            biggest_edges.push((diff, (*u, *v)));
            // println!("Graph: {:?}", graph);
            // println!("New Graph: {:?}", new_graph);

            new_graph.entry(&u).and_modify(|vs| {
                vs.insert(v);
            });
            new_graph.entry(&v).and_modify(|us| {
                us.insert(u);
            });

            seen_edges.insert((u, v));
            seen_edges.insert((v, u));
        }
    }

    println!("Heap: {:?}", biggest_edges);

    let mut cand_edges = vec![];
    cand_edges.push(biggest_edges.pop().unwrap().1);
    cand_edges.push(biggest_edges.pop().unwrap().1);
    cand_edges.push(biggest_edges.pop().unwrap().1);
    println!("{:?}", cand_edges);

    let (mut size, mut comp_size) = find_component_sizes(&graph, &cand_edges);
    if size == 0 || comp_size == 0 {
        loop {
            cand_edges.push(biggest_edges.pop().unwrap().1);
            let mut found = false;
            for i in 0..(cand_edges.len() - 1) {
                for j in i..(cand_edges.len() - 1) {
                    let new_cand_edges =
                        vec![cand_edges[i], cand_edges[j], *cand_edges.last().unwrap()];
                    (size, comp_size) = find_component_sizes(&graph, &new_cand_edges);
                    println!("Temp Sizes: {:?}", (size, comp_size));
                    if size != 0 && comp_size != 0 {
                        found = true;
                        break;
                    }
                }

                if found {
                    break;
                }
            }

            if found {
                break;
            }
        }
    }

    println!("Component Sizes: {:?}", (size, comp_size));
}
