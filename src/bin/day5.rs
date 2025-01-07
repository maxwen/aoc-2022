use std::collections::{HashMap, VecDeque};
use regex::Regex;
use itertools::Itertools;
use aoc_2022::read_lines_as_vec;

fn part2(lines: &[String]) -> String {
    // WDLPFNNNB
    let mut sum = "".to_string();
    let mut stacks: HashMap<usize, VecDeque<char>> = HashMap::new();

    let re = Regex::new(r"\d+").unwrap(); // \d means digit
    for (_, line) in lines.iter().enumerate() {
        if line.starts_with("move") {
            let move_numbers = re.find_iter(line).collect::<Vec<_>>();
            let amount: usize = move_numbers.get(0).unwrap().as_str().parse().unwrap();
            let from: usize = move_numbers.get(1).unwrap().as_str().parse().unwrap();
            let to: usize = move_numbers.get(2).unwrap().as_str().parse().unwrap();

            // move first one last to keep order
            for i in (0..amount).rev() {
                let c = stacks.get_mut(&from).unwrap().remove(i).unwrap();
                stacks.get_mut(&to).unwrap().push_front(c);
            }
        } else if line.len() != 0 {
            for (n, c) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                if c.contains(&'[') {
                    let key = n + 1;
                    stacks.entry(key)
                        .or_insert_with(VecDeque::new)
                        .push_back(c[1]);
                }
            }
        }
    }

    for (_, s) in stacks.iter().sorted() {
        let c = s.front().unwrap_or(&' ');
        sum.push(*c)
    }
    sum
}


fn part1(lines: &[String]) -> String {
    // ZBDRNPMVH
    let mut sum = "".to_string();
    let mut stacks: HashMap<usize, VecDeque<char>> = HashMap::new();

    let re = Regex::new(r"\d+").unwrap(); // \d means digit
    for (_, line) in lines.iter().enumerate() {
        if line.starts_with("move") {
            let move_numbers = re.find_iter(line).collect::<Vec<_>>();
            let amount: usize = move_numbers.get(0).unwrap().as_str().parse().unwrap();
            let from: usize = move_numbers.get(1).unwrap().as_str().parse().unwrap();
            let to: usize = move_numbers.get(2).unwrap().as_str().parse().unwrap();

            // move one at a time like normal stack
            for _ in 0..amount {
                let c = stacks.get_mut(&from).unwrap().pop_front().unwrap();
                stacks.get_mut(&to).unwrap().push_front(c);
            }
        } else if line.len() != 0 {
            for (n, c) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                if c.contains(&'[') {
                    let key = n + 1;
                    stacks.entry(key)
                        .or_insert_with(VecDeque::new)
                        .push_back(c[1]);
                    // or
                    // match stacks.get_mut(&key) {
                    //     Some(s) => {
                    //         s.push_back(c[1]);
                    //     }
                    //     None => {
                    //         let mut s: VecDeque<char> = VecDeque::new();
                    //         s.push_back(c[1]);
                    //         stacks.insert(key, s);
                    //     }
                    // }
                }
            }
        }
    }

    for (_, s) in stacks.iter().sorted() {
        let c = s.front().unwrap_or(&' ');
        sum.push(*c)
    }
    sum
}
fn main() {
    let lines = read_lines_as_vec("input/input_day5.txt").unwrap();

    // let lines = vec!["    [D]    ",
    //                  "[N] [C]    ",
    //                  "[Z] [M] [P]",
    //                  " 1   2   3 ",
    //                  "",
    //                  "move 1 from 2 to 1",
    //                  "move 3 from 1 to 3",
    //                  "move 2 from 2 to 1",
    //                  "move 1 from 1 to 2"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec!["    [D]    ",
                         "[N] [C]    ",
                         "[Z] [M] [P]",
                         " 1   2   3 ",
                         "",
                         "move 1 from 2 to 1",
                         "move 3 from 1 to 3",
                         "move 2 from 2 to 1",
                         "move 1 from 1 to 2"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let result = part1(&lines);
        assert_eq!(result, "CMZ");
        let result = part2(&lines);
        assert_eq!(result, "MCD");
    }
}
