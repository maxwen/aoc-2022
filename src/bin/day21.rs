use aoc_2022::read_lines_as_vec;
use priority_queue::PriorityQueue;
use regex::Regex;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Plus,
    Minus,
    Mul,
    Div,
    Nothing,
}

#[derive(Debug)]
struct Monkey {
    name: String,
    op: Operation,
    value: i64,
    input1: Option<String>,
    input2: Option<String>,
}

impl Monkey {
    fn set_value(&mut self, new_value: i64) {
        self.value = new_value
    }
    fn eval(&mut self, monkey_map: &HashMap<String, RefCell<Monkey>>) {
        match self.op {
            Operation::Plus => {
                let m1 = monkey_map.get(self.input1.as_ref().unwrap()).unwrap().borrow();
                let m2 = monkey_map.get(self.input2.as_ref().unwrap()).unwrap().borrow();
                self.value = m1.value + m2.value
            }
            Operation::Minus => {
                let m1 = monkey_map.get(self.input1.as_ref().unwrap()).unwrap().borrow();
                let m2 = monkey_map.get(self.input2.as_ref().unwrap()).unwrap().borrow();
                self.value = m1.value - m2.value
            }
            Operation::Mul => {
                let m1 = monkey_map.get(self.input1.as_ref().unwrap()).unwrap().borrow();
                let m2 = monkey_map.get(self.input2.as_ref().unwrap()).unwrap().borrow();
                self.value = m1.value * m2.value
            }
            Operation::Div => {
                let m1 = monkey_map.get(self.input1.as_ref().unwrap()).unwrap().borrow();
                let m2 = monkey_map.get(self.input2.as_ref().unwrap()).unwrap().borrow();
                self.value = m1.value / m2.value
            }
            Operation::Nothing => {}
        }
    }

    // do the reverse calculation of one part of the tree to match the wanted result_value
    fn rev_eval(&self, result_value: i64, eval_monkey: &String, monkey_map: &HashMap<String, RefCell<Monkey>>) -> i64 {
        match self.op {
            Operation::Plus => {
                if eval_monkey == self.input1.as_ref().unwrap() {
                    // left side
                    let m2 = monkey_map.get(self.input2.as_ref().unwrap()).unwrap().borrow();
                    result_value - m2.value
                } else {
                    // right side
                    let m1 = monkey_map.get(self.input1.as_ref().unwrap()).unwrap().borrow();
                    result_value - m1.value
                }
            }
            Operation::Minus => {
                if eval_monkey == self.input1.as_ref().unwrap() {
                    let m2 = monkey_map.get(self.input2.as_ref().unwrap()).unwrap().borrow();
                    m2.value + result_value
                } else {
                    let m1 = monkey_map.get(self.input1.as_ref().unwrap()).unwrap().borrow();
                    m1.value - result_value
                }
            }
            Operation::Mul => {
                if eval_monkey == self.input1.as_ref().unwrap() {
                    let m2 = monkey_map.get(self.input2.as_ref().unwrap()).unwrap().borrow();
                    result_value / m2.value
                } else {
                    let m1 = monkey_map.get(self.input1.as_ref().unwrap()).unwrap().borrow();
                    result_value / m1.value
                }
            }
            Operation::Div => {
                if eval_monkey == self.input1.as_ref().unwrap() {
                    let m2 = monkey_map.get(self.input2.as_ref().unwrap()).unwrap().borrow();
                    m2.value * result_value
                } else {
                    let m1 = monkey_map.get(self.input1.as_ref().unwrap()).unwrap().borrow();
                    m1.value / result_value
                }
            }
            Operation::Nothing => { 0i64 }
        }
    }

    fn can_eval(&self, monkey_map: &HashMap<String, RefCell<Monkey>>) -> bool {
        if self.value != -1 {
            return true;
        }
        match self.op {
            Operation::Nothing => { true }
            _ => {
                let m1 = monkey_map.get(self.input1.as_ref().unwrap().as_str()).unwrap().borrow();
                let m2 = monkey_map.get(self.input2.as_ref().unwrap().as_str()).unwrap().borrow();
                m1.value != -1 && m2.value != -1
            }
        }
    }

