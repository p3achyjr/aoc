use std::collections::HashMap;
use std::fs;

#[derive(Eq, PartialEq, Clone, Debug)]
enum Action {
    A,
    R,
    N(String),
}

struct Step {
    cond: Option<(char, char, i64)>,
    action: Action,
}

struct Workflow {
    name: String,
    steps: Vec<Step>,
}

fn parse_cond(cond_serialized: &str) -> (char, char, i64) {
    (
        cond_serialized.chars().nth(0).unwrap(),
        cond_serialized.chars().nth(1).unwrap(),
        cond_serialized[2..].parse::<i64>().unwrap(),
    )
}

fn parse_action(action_serialized: &str) -> Action {
    let action_chars: Vec<char> = action_serialized.chars().collect();
    match action_chars[0] {
        'A' => Action::A,
        'R' => Action::R,
        _ => Action::N(String::from(action_serialized)),
    }
}

fn parse_step(wf_serialized: &str) -> Step {
    let parts: Vec<&str> = wf_serialized.split(":").collect();
    if parts.len() == 1 {
        // no condition.
        Step {
            cond: None,
            action: parse_action(parts[0]),
        }
    } else {
        Step {
            cond: Some(parse_cond(parts[0])),
            action: parse_action(parts[1]),
        }
    }
}

fn proj(part: (i64, i64, i64, i64), cat: char) -> i64 {
    match cat {
        'x' => part.0,
        'm' => part.1,
        'a' => part.2,
        's' => part.3,
        _ => panic!("Invalid Index: {}", cat),
    }
}

fn pass_cond((cat, rel, thresh): (char, char, i64), part: (i64, i64, i64, i64)) -> bool {
    match rel {
        '>' => proj(part, cat) > thresh,
        '<' => proj(part, cat) < thresh,
        _ => panic!("Invalid Rel: {}", rel),
    }
}

fn run_workflow(
    workflow_map: &HashMap<String, Workflow>,
    part: (i64, i64, i64, i64),
) -> Option<(i64, i64, i64, i64)> {
    let mut wf_name = "in";
    let accepted_part;
    loop {
        let mut next_action = None;
        let wf = workflow_map.get(wf_name).unwrap();
        for step in &wf.steps {
            match step.cond {
                Some(cond) => {
                    if pass_cond(cond, part) {
                        next_action = Some(&step.action);
                        break;
                    } else {
                        continue;
                    }
                }
                None => {
                    next_action = Some(&step.action);
                    break;
                }
            }
        }

        match next_action {
            Some(Action::A) => {
                accepted_part = Some(part);
                break;
            }
            Some(Action::R) => {
                accepted_part = None;
                break;
            }
            Some(Action::N(next_wf_name)) => wf_name = next_wf_name.as_str(),
            None => panic!("An action should be assigned from workflow: {}", wf_name),
        }
    }

    accepted_part
}

fn part1(workflow_map: &HashMap<String, Workflow>, parts: &Vec<(i64, i64, i64, i64)>) {
    let accepted_parts: Vec<(i64, i64, i64, i64)> = parts
        .iter()
        .filter_map(|part| run_workflow(&workflow_map, *part))
        .collect();
    let scores: Vec<i64> = accepted_parts
        .iter()
        .map(|(x, m, a, s)| x + m + a + s)
        .collect();
    let total_score: i64 = scores.iter().sum();

    println!(
        "Accepted Parts: {:?}\nScores: {:?}\nTotal Score: {:?}",
        accepted_parts, scores, total_score
    );
}

fn constrain_le(
    ((x_lo, x_hi), (m_lo, m_hi), (a_lo, a_hi), (s_lo, s_hi)): (
        (i64, i64),
        (i64, i64),
        (i64, i64),
        (i64, i64),
    ),
    cat: char,
    thresh: i64,
) -> ((i64, i64), (i64, i64), (i64, i64), (i64, i64)) {
    match cat {
        'x' => ((x_lo, thresh), (m_lo, m_hi), (a_lo, a_hi), (s_lo, s_hi)),
        'm' => ((x_lo, x_hi), (m_lo, thresh), (a_lo, a_hi), (s_lo, s_hi)),
        'a' => ((x_lo, x_hi), (m_lo, m_hi), (a_lo, thresh), (s_lo, s_hi)),
        's' => ((x_lo, x_hi), (m_lo, m_hi), (a_lo, a_hi), (s_lo, thresh)),
        _ => panic!("Unknown Code: {}", cat),
    }
}

