use aoc_2022::read_lines_as_vec;
use std::collections::VecDeque;

fn snafu_to_digit(c: char) -> i32 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => 0,
    }
}

fn max_digit_for_mul(mul: i32) -> i64 {
    let mut sum = 2 * 5_i64.pow(mul.try_into().unwrap());
    for i in 0..mul {
        sum += 2 * 5_i64.pow(i.try_into().unwrap());
    }
    sum
}
fn min_digit_for_mul(mul: i32) -> i64 {
    let mut sum = 1 * 5_i64.pow(mul.try_into().unwrap());
    for i in 0..mul {
        sum -= 2 * 5_i64.pow(i.try_into().unwrap());
    }
    sum
}

fn try_resolve(sum: i64, rem: i64, steps: usize, operations: &mut VecDeque<i64>) -> bool {
    if steps == 0 {
        if sum == rem {
            return true;
        }

        return false;
    }
    let f = 5_i64.pow((steps - 1).try_into().unwrap());

    for i in [2, 1, 0, -1, -2].iter() {
        // println!("{:?}", operations);
        let rem = rem + i * f;
        let max = max_digit_for_mul((steps - 1) as i32);
        // lower parts cant fill the rest
        if (rem - sum).abs() > max {
            continue
        }
        operations.push_back(*i);
        let res = try_resolve(sum, rem, steps - 1, operations);
        if res {
            return res
        }
        operations.pop_back();
    }
    false
}
fn part1(lines: &[String]) -> String {
    // 20=022=21--=2--12=-2
    let mut sum = 0;
    for line in lines.iter() {
        let mut digit = 0;
        for (x, c) in line.chars().enumerate() {
            let mul = line.len() - x - 1;
            let v = 5_i64.pow(mul.try_into().unwrap());
            let d = snafu_to_digit(c) as i64;
            digit += d * v;
        }
        sum += digit;
    }
    // println!("{}", sum);

    let mut mul = 0;
    for x in 0..64 {
        // let f = 5_i64.pow(x.try_into().unwrap());
        let min = min_digit_for_mul(x);
        let max = max_digit_for_mul(x);
        // println!("{} {} {}", min, f, max);
        if sum >= min && sum <= max {
            mul = x;
            break;
        }
    }

    let f = 5_i64.pow(mul.try_into().unwrap());
    let mut operations = VecDeque::new();
    for i in [2, 1].iter() {
        let rem = i * f;
        operations.push_back(*i);
        let res = try_resolve(sum, rem, (mul) as usize, &mut operations);
        if res {
            break
        }
        operations.pop_back();
    }
    let mut str = "".to_string();
    for x in operations {
        match x {
            2 => str += "2",
            1 => str += "1",
            0 => str += "0",
            -1 => str += "-",
            -2 => str += "=",
            _ => {}
        }
    }
    str
}

fn part2(lines: &[String]) -> String {
    "".to_string()
}

fn main() {
    let lines = read_lines_as_vec("input/input_day25.txt").unwrap();

    // let lines = vec![
    //     "1=-0-2", "12111", "2=0=", "21", "2=01", "111", "20012", "112", "1=-1=", "1-12", "12",
    //     "1=", "122",
    // ]
    // .iter()
    // .map(|s| s.to_string())
    // .collect::<Vec<_>>();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec![
            "1=-0-2", "12111", "2=0=", "21", "2=01", "111", "20012", "112", "1=-1=", "1-12", "12",
            "1=", "122",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, "2=-1=0");
        // let result = part2(&lines);
        // assert_eq!(result, 20);
    }
}
