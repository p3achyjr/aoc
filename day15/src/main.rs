use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::rc::Rc;

struct LLNode {
    focal_len: usize,
    prev: Option<Rc<RefCell<LLNode>>>,
    next: Option<Rc<RefCell<LLNode>>>,
}

fn list_str(head: &Option<Rc<RefCell<LLNode>>>) -> String {
    let mut s = String::from("");
    let mut mb_node = head.clone();
    while let Some(node) = mb_node {
        s.extend(node.borrow().focal_len.to_string().chars());
        s.extend(" -> ".chars());
        mb_node = node.borrow().next.clone();
    }

    s.extend("<>".chars());
    s
}

impl fmt::Debug for LLNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:?}) -> {:?} -> ({:?})",
            node_val(&self.prev),
            self.focal_len,
            node_val(&self.next)
        )
    }
}

fn node_val(node: &Option<Rc<RefCell<LLNode>>>) -> Option<usize> {
    match node {
        None => None,
        Some(node) => Some(node.borrow().focal_len),
    }
}

fn hash(id: &Vec<char>) -> usize {
    id.iter().fold(0, |h, c| ((h + (*c as usize)) * 17) % 256)
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let insts: Vec<&str> = contents.split(",").collect();
    let mut boxes: Vec<(
        HashMap<String, Rc<RefCell<LLNode>>>,
        Option<Rc<RefCell<LLNode>>>,
        Option<Rc<RefCell<LLNode>>>,
    )> = (0..256).map(|_| (HashMap::new(), None, None)).collect();

    for inst in insts {
        let inst: Vec<char> = inst.chars().collect();
        let mut id = vec![];
        let mut i = 0;
        while i < inst.len() && inst[i].is_alphabetic() {
            id.push(inst[i]);
            i += 1;
        }

        let h = hash(&id);
        let id: String = id.iter().collect();
        let (map, head, tail) = &mut boxes[h];
        match inst[i] {
            '-' => match map.get(&id) {
                None => {}
                Some(node) => {
                    // delete from list.
                    match &node.borrow().prev {
                        None => {
                            // this must be the head of the list.
                            match &node.borrow().next {
                                None => {
                                    *head = None;
                                    *tail = None;
                                }
                                Some(next_node) => {
                                    *head = Some(Rc::clone(&next_node));
                                }
                            }
                        }
                        Some(prev_node) => {
                            prev_node.borrow_mut().next = node.borrow().next.clone();
                        }
                    };

                    match &node.borrow().next {
                        None => {
                            // this must be the tail of the list.
                            match &node.borrow().prev {
                                None => {
                                    *head = None;
                                    *tail = None;
                                }
                                Some(prev_node) => *tail = Some(Rc::clone(&prev_node)),
                            }
                        }
                        Some(next_node) => {
                            next_node.borrow_mut().prev = node.borrow().prev.clone();
                        }
                    };

                    // then delete from map.
                    map.remove(&id);
                }
            },
            '=' => {
                let focal_len = &inst[(i + 1)..]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();

                match map.get(&id) {
                    None => {
                        let node = Rc::new(RefCell::new(LLNode {
                            focal_len: *focal_len,
                            prev: tail.as_ref().cloned(),
                            next: None,
                        }));

                        match head {
                            None => *head = Some(Rc::clone(&node)),
                            Some(_) => {}
                        }

                        match tail {
                            None => {}
                            Some(tail) => tail.borrow_mut().next = Some(Rc::clone(&node)),
                        }

                        *tail = Some(Rc::clone(&node));
                        map.insert(id.clone(), Rc::clone(&node));
                    }
                    Some(node) => node.borrow_mut().focal_len = *focal_len,
                }
            }
            _ => panic!("Unknown Char: {}", inst[i]),
        }
    }

    let focusing_powers: Vec<usize> = boxes
        .iter()
        .enumerate()
        .map(|(box_idx, (_, head, _))| {
            let box_idx = box_idx + 1;
            let mut i = 1;
            let mut maybe_node = head.clone();
            let mut power = 0;
            while let Some(node) = maybe_node {
                power += i * node.borrow().focal_len;
                i += 1;
                maybe_node = node.borrow().next.clone();
            }

            box_idx * power
        })
        .collect();

    let total_power: usize = focusing_powers.iter().sum();
    println!("Total Power: {}", total_power);

    // let hashes: Vec<usize> = tokens.iter().map(|&token| hash(token)).collect();
    // let hash_sum = hashes.iter().sum::<usize>();
    // println!("Sum: {}", hash_sum);
}