fn constrain_ge(
    ((x_lo, x_hi), (m_lo, m_hi), (a_lo, a_hi), (s_lo, s_hi)): (
        (i64, i64),
        (i64, i64),
        (i64, i64),
        (i64, i64),
    ),
    cat: char,
    thresh: i64,
) -> ((i64, i64), (i64, i64), (i64, i64), (i64, i64)) {
    match cat {
        'x' => ((thresh, x_hi), (m_lo, m_hi), (a_lo, a_hi), (s_lo, s_hi)),
        'm' => ((x_lo, x_hi), (thresh, m_hi), (a_lo, a_hi), (s_lo, s_hi)),
        'a' => ((x_lo, x_hi), (m_lo, m_hi), (thresh, a_hi), (s_lo, s_hi)),
        's' => ((x_lo, x_hi), (m_lo, m_hi), (a_lo, a_hi), (thresh, s_hi)),
        _ => panic!("Unknown Code: {}", cat),
    }
}

fn constrain_bounds(
    bounds: ((i64, i64), (i64, i64), (i64, i64), (i64, i64)),
    (cat, rel, thresh): (char, char, i64),
) -> (
    ((i64, i64), (i64, i64), (i64, i64), (i64, i64)),
    ((i64, i64), (i64, i64), (i64, i64), (i64, i64)),
) {
    match rel {
        '<' => (
            constrain_le(bounds, cat, thresh - 1),
            constrain_ge(bounds, cat, thresh),
        ),
        '>' => (
            constrain_ge(bounds, cat, thresh + 1),
            constrain_le(bounds, cat, thresh),
        ),
        _ => panic!("Unknown Rel: {}", rel),
    }
}

fn interpret_wf(
    workflow_map: &HashMap<String, Workflow>,
    wf_name: &str,
    bounds: ((i64, i64), (i64, i64), (i64, i64), (i64, i64)),
) -> i64 {
    let mut bounds = bounds.clone();
    let workflow = workflow_map.get(wf_name).unwrap();
    let mut num_ranges = 0;
    for step in &workflow.steps {
        match step.cond {
            Some(cond) => {
                let (bounds_pass, bounds_comp) = constrain_bounds(bounds, cond);
                bounds = bounds_comp;
                num_ranges += find_num_ranges(workflow_map, &step.action, bounds_pass);
            }
            None => {
                num_ranges += find_num_ranges(workflow_map, &step.action, bounds.clone());
            }
        }
    }

    num_ranges
}

fn find_num_ranges(
    workflow_map: &HashMap<String, Workflow>,
    action: &Action,
    bounds @ ((x_lo, x_hi), (m_lo, m_hi), (a_lo, a_hi), (s_lo, s_hi)): (
        (i64, i64),
        (i64, i64),
        (i64, i64),
        (i64, i64),
    ),
) -> i64 {
    if x_hi < x_lo || m_hi < m_lo || a_hi < a_lo || s_hi < s_lo {
        return 0;
    }

    match action {
        Action::A => (x_hi - x_lo + 1) * (m_hi - m_lo + 1) * (a_hi - a_lo + 1) * (s_hi - s_lo + 1),
        Action::R => 0,
        Action::N(wf_name) => interpret_wf(workflow_map, wf_name.as_str(), bounds),
    }
}

fn part2(workflow_map: &HashMap<String, Workflow>) {
    let num_ranges = find_num_ranges(
        workflow_map,
        &Action::N(String::from("in")),
        ((1, 4000), (1, 4000), (1, 4000), (1, 4000)),
    );

    println!("Num Ranges: {}", num_ranges);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let sections: Vec<&str> = contents.split("\n\n").collect();
    let workflows: Vec<Workflow> = sections[0]
        .lines()
        .map(|line| {
            let mut name = String::from("");
            let mut i = 0;
            for c in line.chars() {
                if !c.is_alphabetic() {
                    break;
                }

                name.push(c);
                i += 1;
            }

            let steps = line[(i + 1)..(line.len() - 1)]
                .split(",")
                .map(parse_step)
                .collect();
            Workflow { name, steps }
        })
        .collect();

    let mut workflow_map = HashMap::new();
    for workflow in workflows {
        workflow_map.insert(workflow.name.clone(), workflow);
    }

    let parts: Vec<(i64, i64, i64, i64)> = sections[1]
        .lines()
        .map(|line| {
            let cat_scores: Vec<i64> = line[0..(line.len() - 1)]
                .split(",")
                .map(|cat| {
                    cat.split("=").collect::<Vec<&str>>()[1]
                        .parse::<i64>()
                        .unwrap()
                })
                .collect();
            (cat_scores[0], cat_scores[1], cat_scores[2], cat_scores[3])
        })
        .collect();

    // part1(&workflow_map, &parts);
    part2(&workflow_map);
}
