use aoc_2022::read_lines_as_vec;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
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

    fn can_eval(&self, monkey_map: &HashMap<String, RefCell<Monkey>>) -> bool {
        match self.op {
            Operation::Nothing => { true }
            _ => {
                let m1 = monkey_map.get(self.input1.as_ref().unwrap().as_str()).unwrap().borrow();
                let m2 = monkey_map.get(self.input2.as_ref().unwrap().as_str()).unwrap().borrow();
                // m1.can_eval(monkey_map) && m2.can_eval(monkey_map)
                m1.value != -1 && m2.value != -1
            }
        }
    }
}

fn part1(lines: &[String]) -> i64 {
    // 63119856257960
    let re_digit = Regex::new(r"\d+").unwrap();

    let mut monkeys = HashMap::new();
    for line in lines.iter() {
        let parts = line.split(":").collect::<Vec<_>>();
        let monkey = parts.first().unwrap().to_string();
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
fn part2(lines: &[String]) -> usize {
    0usize
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
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::part1;

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
        // let result = part2(&lines);
        // assert_eq!(result, 58);
    }
}
