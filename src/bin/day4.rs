use std::collections::HashSet;
use aoc_2022::read_lines_as_vec;

fn part2(lines: &[String]) -> u32 {
    // 895
    let mut sum = 0u32;

    for (_, line) in lines.iter().enumerate() {
        let sections = line.split(",").collect::<Vec<&str>>();
        let pair1 = sections.first().unwrap().split("-").map(|a| a.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let pair1_start = *pair1.first().unwrap();
        let pair1_end = *pair1.last().unwrap();
        let pair1_set =(pair1_start..pair1_end+1).collect::<HashSet<u32>>();

        let pair2 = sections.last().unwrap().split("-").map(|a| a.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let pair2_start = *pair2.first().unwrap();
        let pair2_end = *pair2.last().unwrap();
        let pair2_set =(pair2_start..pair2_end+1).collect::<HashSet<u32>>();

        if (pair1_set.intersection(&pair2_set).count() != 0) {
            sum += 1
        }
    }
    sum
}

fn part1(lines: &[String]) -> u32 {
    // 580
    let mut sum = 0u32;

    for (_, line) in lines.iter().enumerate() {
        let sections = line.split(",").collect::<Vec<&str>>();
        let pair1 = sections.first().unwrap().split("-").map(|a| a.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let pair1_start = *pair1.first().unwrap();
        let pair1_end = *pair1.last().unwrap();

        let pair2 = sections.last().unwrap().split("-").map(|a| a.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let pair2_start = *pair2.first().unwrap();
        let pair2_end = *pair2.last().unwrap();

        if (pair1_start <= pair2_start && pair1_end >= pair2_end) || (pair2_start <= pair1_start && pair2_end >= pair1_end) {
            sum += 1
        }
    }
    sum
}
fn main() {
    let lines = read_lines_as_vec("input/input_day4.txt").unwrap();

    // let lines = vec!["2-4,6-8",
    //                  "2-3,4-5",
    //                  "5-7,7-9",
    //                  "2-8,3-7",
    //                  "6-6,4-6",
    //                  "2-6,4-8"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec!["2-4,6-8",
                         "2-3,4-5",
                         "5-7,7-9",
                         "2-8,3-7",
                         "6-6,4-6",
                         "2-6,4-8"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let result = part1(&lines);
        assert_eq!(result, 2);
        let result = part2(&lines);
        assert_eq!(result, 4);
    }
}
