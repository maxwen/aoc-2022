use itertools::Itertools;
use std::cmp::max;
use std::collections::HashSet;
use std::ops::Add;
use aoc_2022::read_lines_as_vec;

// thx to https://gitlab.com/harudagondi/alg-grid/-/blob/82a4951f244f8fb86f550970d1807971f8477e4c/src/three_dim.rs
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[derive(Hash)]
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Add for Point3D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn get_neighbors(point: Point3D) -> [Point3D; 6] {
    [
        // centers
        point + Point3D {
            x: 0,
            y: 1,
            z: 0,
        },
        point + Point3D {
            x: 0,
            y: -1,
            z: 0,
        },
        point + Point3D {
            x: 1,
            y: 0,
            z: 0,
        },
        point + Point3D {
            x: -1,
            y: 0,
            z: 0,
        },
        point + Point3D {
            x: 0,
            y: 0,
            z: 1,
        },
        point + Point3D {
            x: 0,
            y: 0,
            z: -1,
        },
    ]
}

fn is_neighbor(p1: Point3D, p2: Point3D) -> bool {
    get_neighbors(p1)
        .iter()
        .any(|&x| x == p2)
}

fn part1(lines: &[String]) -> u32 {
    // 3522
    let mut cubes = HashSet::new();

    for (_, coords) in lines.iter().enumerate() {
        let cube_coords = coords.split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        let x = *cube_coords.get(0).unwrap();
        let y = *cube_coords.get(1).unwrap();
        let z = *cube_coords.get(2).unwrap();
        let cube = Point3D {
            x,
            y,
            z,
        };
        cubes.insert(cube);
    }
    let open_side = get_open_sides(&cubes);
    open_side as u32
}

fn get_enclosed_cubes(cubes: &HashSet<Point3D>, size: i32) -> HashSet<Point3D> {
    let mut enclosed_cubes = HashSet::new();
    for x in 0..size {
        for y in 0..size {
            for z in 0..size {
                let c = Point3D {
                    x,
                    y,
                    z,
                };
                // must be air
                if !cubes.contains(&c) {
                    let n = get_neighbors(c);
                    let mut enclosed = true;
                    for cn in n {
                        if !cubes.contains(&cn) {
                            enclosed = false;
                            break;
                        }
                    }
                    if enclosed {
                        enclosed_cubes.insert(c);
                    }
                }
            }
        }
    }
    enclosed_cubes
}

fn get_open_sides(cubes: &HashSet<Point3D>) -> usize {
    let mut open_side = cubes.len() * 6;

    for pair in cubes.iter().combinations(2) {
        let p1 = *pair.first().unwrap();
        let p2 = *pair.last().unwrap();
        if is_neighbor(*p1, *p2) {
            open_side -= 2;
        }
    }

    open_side
}

fn part2(lines: &[String]) -> u32 {
    // < 3330
    let mut cubes = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    // let mut open_sides = part1(lines);

    for (_, coords) in lines.iter().enumerate() {
        let cube_coords = coords.split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        let x = *cube_coords.get(0).unwrap();
        let y = *cube_coords.get(1).unwrap();
        let z = *cube_coords.get(2).unwrap();

        max_x = max(max_x, x);
        max_y = max(max_y, y);
        max_z = max(max_z, z);

        let cube = Point3D {
            x,
            y,
            z,
        };
        cubes.insert(cube);
    }
    let enclosed_cubes = get_enclosed_cubes(&cubes, max_x);
    println!("{:?}", enclosed_cubes);

    for c in enclosed_cubes {
        cubes.insert(c);
    }

    let open_side = get_open_sides(&cubes);
    open_side as u32
}

fn main() {
    let lines = read_lines_as_vec("input/input_day18.txt").unwrap();

    // let lines = vec!["2,2,2",
    //                  "1,2,2",
    //                  "3,2,2",
    //                  "2,1,2",
    //                  "2,3,2",
    //                  "2,2,1",
    //                  "2,2,3",
    //                  "2,2,4",
    //                  "2,2,6",
    //                  "1,2,5",
    //                  "3,2,5",
    //                  "2,1,5",
    //                  "2,3,5"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

    println!("{}", part1(&lines));
    // println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec!["2,2,2",
                         "1,2,2",
                         "3,2,2",
                         "2,1,2",
                         "2,3,2",
                         "2,2,1",
                         "2,2,3",
                         "2,2,4",
                         "2,2,6",
                         "1,2,5",
                         "3,2,5",
                         "2,1,5",
                         "2,3,5"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 64);
        let result = part2(&lines);
        assert_eq!(result, 58);
    }
}