    fn reset_value(&mut self) {
        match self.op {
            Operation::Nothing => {}
            _ => {
                self.value = -1;
            }
        }
    }
}

#[allow(dead_code)]
fn reset_all_monkey_values(monkey_map: &HashMap<String, RefCell<Monkey>>) {
    monkey_map.iter().for_each(|e| {
        let mut monkey = e.1.borrow_mut();
        monkey.reset_value();
    })
}

fn part1(lines: &[String]) -> i64 {
    // 63119856257960
    let re_digit = Regex::new(r"\d+").unwrap();

    let mut monkeys = HashMap::new();
    for line in lines.iter() {
        let parts = line.split(":").collect::<Vec<_>>();
        let monkey = parts.first().unwrap();
        let op = parts.last().unwrap();
        if re_digit.is_match(op) {
            let m = Monkey {
                name: monkey.to_string(),
                op: Operation::Nothing,
                value: re_digit.find(op).unwrap().as_str().parse().unwrap(),
                input1: None,
                input2: None,
            };
            monkeys.insert(monkey.to_string(), RefCell::new(m));
        } else {
            if op.contains("*") {
                let op_parts = op.split("*").collect::<Vec<_>>();
                let m1_name = op_parts.first().unwrap().trim().to_string();
                let m2_name = op_parts.last().unwrap().trim().to_string();
                let m = Monkey {
                    name: monkey.to_string(),
                    op: Operation::Mul,
                    value: -1,
                    input1: Some(m1_name.to_string()),
                    input2: Some(m2_name.to_string()),
                };
                monkeys.insert(monkey.to_string(), RefCell::new(m));
            }
            if op.contains("/") {
                let op_parts = op.split("/").collect::<Vec<_>>();
                let m1_name = op_parts.first().unwrap().trim().to_string();
                let m2_name = op_parts.last().unwrap().trim().to_string();
                let m = Monkey {
                    name: monkey.to_string(),
                    op: Operation::Div,
                    value: -1,
                    input1: Some(m1_name.to_string()),
                    input2: Some(m2_name.to_string()),
                };
                monkeys.insert(monkey.to_string(), RefCell::new(m));
            }
            if op.contains("+") {
                let op_parts = op.split("+").collect::<Vec<_>>();
                let m1_name = op_parts.first().unwrap().trim().to_string();
                let m2_name = op_parts.last().unwrap().trim().to_string();
                let m = Monkey {
                    name: monkey.to_string(),
                    op: Operation::Plus,
                    value: -1,
                    input1: Some(m1_name.to_string()),
                    input2: Some(m2_name.to_string()),
                };
                monkeys.insert(monkey.to_string(), RefCell::new(m));
            }
            if op.contains("-") {
                let op_parts = op.split("-").collect::<Vec<_>>();
                let m1_name = op_parts.first().unwrap().trim().to_string();
                let m2_name = op_parts.last().unwrap().trim().to_string();
                let m = Monkey {
                    name: monkey.to_string(),
                    op: Operation::Minus,
                    value: -1,
                    input1: Some(m1_name.to_string()),
                    input2: Some(m2_name.to_string()),
                };
                monkeys.insert(monkey.to_string(), RefCell::new(m));
            }
        }
    }

    // println!("{:?}", monkeys);

    let root = monkeys.get(&"root".to_string()).unwrap();
    while root.borrow().value == -1 {
        monkeys.iter().for_each(|e| {
            let mut monkey = e.1.borrow_mut();
            if monkey.can_eval(&monkeys) {
                monkey.eval(&monkeys);
            }
        })
    }

    let res = root.borrow().value;
    res
}
#[allow(dead_code)]
fn print_dot_graph(monkey_map: &HashMap<String, RefCell<Monkey>>) {
    println!("digraph {{");
    monkey_map.iter().for_each(|e| {
        let monkey = e.1.borrow();
        if monkey.op != Operation::Nothing {
            println!("{} -> {};", monkey.name, monkey.input1.as_ref().unwrap());
            println!("{} -> {};", monkey.name, monkey.input2.as_ref().unwrap())
        }
    });
    println!("}}");
}

