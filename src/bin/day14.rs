use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use aoc_2022::read_lines_as_vec;

enum Tile {
    Rock,
    Air,
    Sand,
}

struct Grid {
    data: HashMap<(i32, i32), Tile>,
    min_x: i32,
    max_x: i32,
    max_y: i32,
}
impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Rock => { write!(f, "#") }
            Tile::Air => { write!(f, ".") }
            Tile::Sand => { write!(f, "o") }
        }
    }
}

fn mark_as_rock(grid: &mut Grid, start: (i32, i32), end: (i32, i32)) {
    let mut start_x = vec![start.0, end.0];
    start_x.sort();
    for x in *start_x.first().unwrap()..*start_x.last().unwrap() + 1 {
        let p = (x, start.1);
        grid.data.insert(p, Tile::Rock);
    }
    let mut start_y = vec![start.1, end.1];
    start_y.sort();
    for y in *start_y.first().unwrap()..*start_y.last().unwrap() + 1 {
        let p = (start.0, y);
        grid.data.insert(p, Tile::Rock);
    }
}

fn print_grid(grid: &Grid, start: (i32, i32)) {
    for y in 0..grid.max_y + 1 {
        for x in grid.min_x..grid.max_x + 1 {
            if (x, y) == start {
                print!("+")
            } else if let Some(tile) = grid.data.get(&(x, y)) {
                print!("{}", tile)
            } else {
                print!("{}", Tile::Air)
            }
        }
        println!();
    }
}

// fn is_possible_move(grid: &Grid, pos: (i32, i32)) -> bool {
//     pos.0 >= grid.min_x && pos.0 <= grid.max_x && pos.1 <= grid.max_y
// }

fn try_move_sand(grid: &mut Grid, pos: (i32, i32)) -> Option<(i32, i32)> {
    let down = (pos.0, pos.1 + 1);
    if grid.data.get(&down).is_none() {
        grid.data.remove(&pos);
        grid.data.insert(down, Tile::Sand);
        return Some(down);
    }

    let down_left = (pos.0 - 1, pos.1 + 1);
    if grid.data.get(&down_left).is_none() {
        grid.data.remove(&pos);
        grid.data.insert(down_left, Tile::Sand);
        return Some(down_left);
    }

    let down_right = (pos.0 + 1, pos.1 + 1);
    if grid.data.get(&down_right).is_none() {
        grid.data.remove(&pos);
        grid.data.insert(down_right, Tile::Sand);
        return Some(down_right);
    }

    None
}

fn part1(lines: &[String]) -> u32 {
    // 1406
    let mut grid = Grid {
        data: HashMap::new(),
        min_x: i32::MAX,
        max_x: 0,
        max_y: 0,
    };
    let start = (500, 0);

    for (_, line) in lines.iter().enumerate() {
        let mut last: Option<(i32, i32)> = None;
        for (_, ccords) in line.split(" -> ").enumerate() {
            let pair = ccords.split(",").collect::<Vec<_>>();
            let x: i32 = pair.first().unwrap().parse().unwrap();
            let y: i32 = pair.last().unwrap().parse().unwrap();
            grid.max_y = max(grid.max_y, y);
            grid.min_x = min(grid.min_x, x);
            grid.max_x = max(grid.max_x, x);

            if last.is_some() {
                mark_as_rock(&mut grid, last.unwrap(), (x, y))
            }
            last = Some((x, y))
        }
    }

    // print_grid(&grid, start);

    let mut overflow = false;
    let mut count = 0;
    while !overflow {
        let mut sand_corn = start;

        loop {
            match try_move_sand(&mut grid, sand_corn) {
                None => {
                    count += 1;
                    // print_grid(&grid, start);
                    break;
                }
                Some(new_pos) => {
                    sand_corn = new_pos;
                    if new_pos.1 > grid.max_y {
                        // print_grid(&grid, start);
                        overflow = true;
                        break;
                    }
                }
            }
        }
    }
    // print_grid(&grid, start);
    count
}


fn part2(lines: &[String]) -> u32 {
    0u32
}


fn main() {
    let lines = read_lines_as_vec("input/input_day14.txt").unwrap();

    // let lines = vec!["498,4 -> 498,6 -> 496,6",
    //                  "503,4 -> 502,4 -> 502,9 -> 494,9"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec!["498,4 -> 498,6 -> 496,6",
                         "503,4 -> 502,4 -> 502,9 -> 494,9"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 24);
        // let result = part2(&lines);
        // assert_eq!(result, 36);
    }
}
