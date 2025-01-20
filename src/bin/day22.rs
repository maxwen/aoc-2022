use regex::Regex;
use std::cmp::{max, PartialEq};
use std::collections::HashMap;
use aoc_2022::read_lines_as_vec;

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_direction_value(direction: &Direction) -> usize {
    match direction {
        Direction::Up => 3,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 0
    }
}
#[derive(Debug, Clone)]
enum Order {
    Move(usize),
    Right,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Space,
    Wall,
}

#[derive(Debug, Clone)]
struct GridLine {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone)]
struct GridColumn {
    start: usize,
    end: usize,
}

fn add_direction_clockwise(direction: &Direction) -> Direction {
    match direction {
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
    }
}

fn add_direction_counterclockwise(direction: &Direction) -> Direction {
    match direction {
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
    }
}

// TODO need to add wrapping
fn get_pos_in_direction(pos: (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (pos.0, pos.1 + 1),
        Direction::Down => (pos.0, pos.1 - 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}

fn is_row_in_void(grid_lines: &HashMap<usize, GridLine>, pos: (usize, usize)) -> bool {
    let grid_line = grid_lines.get(&pos.1).unwrap();
    if grid_line.start > pos.0 {
        return true;
    }
    if grid_line.end < pos.0 {
        return true;
    }
    false
}

fn get_possible_pos_in_direction(grid: &HashMap<(usize, usize), Tile>,
                                 grid_lines: &HashMap<usize, GridLine>,
                                 grid_columns: &HashMap<usize, GridColumn>,
                                 steps: usize,
                                 pos: (usize, usize),
                                 direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => {
            let mut move_pos = pos;
            for _ in 0..steps {
                let old_pos = move_pos;
                let grid_columns = grid_columns.get(&move_pos.0).unwrap();
                if move_pos.1 == grid_columns.start {
                    move_pos = (move_pos.0, grid_columns.end);
                } else {
                    move_pos = (move_pos.0, move_pos.1 - 1);
                }
                if grid.get(&move_pos).unwrap() == &Tile::Wall {
                    return (old_pos.0, old_pos.1);
                }
            }
            move_pos
        }
        Direction::Down => {
            let mut move_pos = pos;
            for _ in 0..steps {
                let old_pos = move_pos;
                let grid_columns = grid_columns.get(&move_pos.0).unwrap();
                if move_pos.1 == grid_columns.end {
                    move_pos = (move_pos.0, grid_columns.start);
                } else {
                    move_pos = (move_pos.0, move_pos.1 + 1);
                }
                if grid.get(&move_pos).unwrap() == &Tile::Wall {
                    return (old_pos.0, old_pos.1);
                }
            }
            move_pos
        }
        Direction::Left => {
            let mut move_pos = pos;
            for _ in 0..steps {
                // wrap around
                let old_pos = move_pos;
                let grid_line = grid_lines.get(&move_pos.1).unwrap();
                if move_pos.0 == grid_line.start {
                    move_pos = (grid_line.end, move_pos.1);
                } else {
                    move_pos = (move_pos.0 - 1, move_pos.1);
                }
                if grid.get(&move_pos).unwrap() == &Tile::Wall {
                    return (old_pos.0, old_pos.1);
                }
            }
            move_pos
        }
        Direction::Right => {
            let mut move_pos = pos;
            for _ in 0..steps {
                let old_pos = move_pos;
                let grid_line = grid_lines.get(&move_pos.1).unwrap();
                if move_pos.0 == grid_line.end {
                    move_pos = (grid_line.start, move_pos.1);
                } else {
                    move_pos = (move_pos.0 + 1, move_pos.1);
                }
                if grid.get(&move_pos).unwrap() == &Tile::Wall {
                    return (old_pos.0, old_pos.1);
                }
            }
            move_pos
        }
    }
}

fn part1(lines: &[String]) -> usize {
    // 75388
    let mut path_list: Vec<Order> = vec![];
    let mut grid: HashMap<(usize, usize), Tile> = HashMap::new();
    let mut grid_lines: HashMap<usize, GridLine> = HashMap::new();
    let mut grid_columns: HashMap<usize, GridColumn> = HashMap::new();
    let mut max_x = 0;
    let mut path_idx = 0;
    for (y, line) in lines.iter().enumerate() {
        if line.len() == 0 {
            path_idx = y + 1;
            break;
        }

        let mut start_x: i32 = -1;
        let mut end_x: i32 = -1;
        for (x, c) in line.chars().enumerate() {
            let pos = (x, y);
            if c == '#' {
                if start_x == -1 {
                    start_x = x as i32;
                }
                grid.insert(pos, Tile::Wall);
            } else if c == '.' {
                if start_x == -1 {
                    start_x = x as i32;
                }
                grid.insert(pos, Tile::Space);
            }
        }
        end_x = (line.len() - 1) as i32;
        max_x = max(end_x, max_x);

        let grid_line = GridLine {
            start: start_x as usize,
            end: end_x as usize,
        };
        grid_lines.insert(y, grid_line);
    }


    for x in 0..max_x + 1 {
        let mut start_y: i32 = -1;
        let mut end_y: i32 = -1;
        for y in 0..grid_lines.len() {
            let line = grid_lines.get(&y).unwrap();
            if line.start <= x as usize && line.end >= x as usize {
                if start_y == -1 {
                    start_y = y as i32;
                }
            } else {
                if start_y != -1 && end_y == -1 {
                    end_y = (y - 1) as i32;
                }
            }
        }
        if start_y != -1 && end_y == -1 {
            end_y = (grid_lines.len() - 1) as i32;
        }
        if start_y != -1 && end_y != -1 {
            let r = GridColumn {
                start: start_y as usize,
                end: end_y as usize,
            };
            grid_columns.insert(x as usize, r);
        }
    }
    // println!("{:?}", grid_lines);
    // println!("{:?}", grid_columns);

    let re = Regex::new(r"(\d+|L|R)").unwrap(); // \d means digit
    let path = &lines[path_idx];
    // println!("{}", path);

    let moves = re.find_iter(path).collect::<Vec<_>>();
    for m in moves {
        let order = m.as_str();
        match order {
            "L" => path_list.push(Order::Left),
            "R" => path_list.push(Order::Right),
            _ => {
                let num: usize = order.parse().unwrap();
                path_list.push(Order::Move(num))
            }
        }
    }
    // println!("{:?}", path_list);

    let mut current_pos = ((grid_lines.get(&0).unwrap().start, 0), Direction::Right);

    for order in path_list.iter() {
        // println!("{:?} {:?}", order, current_pos);
        match order {
            Order::Move(steps) => {
                let new_pos = get_possible_pos_in_direction(&grid, &grid_lines, &grid_columns,
                                                            *steps, current_pos.0, &current_pos.1);
                current_pos = (new_pos, current_pos.1);
            }
            Order::Right => {
                let d = add_direction_clockwise(&current_pos.1);
                current_pos = (current_pos.0, d);
            }
            Order::Left => {
                let d = add_direction_counterclockwise(&current_pos.1);
                current_pos = (current_pos.0, d);
            }
        }
    }
    (current_pos.0.0 + 1) * 4 + (current_pos.0.1 + 1) * 1000 + get_direction_value(&current_pos.1)
}

fn part2(lines: &[String]) -> usize {
    0usize
}

fn main() {
    let lines = read_lines_as_vec("input/input_day22.txt").unwrap();

    // let lines = vec!["        ...#",
    //                  "        .#..",
    //                  "        #...",
    //                  "        ....",
    //                  "...#.......#",
    //                  "........#...",
    //                  "..#....#....",
    //                  "..........#.",
    //                  "        ...#....",
    //                  "        .....#..",
    //                  "        .#......",
    //                  "        ......#.",
    //                  "",
    //                  "10R5L5R10L4R5L5"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec!["        ...#",
                         "        .#..",
                         "        #...",
                         "        ....",
                         "...#.......#",
                         "........#...",
                         "..#....#....",
                         "..........#.",
                         "        ...#....",
                         "        .....#..",
                         "        .#......",
                         "        ......#.",
                         "",
                         "10R5L5R10L4R5L5"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 6032);
        // let result = part2(&lines);
        // assert_eq!(result, 301);
    }
}