#[allow(dead_code)]
fn try_eval(monkey_map: &HashMap<String, RefCell<Monkey>>, target_node: &String, value: i64) -> i64 {
    reset_all_monkey_values(&monkey_map);

    let humn = monkey_map.get(&"humn".to_string()).unwrap();
    humn.borrow_mut().set_value(value);

    let target = monkey_map.get(target_node).unwrap();
    while target.borrow().value == -1 {
        monkey_map.iter().for_each(|e| {
            let mut monkey = e.1.borrow_mut();
            if monkey.can_eval(&monkey_map) {
                monkey.eval(&monkey_map);
            }
        })
    }
    target.borrow().value
}

fn dijkstra(start_id: &String, end_id: &String, monkey_map: &HashMap<String, RefCell<Monkey>>, path: &mut Vec<String>) {
    let mut stack = PriorityQueue::new();
    let mut p = vec![];
    p.push(start_id.clone());
    stack.push(p, 0);

    let mut seen: HashMap<String, u32> = HashMap::new();
    seen.insert(start_id.clone(), 0);

    while let Some((p, steps)) = stack.pop() {
        let current = p.last().unwrap();
        if current == end_id {
            p.iter().for_each(|e| path.push(e.clone()));
            return;
        }

        let m = monkey_map.get(&current.to_string()).unwrap();
        if m.borrow().op != Operation::Nothing {
            let m1 = m.borrow().input1.as_ref().unwrap().to_string();
            let m2 = m.borrow().input2.as_ref().unwrap().to_string();
            for edge in vec![m1, m2] {
                let dist_next_pos = seen.get(&edge).unwrap_or(&u32::MAX);
                if steps + 1 < *dist_next_pos {
                    let mut p1 = p.clone();
                    p1.push(edge.clone());
                    seen.insert(edge, steps + 1);
                    stack.push(p1, steps + 1);
                }
            }
        }
    }
}

