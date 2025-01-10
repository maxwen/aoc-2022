use priority_queue::PriorityQueue;
use std::collections::{HashMap};
use aoc_2022::read_lines_as_vec;

#[derive(Copy)]
#[derive(Clone)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

fn get_height(c: char) -> usize {
    c.to_ascii_lowercase() as usize - 96
}

fn is_possible_step(grid: &Vec<Vec<usize>>, height: usize, pos: (i32, i32)) -> bool {
    let grid_lines = grid.len() as i32;
    let grid_cols = grid.first().unwrap().len() as i32;
    let pos_valid = pos.0 >= 0 && pos.0 < grid_cols && pos.1 >= 0 && pos.1 < grid_lines;
    if pos_valid {
        let new_height = grid[pos.1 as usize][pos.0 as usize];
        return new_height <= height + 1;
    }
    false
}

fn is_possible_step2(grid: &Vec<Vec<usize>>, height: usize, pos: (i32, i32)) -> bool {
    let grid_lines = grid.len() as i32;
    let grid_cols = grid.first().unwrap().len() as i32;
    let pos_valid = pos.0 >= 0 && pos.0 < grid_cols && pos.1 >= 0 && pos.1 < grid_lines;
    if pos_valid {
        let new_height = grid[pos.1 as usize][pos.0 as usize];
        return new_height > height || new_height == height - 1 || new_height == height;
    }
    false
}

fn get_next_pos(pos: (i32, i32), d: Direction) -> (i32, i32) {
    match d {
        Direction::Up => (pos.0, pos.1 + 1),
        Direction::Down => (pos.0, pos.1 - 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}

fn dijkstra(grid: &Vec<Vec<usize>>, start: (i32, i32), end: (i32, i32)) -> usize {
    let directions = vec![Direction::Down, Direction::Up, Direction::Left, Direction::Right];
    let mut stack = PriorityQueue::new();
    stack.push(start, 0);

    let mut seen: HashMap<(i32, i32), usize> = HashMap::new();
    seen.insert(start, 0);

    let mut min = usize::MAX;

    while !stack.is_empty() {
        let (current, steps) = stack.pop().unwrap();
        if current == end {
            if steps < min {
                min = steps
            }
        }

        let current_height = grid[current.1 as usize][current.0 as usize];
        for d in &directions {
            let next_pos = get_next_pos(current, *d);
            if is_possible_step(grid, current_height, next_pos) {

                let dist_next_pos = seen.get(&next_pos).unwrap_or(&usize::MAX);
                if steps + 1 < *dist_next_pos {
                    seen.insert(next_pos, steps + 1);
                    stack.push(next_pos, steps + 1);
                }
            }
        }
    }
    min
}

fn part1(lines: &[String]) -> usize {
    // 447
    let mut grid: Vec<Vec<usize>> = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        let mut l = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x as i32, y as i32);
                let height = get_height('a');
                l.push(height);
            } else if c == 'E' {
                end = (x as i32, y as i32);
                let height = get_height('z');
                l.push(height);
            } else {
                let height = get_height(c);
                l.push(height);
            }
        }
        grid.push(l);
    }
    dijkstra(&grid, start, end)
}

fn dijkstra2(grid: &Vec<Vec<usize>>, start: (i32, i32), end: (i32, i32)) -> usize {
    let directions = vec![Direction::Down, Direction::Up, Direction::Left, Direction::Right];
    let end_height = grid[end.1 as usize][end.0 as usize];
    let mut stack = PriorityQueue::new();
    stack.push(start, 0);

    let mut seen: HashMap<(i32, i32), usize> = HashMap::new();
    seen.insert(start, 0);

    let mut min = usize::MAX;

    while !stack.is_empty() {
        let (current, steps) = stack.pop().unwrap();
        let current_height = grid[current.1 as usize][current.0 as usize];

        if current_height == end_height {
            if steps < min {
                min = steps
            }
        }

        for d in &directions {
            let next_pos = get_next_pos(current, *d);
            if is_possible_step2(grid, current_height, next_pos) {

                let dist_next_pos = seen.get(&next_pos).unwrap_or(&usize::MAX);
                if steps + 1 < *dist_next_pos {
                    seen.insert(next_pos, steps + 1);
                    stack.push(next_pos, steps + 1);
                }
            }
        }
    }
    min
}

// search reverse
fn part2(lines: &[String]) -> usize {
    // 446
    let mut grid: Vec<Vec<usize>> = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        let mut l = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x as i32, y as i32);
                let height = get_height('a');
                l.push(height);
            } else if c == 'E' {
                end = (x as i32, y as i32);
                let height = get_height('z');
                l.push(height);
            } else {
                let height = get_height(c);
                l.push(height);
            }
        }
        grid.push(l);
    }
    dijkstra2(&grid, end, start)
}


fn main() {
    let lines = read_lines_as_vec("input/input_day12.txt").unwrap();
    // let lines = vec!["Sabqponm",
    //                  "abcryxxl",
    //                  "accszExk",
    //                  "acctuvwj",
    //                  "abdefghi"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc_2022::read_lines_as_vec;

    #[test]
    fn it_works() {
        let lines = vec!["Sabqponm",
                         "abcryxxl",
                         "accszExk",
                         "acctuvwj",
                         "abdefghi"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 31);
        let result = part2(&lines);
        assert_eq!(result, 29);
    }
}
