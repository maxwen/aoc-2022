use aoc_2022::read_lines_as_vec;
use std::collections::HashSet;

enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        match c {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
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
fn move_tail(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    if tail_must_move(head_pos, tail_pos) {
        let mut new_x = tail_pos.0;
        let mut new_y = tail_pos.1;

        if head_pos.0 > tail_pos.0 {
            new_x += 1;
        } else if head_pos.0 < tail_pos.0 {
            new_x -= 1;
        }

        if head_pos.1 > tail_pos.1 {
            new_y += 1;
        } else if head_pos.1 < tail_pos.1 {
            new_y -= 1;
        }
        (new_x, new_y)
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
        let d = Direction::try_from(*direction).unwrap();
        let steps: usize = step.last().unwrap().parse().unwrap();
        for _ in 0..steps {
            head = move_head(head, &d);
            tail = move_tail(head, tail);
            path.insert(tail);
        }
    }
    path.len() as u32
}

#[allow(dead_code)]
fn print_grid(snake: &Vec<(i32, i32)>) {
    for y in -20..20 {
        for x in -20..20 {
            if x == 0 && y == 0 {
                print!("s")
            } else {
                if snake.contains(&(x, -y)) {
                    print!("*")
                } else {
                    print!(".")
                }
            }
        }
        println!();
    }
}
fn part2(lines: &[String]) -> u32 {
    // 2367
    let mut path = HashSet::new();
    let mut snake = vec![(0i32, 0i32); 10];

    for (_, line) in lines.iter().enumerate() {
        let step: Vec<_> = line.split_whitespace().collect();
        let direction = step.first().unwrap();
        let d = Direction::try_from(*direction).unwrap();
        let steps: usize = step.last().unwrap().parse().unwrap();

        for _ in 0..steps {
            let mut new_snake = vec![];
            let mut head = *snake.first().unwrap();

            head = move_head(head, &d);
            new_snake.push(head.clone());

            for knot in snake.get(1..).unwrap().iter() {
                let new_knot = move_tail(head, *knot);
                new_snake.push(new_knot);
                head = new_knot;
            }

            snake.clear();
            for knot in new_snake {
                snake.push(knot.clone());
            }
            path.insert(snake.last().unwrap().clone());
        }
    }
    path.len() as u32
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

    // let lines1 = vec!["R 5",
    //                   "U 8",
    //                   "L 8",
    //                   "D 3",
    //                   "R 17",
    //                   "D 10",
    //                   "L 25",
    //                   "U 20"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    println!("{}", part2(&lines));
}
#[cfg(test)]
mod tests {
    use crate::{part1, part2};

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

        let lines1 = vec!["R 5",
                          "U 8",
                          "L 8",
                          "D 3",
                          "R 17",
                          "D 10",
                          "L 25",
                          "U 20"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let result = part2(&lines1);
        assert_eq!(result, 36);
    }
}