// target_node is the one we need to calc the value to
// be the same as ref_node
fn part2(lines: &[String], target_node: &String, ref_node: &String) -> i64 {
    // 3006709232464
    let re_digit = Regex::new(r"\d+").unwrap();

    let mut monkeys = HashMap::new();
    for line in lines.iter() {
        let parts = line.split(":").collect::<Vec<_>>();
        let monkey = parts.first().unwrap();
        let op = parts.last().unwrap();
        if re_digit.is_match(op) {
            let m = Monkey {
                name: monkey.to_string(),
                op: Operation::Nothing,
                value: re_digit.find(op).unwrap().as_str().parse().unwrap(),
                input1: None,
                input2: None,
            };
            monkeys.insert(monkey.to_string(), RefCell::new(m));
        } else {
            if op.contains("*") {
                let op_parts = op.split("*").collect::<Vec<_>>();
                let m1_name = op_parts.first().unwrap().trim().to_string();
                let m2_name = op_parts.last().unwrap().trim().to_string();
                let m = Monkey {
                    name: monkey.to_string(),
                    op: Operation::Mul,
                    value: -1,
                    input1: Some(m1_name.to_string()),
                    input2: Some(m2_name.to_string()),
                };
                monkeys.insert(monkey.to_string(), RefCell::new(m));
            }
            if op.contains("/") {
                let op_parts = op.split("/").collect::<Vec<_>>();
                let m1_name = op_parts.first().unwrap().trim().to_string();
                let m2_name = op_parts.last().unwrap().trim().to_string();
                let m = Monkey {
                    name: monkey.to_string(),
                    op: Operation::Div,
                    value: -1,
                    input1: Some(m1_name.to_string()),
                    input2: Some(m2_name.to_string()),
                };
                monkeys.insert(monkey.to_string(), RefCell::new(m));
            }
            if op.contains("+") {
                let op_parts = op.split("+").collect::<Vec<_>>();
                let m1_name = op_parts.first().unwrap().trim().to_string();
                let m2_name = op_parts.last().unwrap().trim().to_string();
                let m = Monkey {
                    name: monkey.to_string(),
                    op: Operation::Plus,
                    value: -1,
                    input1: Some(m1_name.to_string()),
                    input2: Some(m2_name.to_string()),
                };
                monkeys.insert(monkey.to_string(), RefCell::new(m));
            }
            if op.contains("-") {
                let op_parts = op.split("-").collect::<Vec<_>>();
                let m1_name = op_parts.first().unwrap().trim().to_string();
                let m2_name = op_parts.last().unwrap().trim().to_string();
                let m = Monkey {
                    name: monkey.to_string(),
                    op: Operation::Minus,
                    value: -1,
                    input1: Some(m1_name.to_string()),
                    input2: Some(m2_name.to_string()),
                };
                monkeys.insert(monkey.to_string(), RefCell::new(m));
            }
        }
    }

    // humn is in left side of tree so value of cgdh must match qhpl

    let mut path: Vec<String> = vec![];
    dijkstra(target_node, &"humn".to_string(), &monkeys, &mut path);
    // println!("{} {:?}", steps, path);

    // ebal once
    let root = monkeys.get(&"root".to_string()).unwrap();
    while root.borrow().value == -1 {
        monkeys.iter().for_each(|e| {
            let mut monkey = e.1.borrow_mut();
            if monkey.can_eval(&monkeys) {
                monkey.eval(&monkeys);
            }
        })
    }

    // target value
    let qhpl = monkeys.get(ref_node).unwrap();
    let ref_result = qhpl.borrow().value;
    // println!("qhpl ref_result: {}", ref_result);

    // reverse calculation path down to humn
    let mut result_value = ref_result;
    for (i, monkey_name) in path.iter().enumerate() {
        let m = monkeys.get(&monkey_name.to_string()).unwrap().borrow();
        let eval_monkey_name = path.get(i + 1).unwrap();
        result_value = m.rev_eval(result_value, &eval_monkey_name, &monkeys);
        if *eval_monkey_name == "humn".to_string() {
            break;
        }
    }

    // println!("try_eval {}", try_eval(&monkeys, result_value));

    result_value
}

fn main() {
    let lines = read_lines_as_vec("input/input_day21.txt").unwrap();

    // let lines = vec!["root: pppw + sjmn",
    //                  "dbpl: 5",
    //                  "cczh: sllz + lgvd",
    //                  "zczc: 2",
    //                  "ptdq: humn - dvpt",
    //                  "dvpt: 3",
    //                  "lfqf: 4",
    //                  "humn: 5",
    //                  "ljgn: 2",
    //                  "sjmn: drzm * dbpl",
    //                  "sllz: 4",
    //                  "pppw: cczh / lfqf",
    //                  "lgvd: ljgn * ptdq",
    //                  "drzm: hmdt - zczc",
    //                  "hmdt: 32"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines, &"cgdh".to_string(), &"qhpl".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec!["root: pppw + sjmn",
                         "dbpl: 5",
                         "cczh: sllz + lgvd",
                         "zczc: 2",
                         "ptdq: humn - dvpt",
                         "dvpt: 3",
                         "lfqf: 4",
                         "humn: 5",
                         "ljgn: 2",
                         "sjmn: drzm * dbpl",
                         "sllz: 4",
                         "pppw: cczh / lfqf",
                         "lgvd: ljgn * ptdq",
                         "drzm: hmdt - zczc",
                         "hmdt: 32"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 152);
        let result = part2(&lines, &"pppw".to_string(), &"sjmn".to_string());
        assert_eq!(result, 301);
    }
}
