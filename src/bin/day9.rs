use itertools::Itertools;
use std::collections::HashSet;
use aoc_2022::read_lines_as_vec;

enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl TryFrom<i32> for Direction {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Direction::Up as i32 => Ok(Direction::Up),
            x if x == Direction::Down as i32 => Ok(Direction::Down),
            x if x == Direction::Left as i32 => Ok(Direction::Left),
            x if x == Direction::Right as i32 => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

fn move_head(pos: (i32, i32), d: &Direction) -> (i32, i32) {
    match d {
        Direction::Up => (pos.0, pos.1 + 1),
        Direction::Down => (pos.0, pos.1 - 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}

fn tail_must_move(head_pos: (i32, i32), tail_pos: (i32, i32)) -> bool {
    head_pos.0.abs_diff(tail_pos.0) > 1 || head_pos.1.abs_diff(tail_pos.1) > 1
}
fn move_tail(head_pos: (i32, i32), tail_pos: (i32, i32), d: &Direction) -> (i32, i32) {
    if tail_must_move(head_pos, tail_pos) {
        match d {
            Direction::Up => {
                if tail_pos.0 != head_pos.0 {
                    (head_pos.0, tail_pos.1 + 1)
                } else {
                    (tail_pos.0, tail_pos.1 + 1)
                }
            }
            Direction::Down => {
                if tail_pos.0 != head_pos.0 {
                    (head_pos.0, tail_pos.1 - 1)
                } else {
                    (tail_pos.0, tail_pos.1 - 1)
                }
            }
            Direction::Left => {
                if tail_pos.1 != head_pos.1 {
                    (tail_pos.0 - 1, head_pos.1)
                } else {
                    (tail_pos.0 - 1, tail_pos.1)
                }
            }
            Direction::Right => {
                if tail_pos.1 != head_pos.1 {
                    (tail_pos.0 + 1, head_pos.1)
                } else {
                    (tail_pos.0 + 1, tail_pos.1)
                }
            }
        }
    } else {
        tail_pos
    }
}

fn part1(lines: &[String]) -> u32 {
    // 5883
    let mut head = (0i32, 0i32);
    let mut tail = (0i32, 0i32);
    let mut path = HashSet::new();

    for (_, line) in lines.iter().enumerate() {
        let step: Vec<_> = line.split_whitespace().collect();
        let direction = step.first().unwrap();
        let d = match direction {
            &"U" => Direction::try_from(0).unwrap(),
            &"D" => Direction::try_from(1).unwrap(),
            &"L" => Direction::try_from(2).unwrap(),
            &"R" => Direction::try_from(3).unwrap(),
            _ => continue
        };
        let steps: usize = step.last().unwrap().parse().unwrap();
        for _ in 0..steps {
            head = move_head(head, &d);
            tail = move_tail(head, tail, &d);
            path.insert(tail);
        }
        // println!("head {}:{} tail {}:{}", head.0, head.1, tail.0, tail.1);
    }
    // println!("{:?}", path);
    path.len() as u32
}

fn part2(lines: &[String]) -> u32 {
    let mut sum = 0u32;

    sum
}


fn main() {
    let lines = read_lines_as_vec("input/input_day9.txt").unwrap();

    // let lines = vec!["R 4",
    //                  "U 4",
    //                  "L 3",
    //                  "D 1",
    //                  "R 4",
    //                  "D 1",
    //                  "L 5",
    //                  "R 2"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let lines = vec!["R 4",
                         "U 4",
                         "L 3",
                         "D 1",
                         "R 4",
                         "D 1",
                         "L 5",
                         "R 2"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 13);
        // let result = part2(&lines);
        // assert_eq!(result, 8);
    }
}
