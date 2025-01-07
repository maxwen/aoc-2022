use std::collections::HashSet;
use aoc_2022::read_lines_as_vec;

fn part2(lines: &[String]) -> u32 {
    // 2508
    let mut sum = 0u32;
    for (_, group) in lines.chunks(3).enumerate() {
        let rucksack1 = group[0].chars().collect::<HashSet<_>>();
        let rucksack2 = group[1].chars().collect::<HashSet<_>>();
        let rucksack3 = group[2].chars().collect::<HashSet<_>>();

        let i1 = rucksack1.intersection(&rucksack2).cloned().collect::<HashSet<_>>();
        let i2 = i1.intersection(&rucksack3).collect::<Vec<_>>();

        for (_, c) in i2.iter().enumerate() {
            sum += get_priority(c)
        }
    }
    sum
}

fn get_priority(c: &char) -> u32 {
    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    if c.is_lowercase() {
        c.to_ascii_lowercase() as u32 - 96
    } else {
        c.to_ascii_lowercase() as u32 - 96 + 26
    }
}

fn part1(lines: &[String]) -> u32 {
    // 7691
    let mut sum = 0u32;
    for (_, line) in lines.iter().enumerate() {
        let rucksack_size = line.len() / 2;

        let rucksack1 = line[0..rucksack_size].chars().collect::<HashSet<_>>();
        let rucksack2 = line[rucksack_size..].chars().collect::<HashSet<_>>();
        let intersection = rucksack1.intersection(&rucksack2).collect::<Vec<_>>();
        // println!("{:?}", intersection);

        for (_, c) in intersection.iter().enumerate() {
            sum += get_priority(c)
        }
    }
    sum
}
fn main() {
    let lines = read_lines_as_vec("input/input_day3.txt").unwrap();

    // let lines = vec!["vJrwpWtwJgWrhcsFMMfFFhFp",
    //                  "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
    //                  "PmmdzqPrVvPwwTWBwg",
    //                  "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
    //                  "ttgJtRGJQctTZtZT",
    //                  "CrZsJsPPZsGzwwsLwLmpwMDw"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec!["vJrwpWtwJgWrhcsFMMfFFhFp",
                         "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                         "PmmdzqPrVvPwwTWBwg",
                         "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                         "ttgJtRGJQctTZtZT",
                         "CrZsJsPPZsGzwwsLwLmpwMDw"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let result = part1(&lines);
        assert_eq!(result, 157);
        let result = part2(&lines);
        assert_eq!(result, 70);
    }
}
