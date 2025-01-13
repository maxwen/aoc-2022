use aoc_2022::read_lines_as_vec;
use range_collections::{RangeSet, RangeSet2};
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Range};

// just for fun visualizing the test data
#[allow(dead_code)]
#[derive(Debug)]
enum Tile {
    S,
    Beacon,
    Air,
}

#[derive(Debug)]
struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
}

impl Sensor {
    fn manhatten_distance(&self) -> i32 {
        (self.pos.0 - self.beacon.0).abs() + (self.pos.1 - self.beacon.1).abs()
    }
    fn get_coverage_x_area(&self, y: i32) -> (i32, i32) {
        let d = self.manhatten_distance();
        let y_diff = (self.pos.1 - y).abs();
        if y_diff < d {
            // inside the area
            let x_diff = d - y_diff;
            return (self.pos.0 - x_diff, self.pos.0 + x_diff);
        }
        (0, 0)
    }

    #[allow(dead_code)]
    fn get_coverage_y_area(&self, x: i32) -> (i32, i32) {
        let d = self.manhatten_distance();
        let x_diff = (self.pos.0 - x).abs();
        if x_diff < d {
            // inside the area
            let y_diff = d - x_diff;
            return (self.pos.1 - y_diff, self.pos.1 + y_diff);
        }
        (0, 0)
    }
}

#[derive(Debug)]
struct Grid {
    // just for fun visualizing the test data
    data: HashMap<(i32, i32), Tile>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    sensors: HashMap<(i32, i32), Sensor>,
}
impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::S => { write!(f, "S") }
            Tile::Beacon => { write!(f, "B") }
            Tile::Air => { write!(f, ".") }
        }
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    for y in grid.min_y..grid.max_y + 1 {
        for x in grid.min_x..grid.max_x + 1 {
            if let Some(tile) = grid.data.get(&(x, y)) {
                print!("{}", tile)
            } else {
                print!("{}", Tile::Air)
            }
        }
        println!();
    }
}

fn part1(lines: &[String], y: i32) -> usize {
    // 5108096
    let mut grid = Grid {
        data: HashMap::new(),
        min_x: i32::MAX,
        max_x: 0,
        min_y: i32::MAX,
        max_y: 0,
        sensors: HashMap::new(),
    };

    build_grid(&mut grid, &lines);
    // print_grid(&grid);

    let mut min_x = i32::MAX;
    let mut max_x = 0;
    for s in grid.sensors.values() {
        // println!("{:?} - {}", s, s.manhatten_distance());
        let x_range = s.get_coverage_x_area(y);
        if x_range != (0, 0) {
            // println!("{:?} {:?}", s, x_range);
            min_x = min(min_x, x_range.0);
            max_x = max(max_x, x_range.1);
        }
    }

    (min_x..max_x).len()
}


fn part2(lines: &[String], max_pos: i32) -> u64 {
    // 10553942650264
    let mut grid = Grid {
        data: HashMap::new(),
        min_x: i32::MAX,
        max_x: 0,
        min_y: i32::MAX,
        max_y: 0,
        sensors: HashMap::new(),
    };

    build_grid(&mut grid, &lines);

    // fortunately there can only be one line so we can stop
    // right there
    for y in 0..max_pos + 1 {
        // use ranges to build possible values of x where it can be
        // if we found a y where there is a x possible we are done
        let mut x_max_area_range: Vec<Range<i32>> = vec![0..max_pos];
        let mut x_max_area_range_new: Vec<Range<i32>> = vec![];

        for s in grid.sensors.values() {
            let x_coverage = s.get_coverage_x_area(y);

            // println!("y ={} x_max_area_range = {:?} {:?}", y, x_max_area_range, x_coverage);
            if x_coverage != (0, 0) {
                // stupid me a range is always < last so we need to add + 1 here
                let x_area_range: RangeSet2<i32> = RangeSet::from(x_coverage.0..x_coverage.1 + 1);
                for r in &x_max_area_range {
                    let r_range: RangeSet2<i32> = RangeSet::from(r.start..r.end);

                    let r_diff = r_range.clone() - x_area_range.clone();
                    if !r_diff.is_empty() {
                        let r_diff_boundaries = r_diff.boundaries();

                        // println!("{} {:?} {:?} {:?}", y, x_area_range, r_range, r_diff);

                        // split into two
                        if r_diff_boundaries.len() == 4 {
                            x_max_area_range_new.push(r_diff_boundaries[0]..r_diff_boundaries[1]);
                            x_max_area_range_new.push(r_diff_boundaries[2]..r_diff_boundaries[3]);
                        } else {
                            x_max_area_range_new.push(r_diff_boundaries[0]..r_diff_boundaries[1]);
                        }
                    }
                }
                x_max_area_range.clear();
                x_max_area_range.append(&mut x_max_area_range_new);
            }
        }
        if !x_max_area_range.is_empty() {
            // if x range is not empty we found the line
            return x_max_area_range.first().unwrap().start as u64 * 4000000u64 + y as u64;
        }
    }

    0u64
}

fn build_grid(grid: &mut Grid, lines: &[String]) {
    let re = Regex::new(r"-?\d+").unwrap();

    for (_, line) in lines.iter().enumerate() {
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let digits = re.find_iter(line).collect::<Vec<_>>();
        let x1: i32 = digits.get(0).unwrap().as_str().parse().unwrap();
        let y1: i32 = digits.get(1).unwrap().as_str().parse().unwrap();

        grid.data.insert((x1, y1), Tile::S);

        grid.min_x = min(grid.min_x, x1);
        grid.max_x = max(grid.max_x, x1);

        grid.min_y = min(grid.min_y, y1);
        grid.max_y = max(grid.max_y, y1);

        let x2: i32 = digits.get(2).unwrap().as_str().parse().unwrap();
        let y2: i32 = digits.get(3).unwrap().as_str().parse().unwrap();
        grid.data.insert((x2, y2), Tile::Beacon);

        grid.min_x = min(grid.min_x, x2);
        grid.max_x = max(grid.max_x, x2);

        grid.min_y = min(grid.min_y, y2);
        grid.max_y = max(grid.max_y, y2);

        let s = Sensor {
            pos: (x1, y1),
            beacon: (x2, y2),
        };
        grid.sensors.insert((x1, y1), s);
    }
}

fn main() {
    let lines = read_lines_as_vec("input/input_day15.txt").unwrap();

    // let lines = read_lines_as_vec("input_test/input_day15_test.txt").unwrap();

    println!("{}", part1(&lines, 2000000));
    println!("{}", part2(&lines, 4000000));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc_2022::read_lines_as_vec;

    #[test]
    fn it_works() {
        let lines = read_lines_as_vec("input_test/input_day15_test.txt").unwrap();

        let result = part1(&lines, 10);
        assert_eq!(result, 26);
        let result = part2(&lines, 20);
        assert_eq!(result, 56000011);
    }
}
