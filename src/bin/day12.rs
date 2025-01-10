use aoc_2022::read_lines_as_vec;
use priority_queue::PriorityQueue;
use std::collections::HashMap;

fn get_height(c: char) -> usize {
    c.to_ascii_lowercase() as usize - 96
}

fn get_neighbours(grid: &Vec<Vec<usize>>, pos: (i32, i32)) -> Vec<(i32, i32)> {
    let grid_lines = grid.len() as i32;
    let grid_cols = grid.first().unwrap().len() as i32;

    [(pos.0, pos.1 + 1), (pos.0, pos.1 - 1), (pos.0 - 1, pos.1), (pos.0 + 1, pos.1)]
        .iter()
        .filter(|pos| pos.0 >= 0 && pos.0 < grid_cols && pos.1 >= 0 && pos.1 < grid_lines)
        .map(|(r, c)| (*r, *c))
        .collect::<Vec<_>>()
}

fn get_next_possible_pos2(grid: &Vec<Vec<usize>>, current_height: usize, pos: (i32, i32)) -> Vec<(i32, i32)> {
    let candidates = get_neighbours(grid, pos);

    let mut pos_list = vec![];
    for next_pos in candidates {
        let new_height = grid[next_pos.1 as usize][next_pos.0 as usize];
        if new_height > current_height || new_height == current_height - 1 || new_height == current_height {
            pos_list.push(next_pos);
        }
    }
    pos_list
}

fn get_next_possible_pos(grid: &Vec<Vec<usize>>, current_height: usize, pos: (i32, i32)) -> Vec<(i32, i32)> {
    let candidates = get_neighbours(grid, pos);

    let mut pos_list = vec![];
    for next_pos in candidates {
        let new_height = grid[next_pos.1 as usize][next_pos.0 as usize];
        if new_height <= current_height + 1 {
            pos_list.push(next_pos);
        }
    }
    pos_list
}


fn dijkstra(grid: &Vec<Vec<usize>>, start: (i32, i32), end: (i32, i32)) -> usize {
    let mut stack = PriorityQueue::new();
    stack.push(start, 0);

    let mut seen: HashMap<(i32, i32), usize> = HashMap::new();
    seen.insert(start, 0);

    let mut min = usize::MAX;

    while let Some((current, steps)) = stack.pop() {
        if current == end {
            if steps < min {
                min = steps
            }
        }

        let current_height = grid[current.1 as usize][current.0 as usize];
        for next_pos in get_next_possible_pos(grid, current_height, current) {
            let dist_next_pos = seen.get(&next_pos).unwrap_or(&usize::MAX);
            if steps + 1 < *dist_next_pos {
                seen.insert(next_pos, steps + 1);
                stack.push(next_pos, steps + 1);
            }
        }
    }
    min
}

fn dijkstra2(grid: &Vec<Vec<usize>>, start: (i32, i32), end: (i32, i32)) -> usize {
    let end_height = grid[end.1 as usize][end.0 as usize];
    let mut stack = PriorityQueue::new();
    stack.push(start, 0);

    let mut seen: HashMap<(i32, i32), usize> = HashMap::new();
    seen.insert(start, 0);

    let mut min = usize::MAX;

    while let Some((current, steps)) = stack.pop() {
        let current_height = grid[current.1 as usize][current.0 as usize];

        if current_height == end_height {
            if steps < min {
                min = steps
            }
        }

        for next_pos in get_next_possible_pos2(grid, current_height, current) {
            let dist_next_pos = seen.get(&next_pos).unwrap_or(&usize::MAX);
            if steps + 1 < *dist_next_pos {
                seen.insert(next_pos, steps + 1);
                stack.push(next_pos, steps + 1);
            }
        }
    }
    min
}

// part2: search reverse
fn part12(lines: &[String]) -> (usize, usize) {
    // 447,446
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
    let part1 = dijkstra(&grid, start, end);
    let part2 = dijkstra2(&grid, end, start);
    (part1, part2)
}

fn main() {
    let lines = read_lines_as_vec("input/input_day12.txt").unwrap();
    // let lines = vec!["Sabqponm",
    //                  "abcryxxl",
    //                  "accszExk",
    //                  "acctuvwj",
    //                  "abdefghi"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

    let res = part12(&lines);
    println!("{}", res.0);
    println!("{}", res.1);
}

#[cfg(test)]
mod tests {
    use crate::part12;

    #[test]
    fn it_works() {
        let lines = vec!["Sabqponm",
                         "abcryxxl",
                         "accszExk",
                         "acctuvwj",
                         "abdefghi"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let res = part12(&lines);
        assert_eq!(res.0, 31);
        assert_eq!(res.1, 29);
    }
}
