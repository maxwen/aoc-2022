use aoc_2022::read_lines_as_vec;
use std::collections::VecDeque;


#[derive(Debug, Clone)]
struct Monkey {
    stack: VecDeque<u32>,
    operation: fn(u32) -> u32,
    condition: fn(u32) -> bool,
    monkey_true_idx: usize,
    monkey_false_idx: usize,
    inspect_count: u32,
}

impl Monkey {
    fn add_item(&mut self, item: u32) {
        self.stack.push_back(item)
    }

    fn inspect_item(&mut self, monkey_true: &mut Monkey, monkey_false: &mut Monkey) -> bool {
        if self.stack.len() != 0 {
            let item = self.stack.pop_front().unwrap();
            let item_new = (self.operation)(item);
            let item_bored = item_new / 3;

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
fn part1(lines: &[String]) -> u32 {
    let mut monkey0 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value * 13,
        condition: |value| value % 19 == 0,
        monkey_true_idx: 2,
        monkey_false_idx: 7,
        inspect_count: 0,
    };
    vec![75, 75, 98, 97, 79, 97, 64].iter().for_each(|x| monkey0.stack.push_back(*x));

    let mut monkey1 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value + 2,
        condition: |value| value % 3 == 0,
        monkey_true_idx: 4,
        monkey_false_idx: 5,
        inspect_count: 0,
    };
    vec![50, 99, 80, 84, 65, 95].iter().for_each(|x| monkey1.stack.push_back(*x));

    let mut monkey2 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value + 1,
        condition: |value| value % 11 == 0,
        monkey_true_idx: 7,
        monkey_false_idx: 3,
        inspect_count: 0,
    };
    vec![96, 74, 68, 96, 56, 71, 75, 53].iter().for_each(|x| monkey2.stack.push_back(*x));

    let mut monkey3 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value + 8,
        condition: |value| value % 17 == 0,
        monkey_true_idx: 6,
        monkey_false_idx: 1,
        inspect_count: 0,
    };
    vec![83, 96, 86, 58, 92].iter().for_each(|x| monkey3.stack.push_back(*x));

    let mut monkey4 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value * value,
        condition: |value| value % 5 == 0,
        monkey_true_idx: 0,
        monkey_false_idx: 5,
        inspect_count: 0,
    };
    vec![99].iter().for_each(|x| monkey4.stack.push_back(*x));

    let mut monkey5 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value + 4,
        condition: |value| value % 2 == 0,
        monkey_true_idx: 2,
        monkey_false_idx: 0,
        inspect_count: 0,
    };
    vec![60, 54, 83].iter().for_each(|x| monkey5.stack.push_back(*x));

    let mut monkey6 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value * 17,
        condition: |value| value % 13 == 0,
        monkey_true_idx: 4,
        monkey_false_idx: 1,
        inspect_count: 0,
    };
    vec![77, 67].iter().for_each(|x| monkey6.stack.push_back(*x));

    let mut monkey7 = Monkey {
        stack: VecDeque::new(),
        operation: |value| value + 5,
        condition: |value| value % 7 == 0,
        monkey_true_idx: 3,
        monkey_false_idx: 6,
        inspect_count: 0,
    };
    vec![95, 65, 58, 76].iter().for_each(|x| monkey7.stack.push_back(*x));

    for _ in 0..20 {
        {
            while monkey0.inspect_item(&mut monkey2, &mut monkey7) {}
        }
        {
            while monkey1.inspect_item(&mut monkey4, &mut monkey5) {}
        }
        {
            while monkey2.inspect_item(&mut monkey7, &mut monkey3) {}
        }
        {
            while monkey3.inspect_item(&mut monkey6, &mut monkey1) {}
        }
        {
            while monkey4.inspect_item(&mut monkey0, &mut monkey5) {}
        }
        {
            while monkey5.inspect_item(&mut monkey2, &mut monkey0) {}
        }
        {
            while monkey6.inspect_item(&mut monkey4, &mut monkey1) {}
        }
        {
            while monkey7.inspect_item(&mut monkey3, &mut monkey6) {}
        }
    }

    // println!("Monkey 0: {:?}", monkey0.stack);
    // println!("Monkey 1: {:?}", monkey1.stack);
    // println!("Monkey 2: {:?}", monkey2.stack);
    // println!("Monkey 3: {:?}", monkey3.stack);

    let mut inspect_count = vec![];
    inspect_count.push(monkey0.inspect_count);
    inspect_count.push(monkey1.inspect_count);
    inspect_count.push(monkey2.inspect_count);
    inspect_count.push(monkey3.inspect_count);
    inspect_count.push(monkey4.inspect_count);
    inspect_count.push(monkey5.inspect_count);
    inspect_count.push(monkey6.inspect_count);
    inspect_count.push(monkey7.inspect_count);

    inspect_count.sort();
    inspect_count.reverse();
    inspect_count[0] * inspect_count[1]
}

fn part2(lines: &[String]) -> u32 {
    0u32
}


fn main() {
    // let lines = read_lines_as_vec("input/input_day11.txt").unwrap();

    unsafe {
        let lines = read_lines_as_vec("input_test/input_day11_test.txt").unwrap();

        println!("{}", part1(&lines));
        println!("{}", part2(&lines));
    }
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
