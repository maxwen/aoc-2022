use aoc_2022::read_lines_as_vec;
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashSet;
use std::ops::Add;

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

fn get_enclosed_cubes(cubes: &HashSet<Point3D>, max_x: i32, max_y: i32, max_z: i32) -> HashSet<Point3D> {
    let mut enclosed_cubes = HashSet::new();
    for z in 0..max_z + 1 {
        for y in 0..max_y + 1 {
            for x in 0..max_x + 1 {
                let c = Point3D {
                    x,
                    y,
                    z,
                };
                // must be air
                if !cubes.contains(&c) && !enclosed_cubes.contains(&c) {
                    if is_enclosed_air(cubes, &c, max_x, max_y, max_z) {
                        enclosed_cubes.insert(c);
                    }
                }
            }
        }
    }
    enclosed_cubes
}

fn is_enclosed_air(cubes: &HashSet<Point3D>, cube: &Point3D, max_x: i32, max_y: i32, max_z: i32) -> bool {
    let this_x = cube.x;

    let mut enclosed_left = false;
    let mut enclosed_right = false;

    for x in (0..this_x).rev() {
        let c = Point3D {
            x,
            y: cube.y,
            z: cube.z,
        };
        if cubes.contains(&c) {
            enclosed_left = true;
            break;
        }
    }
    for x in this_x..max_x + 1 {
        let c = Point3D {
            x,
            y: cube.y,
            z: cube.z,
        };
        if cubes.contains(&c) {
            enclosed_right = true;
            break;
        }
    }
    if !(enclosed_left && enclosed_right) {
        return false;
    }

    let this_y = cube.y;

    let mut enclosed_left = false;
    let mut enclosed_right = false;

    for y in (0..this_y).rev() {
        let c = Point3D {
            x: cube.x,
            y,
            z: cube.z,
        };
        if cubes.contains(&c) {
            enclosed_left = true;
            break;
        }
    }
    for y in this_y..max_y + 1 {
        let c = Point3D {
            x: cube.x,
            y,
            z: cube.z,
        };
        if cubes.contains(&c) {
            enclosed_right = true;
            break;
        }
    }
    if !(enclosed_left && enclosed_right) {
        return false;
    }

    let this_z = cube.z;

    let mut enclosed_left = false;
    let mut enclosed_right = false;

    for z in (0..this_z).rev() {
        let c = Point3D {
            x: cube.x,
            y: cube.y,
            z,
        };
        if cubes.contains(&c) {
            enclosed_left = true;
            break;
        }
    }
    for z in this_z..max_z + 1 {
        let c = Point3D {
            x: cube.x,
            y: cube.y,
            z,
        };
        if cubes.contains(&c) {
            enclosed_right = true;
            break;
        }
    }
    if !(enclosed_left && enclosed_right) {
        return false;
    }

    true
}

#[warn(dead_code)]
fn print_cube(cubes: &HashSet<Point3D>, enclosed_cubes: &HashSet<Point3D>, max_x: i32, max_y: i32, max_z: i32) {
    for z in 0..max_z + 1 {
        println!("{}", z);
        for y in 0..max_y + 1 {
            for x in 0..max_x + 1 {
                let c = Point3D {
                    x,
                    y,
                    z,
                };
                if enclosed_cubes.contains(&c) {
                    print!("@")
                } else if cubes.contains(&c) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
        println!();
    }
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

fn filter_wrong_enclosed(cubes: &HashSet<Point3D>, enclosed_cubes: &HashSet<Point3D>) -> HashSet<Point3D> {
    let mut false_positives = HashSet::new();
    for &e in enclosed_cubes {
        if !get_neighbors(e)
            .iter()
            .all(|&x| cubes.contains(&x) || enclosed_cubes.contains(&x)) {
            false_positives.insert(e);
        }
    }
    false_positives
}

fn part2(lines: &[String]) -> u32 {
    // 2074
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

    let mut enclosed_cubes = get_enclosed_cubes(&cubes, max_x, max_y, max_z);
    // println!("enclosed_cubes {}", enclosed_cubes.len());

    // print_cube(&cubes, &enclosed_cubes, max_x, max_y, max_z);

    let mut false_positives = filter_wrong_enclosed(&cubes, &enclosed_cubes);
    while false_positives.len() != 0 {
        // println!("false_positives {}", false_positives.len());

        enclosed_cubes.retain(|&c| !false_positives.contains(&c));
        false_positives = filter_wrong_enclosed(&cubes, &enclosed_cubes);
    }

    // print_cube(&cubes, &final_enclosed, max_x, max_y, max_z);

    cubes.extend(&enclosed_cubes);

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
    println!("{}", part2(&lines));
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
