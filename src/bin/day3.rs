use std::collections::HashSet;
use aoc_2022::read_lines_as_vec;

fn part2(lines: &[String]) -> u32 {
    0u32
}

fn get_priority(c: &char) -> u32 {

    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    // match c {
    //     'a'..'z' => c.to_ascii_lowercase() as u32,
    //     'A'..'Z' => 1,
    //     _ => 0
    // }
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

        let rucksack1: HashSet<char> = line[0..rucksack_size].chars().collect();
        let rucksack2: HashSet<char> = line[rucksack_size..].chars().collect();
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
    use crate::part1;

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
        // let result = part2(&lines);
        // assert_eq!(result, 12);
    }
}
