use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines(filepath: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_as_vec(filepath: &str) -> io::Result<Vec<String>> {
    let lines = read_lines(filepath)?;
    Ok(lines.flatten().collect())
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
    println!("{}", part1(&lines))
}