use itertools::Itertools;
use aoc_2022::read_lines_as_vec;

fn part2(line: &String) -> u32 {
    // 2472
    let mut sum = 0u32;

    let mut start = 0;
    for i in 13..line.len() {
        let chunk = line.get(start..i + 1).unwrap();
        if chunk.chars().all_unique() {
            sum = start as u32 + 14;
            break;
        }

        start += 1;
    }
    sum
}


fn part1(line: &String) -> u32 {
    // 1034
    let mut sum = 0u32;

    let mut start = 0;
    for i in 3..line.len() {
        let chunk = line.get(start..i + 1).unwrap();
        if chunk.chars().all_unique() {
            sum = start as u32 + 4;
            break;
        }

        start += 1;
    }
    sum
}
fn main() {
    let lines = read_lines_as_vec("input/input_day6.txt").unwrap();

    // let lines = vec!["mjqjpqmgbljsphdztnvjfqwrcgsmlb"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    println!("{}", part1(lines.first().unwrap()));
    println!("{}", part2(lines.first().unwrap()));
}
#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec!["mjqjpqmgbljsphdztnvjfqwrcgsmlb"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let result = part1(lines.first().unwrap());
        assert_eq!(result, 7);
        let result = part2(lines.first().unwrap());
        assert_eq!(result, 19);
    }
}
