use std::cell::RefCell;
use aoc_2022::read_lines_as_vec;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
struct Monkey {
    stack: VecDeque<u64>,
    operation: fn(u64) -> u64,
    condition: fn(u64) -> bool,
    monkey_true_idx: usize,
    monkey_false_idx: usize,
    inspect_count: u64,
}

impl Monkey {
    fn add_item(&mut self, item: u64) {
        self.stack.push_back(item)
    }

    fn inspect_item(&mut self, monkey_true: &mut Monkey, monkey_false: &mut Monkey) -> bool {
        if self.stack.len() != 0 {
            let item = self.stack.pop_front().unwrap();
            let item_new = (self.operation)(item);
            let item_bored = item_new / 3;
            // let item_bored = item_new;

            if (self.condition)(item_bored) {
                monkey_true.add_item(item_bored)
            } else {
                monkey_false.add_item(item_bored)
            }
            self.inspect_count += 1;

            return true;
        }
        false
    }
}
fn part1(lines: &[String]) -> u64 {
    // 66124
    let mut monkey_map: HashMap<usize, RefCell<Monkey>> = HashMap::new();

    let mut monkey0 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value * 13,
        condition: |value| value % 19 == 0,
        monkey_true_idx: 2,
        monkey_false_idx: 7,
        inspect_count: 0,
    };
    vec![75, 75, 98, 97, 79, 97, 64].iter().for_each(|x| monkey0.stack.push_back(*x));
    monkey_map.insert(0, RefCell::new(monkey0));

    let mut monkey1 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value + 2,
        condition: |value| value % 3 == 0,
        monkey_true_idx: 4,
        monkey_false_idx: 5,
        inspect_count: 0,
    };
    vec![50, 99, 80, 84, 65, 95].iter().for_each(|x| monkey1.stack.push_back(*x));
    monkey_map.insert(1, RefCell::new(monkey1));

    let mut monkey2 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value + 1,
        condition: |value| value % 11 == 0,
        monkey_true_idx: 7,
        monkey_false_idx: 3,
        inspect_count: 0,
    };
    vec![96, 74, 68, 96, 56, 71, 75, 53].iter().for_each(|x| monkey2.stack.push_back(*x));
    monkey_map.insert(2, RefCell::new(monkey2));

    let mut monkey3 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value + 8,
        condition: |value| value % 17 == 0,
        monkey_true_idx: 6,
        monkey_false_idx: 1,
        inspect_count: 0,
    };
    vec![83, 96, 86, 58, 92].iter().for_each(|x| monkey3.stack.push_back(*x));
    monkey_map.insert(3, RefCell::new(monkey3));

    let mut monkey4 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value * value,
        condition: |value| value % 5 == 0,
        monkey_true_idx: 0,
        monkey_false_idx: 5,
        inspect_count: 0,
    };
    vec![99].iter().for_each(|x| monkey4.stack.push_back(*x));
    monkey_map.insert(4, RefCell::new(monkey4));

    let mut monkey5 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value + 4,
        condition: |value| value % 2 == 0,
        monkey_true_idx: 2,
        monkey_false_idx: 0,
        inspect_count: 0,
    };
    vec![60, 54, 83].iter().for_each(|x| monkey5.stack.push_back(*x));
    monkey_map.insert(5, RefCell::new(monkey5));

    let mut monkey6 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value * 17,
        condition: |value| value % 13 == 0,
        monkey_true_idx: 4,
        monkey_false_idx: 1,
        inspect_count: 0,
    };
    vec![77, 67].iter().for_each(|x| monkey6.stack.push_back(*x));
    monkey_map.insert(6, RefCell::new(monkey6));

    let mut monkey7 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value + 5,
        condition: |value| value % 7 == 0,
        monkey_true_idx: 3,
        monkey_false_idx: 6,
        inspect_count: 0,
    };
    vec![95, 65, 58, 76].iter().for_each(|x| monkey7.stack.push_back(*x));
    monkey_map.insert(7, RefCell::new(monkey7));

    for _ in 0..20 {
        for idx in 0..8 {
            unsafe {
                let m = monkey_map.get(&idx).unwrap();
                let m_true = monkey_map.get(&m.borrow().monkey_true_idx).unwrap();
                let m_false = monkey_map.get(&m.borrow().monkey_false_idx).unwrap();

                while m.borrow_mut().inspect_item(&mut *m_true.borrow_mut(), &mut *m_false.borrow_mut()) {}
            }
        }
    }

    let mut inspect_count = vec![];
    monkey_map.values().for_each(|m| inspect_count.push(m.borrow().inspect_count));

    inspect_count.sort();
    inspect_count.reverse();
    inspect_count.get(0..2).unwrap().iter().product::<u64>()
}

fn part2(lines: &[String]) -> u32 {
    0u32
}


fn main() {
    let lines = read_lines_as_vec("input/input_day11.txt").unwrap();

    // let lines = read_lines_as_vec("input_test/input_day11_test.txt").unwrap();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
#[cfg(test)]
mod tests {
    use crate::part1;
    use aoc_2022::read_lines_as_vec;

    #[test]
    fn it_works() {
        let lines = read_lines_as_vec("input_test/input_day11_test.txt").unwrap();

        let result = part1(&lines);
        assert_eq!(result, 1065);
        // assert_eq!(result, 36);
    }
}
