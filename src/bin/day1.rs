use aoc_2022::read_lines_as_vec;

fn part2(lines: &[String]) -> u32 {
    // 204610
    let mut sum = 0u32;
    let mut sum_list = vec![];

    for (_, line) in lines.iter().enumerate() {
        if !line.is_empty() {
            let cal:u32 = line.parse().unwrap();
            sum += cal
        } else {
            // println!("{} {}", elve, sum);
            sum_list.push(sum);
            sum = 0;
        }
    }
    // println!("{} {}", elve, sum);
    sum_list.push(sum);

    sum_list.sort();
    sum_list.reverse();
    sum_list[0..3].into_iter().sum::<u32>()
}

fn part1(lines: &[String]) -> u32 {
    // 70374
    let mut sum = 0u32;
    let mut max_sum = 0u32;

    for (_, line) in lines.iter().enumerate() {
        if !line.is_empty() {
            let cal:u32 = line.parse().unwrap();
            sum += cal
        } else {
            // println!("{} {}", elve, sum);
            if sum > max_sum {
                max_sum = sum;
            }
            sum = 0;
        }
    }
    // println!("{} {}", elve, sum);
    if sum > max_sum {
        max_sum = sum;
    }
    max_sum
}
fn main() {
    let lines = read_lines_as_vec("input/input_day1.txt").unwrap();

    // let lines = vec!["1000",
    //                  "2000",
    //                  "3000",
    //                  "",
    //                  "4000",
    //                  "",
    //                  "5000",
    //                  "6000",
    //                  "",
    //                  "7000",
    //                  "8000",
    //                  "9000",
    //                  "",
    //                  "10000"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = vec!["1000",
                         "2000",
                         "3000",
                         "",
                         "4000",
                         "",
                         "5000",
                         "6000",
                         "",
                         "7000",
                         "8000",
                         "9000",
                         "",
                         "10000"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let result = part1(&lines);
        assert_eq!(result, 24000);
        let result = part2(&lines);
        assert_eq!(result, 45000);
    }
}