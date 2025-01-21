use aoc_2022::read_lines_as_vec;
use regex::Regex;
use std::cmp::{max, PartialEq};
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        Direction::Right => 0,
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

#[derive(Debug, Clone)]
struct CubeFace {
    x_range: Range<usize>,
    y_range: Range<usize>,
    down_face_id: usize,
    up_face_id: usize,
    left_face_id: usize,
    right_face_id: usize,
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

fn get_possible_pos_in_direction(
    grid: &HashMap<(usize, usize), Tile>,
    grid_lines: &HashMap<usize, GridLine>,
    grid_columns: &HashMap<usize, GridColumn>,
    steps: usize,
    pos: (usize, usize),
    direction: &Direction,
) -> (usize, usize) {
    match direction {
        Direction::Up => {
            let mut move_pos = pos;
            for _ in 0..steps {
                let old_pos = move_pos;
                let grid_columns = grid_columns.get(&move_pos.0).unwrap();
                if move_pos.1 == grid_columns.start {
                    // wrap around
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
                    // wrap around
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
                let old_pos = move_pos;
                let grid_line = grid_lines.get(&move_pos.1).unwrap();
                if move_pos.0 == grid_line.start {
                    // wrap around
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
                    // wrap around
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
    let max_y = grid_lines.len() - 1;

    for x in 0..max_x + 1 {
        let mut start_y: i32 = -1;
        let mut end_y: i32 = -1;
        for y in 0..max_y + 1 {
            let line = grid_lines.get(&y).unwrap();
            if line.start <= x as usize && line.end >= x as usize {
                if start_y == -1 {
                    start_y = y as i32;
                }
            } else {
                if start_y != -1 && end_y == -1 {
                    end_y = (y - 1) as i32;
                    break;
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
                let new_pos = get_possible_pos_in_direction(
                    &grid,
                    &grid_lines,
                    &grid_columns,
                    *steps,
                    current_pos.0,
                    &current_pos.1,
                );
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
    (current_pos.0 .0 + 1) * 4 + (current_pos.0 .1 + 1) * 1000 + get_direction_value(&current_pos.1)
}

fn get_cube_face_id_of_pos(
    cube_face_map: &HashMap<usize, CubeFace>,
    pos: (usize, usize),
) -> Option<usize> {
    for cube_face in cube_face_map.iter() {
        let face_id = *cube_face.0;
        let face = cube_face.1;

        if face.x_range.contains(&pos.0) && face.y_range.contains(&pos.1) {
            return Some(face_id);
        }
    }
    // should never happen
    None
}

fn get_position_on_face_transition(
    cube_face_map: &HashMap<usize, CubeFace>,
    cube_face_edge_map: &HashMap<(usize, usize), Direction>,
    edge: (usize, usize),
    pos: (usize, usize),
    direction: &Direction,
) -> ((usize, usize), Direction) {
    let current_face = cube_face_map.get(&edge.0).unwrap();
    let next_face = cube_face_map.get(&edge.1).unwrap();
    let next_face_start_x = next_face.x_range.start;
    let next_face_end_x = next_face.x_range.end - 1;

    let next_face_start_y = next_face.y_range.start;
    let next_face_end_y = next_face.y_range.end - 1;

    let pos_offset_x = pos.0 - current_face.x_range.start;
    let pos_offset_y = pos.1 - current_face.y_range.start;

    let new_direction = cube_face_edge_map.get(&edge).unwrap().clone();
    match direction {
        Direction::Up => match new_direction {
            Direction::Up => (
                (next_face_start_x + pos_offset_x, next_face_end_y),
                new_direction,
            ),
            Direction::Down => (
                (next_face_end_x - pos_offset_x, next_face_start_y),
                new_direction,
            ),
            Direction::Left => {
                let trans_pos = (next_face_end_x, next_face_end_y - pos_offset_x);
                (trans_pos, new_direction)
            }
            Direction::Right => {
                let trans_pos = (next_face_start_x, next_face_start_y + pos_offset_x);
                (trans_pos, new_direction)
            }
        },
        Direction::Down => match new_direction {
            Direction::Up => (
                (next_face_end_x - pos_offset_x, next_face_end_y),
                new_direction,
            ),

            Direction::Down => (
                (next_face_start_x + pos_offset_x, next_face_start_y),
                new_direction,
            ),
            Direction::Left => {
                let trans_pos = (next_face_end_x, next_face_start_y + pos_offset_x);
                (trans_pos, new_direction)
            }
            Direction::Right => {
                let trans_pos = (next_face_start_x, next_face_end_y - pos_offset_x);
                (trans_pos, new_direction)
            }
        },
        Direction::Left => match new_direction {
            Direction::Up => {
                let trans_pos = (next_face_end_x - pos_offset_y, next_face_end_y);
                (trans_pos, new_direction)
            }
            Direction::Down => {
                let trans_pos = (next_face_start_x + pos_offset_y, next_face_start_y);
                (trans_pos, new_direction)
            }
            Direction::Left => (
                (next_face_end_x, next_face_start_y + pos_offset_y),
                new_direction,
            ),
            Direction::Right => (
                (next_face_start_x, next_face_end_y - pos_offset_y),
                new_direction,
            ),
        },
        Direction::Right => match new_direction {
            Direction::Up => {
                let trans_pos = (next_face_start_x + pos_offset_y, next_face_end_y);
                (trans_pos, new_direction)
            }
            Direction::Down => {
                let trans_pos = (next_face_end_x - pos_offset_y, next_face_start_y);
                (trans_pos, new_direction)
            }
            Direction::Left => (
                (next_face_end_x, next_face_end_y - pos_offset_y),
                new_direction,
            ),
            Direction::Right => (
                (next_face_start_x, next_face_start_y + pos_offset_y),
                new_direction,
            ),
        },
    }
}

fn get_possible_position_on_face(
    grid: &HashMap<(usize, usize), Tile>,
    cube_face_map: &HashMap<usize, CubeFace>,
    cube_face_edge_map: &HashMap<(usize, usize), Direction>,
    steps: usize,
    pos: (usize, usize),
    direction: &Direction,
) -> ((usize, usize), Direction) {
    let start_face_id = get_cube_face_id_of_pos(cube_face_map, pos).unwrap();
    let start_face = cube_face_map.get(&start_face_id).unwrap();

    let mut move_pos = (pos, direction.clone());
    for _ in 0..steps {
        let old_pos = move_pos.clone();

        let current_face_id = get_cube_face_id_of_pos(cube_face_map, move_pos.0).unwrap();
        let current_face = cube_face_map.get(&current_face_id).unwrap();

        let move_pos_pos = move_pos.0;
        let move_pos_direction = &move_pos.1;

        match move_pos_direction {
            Direction::Up => {
                if move_pos_pos.1 == current_face.y_range.start {
                    let edge = (current_face_id, current_face.up_face_id);
                    move_pos = get_position_on_face_transition(
                        cube_face_map,
                        cube_face_edge_map,
                        edge,
                        move_pos_pos,
                        move_pos_direction,
                    );
                } else {
                    move_pos = (
                        (move_pos_pos.0, move_pos_pos.1 - 1),
                        move_pos_direction.clone(),
                    )
                }
                if grid.get(&move_pos.0).unwrap() == &Tile::Wall {
                    return old_pos;
                }
            }
            Direction::Down => {
                if move_pos_pos.1 == current_face.y_range.end - 1 {
                    let edge = (current_face_id, current_face.down_face_id);
                    move_pos = get_position_on_face_transition(
                        cube_face_map,
                        cube_face_edge_map,
                        edge,
                        move_pos_pos,
                        move_pos_direction,
                    );
                } else {
                    move_pos = (
                        (move_pos_pos.0, move_pos_pos.1 + 1),
                        move_pos_direction.clone(),
                    )
                }
                if grid.get(&move_pos.0).unwrap() == &Tile::Wall {
                    return old_pos;
                }
            }
            Direction::Left => {
                if move_pos_pos.0 == current_face.x_range.start {
                    let edge = (current_face_id, current_face.left_face_id);
                    move_pos = get_position_on_face_transition(
                        cube_face_map,
                        cube_face_edge_map,
                        edge,
                        move_pos_pos,
                        move_pos_direction,
                    );
                } else {
                    move_pos = (
                        (move_pos_pos.0 - 1, move_pos_pos.1),
                        move_pos_direction.clone(),
                    )
                }
                if grid.get(&move_pos.0).unwrap() == &Tile::Wall {
                    return old_pos;
                }
            }
            Direction::Right => {
                if move_pos_pos.0 == current_face.x_range.end - 1 {
                    let edge = (current_face_id, current_face.right_face_id);
                    move_pos = get_position_on_face_transition(
                        cube_face_map,
                        cube_face_edge_map,
                        edge,
                        move_pos_pos,
                        move_pos_direction,
                    );
                } else {
                    move_pos = (
                        (move_pos_pos.0 + 1, move_pos_pos.1),
                        move_pos_direction.clone(),
                    )
                }
                if grid.get(&move_pos.0).unwrap() == &Tile::Wall {
                    return old_pos;
                }
            }
        }
    }
    move_pos
}

fn define_input_cube(
    max_x: usize,
    max_y: usize,
    cube_face_map: &mut HashMap<usize, CubeFace>,
    cube_face_edge_map: &mut HashMap<(usize, usize), Direction>,
) {
    let cube_edge_x_length = (max_x + 1) / 3;
    let cube_edge_x_0 = 0..cube_edge_x_length;
    let cube_edge_x_1 = cube_edge_x_length..cube_edge_x_length * 2;
    let cube_edge_x_2 = cube_edge_x_length * 2..cube_edge_x_length * 3;

    let cube_edge_y_length = (max_y + 1) / 4;
    let cube_edge_y_0 = 0..cube_edge_y_length;
    let cube_edge_y_1 = cube_edge_y_length..cube_edge_y_length * 2;
    let cube_edge_y_2 = cube_edge_y_length * 2..cube_edge_y_length * 3;
    let cube_edge_y_3 = cube_edge_y_length * 3..cube_edge_y_length * 4;

    // all of this is hard coded specific to my puzzle input
    // this will NOR work with example input

    let cube_face_0 = CubeFace {
        x_range: cube_edge_x_1.clone(),
        y_range: cube_edge_y_0.clone(),
        down_face_id: 2,
        up_face_id: 5,
        left_face_id: 3,
        right_face_id: 1,
    };
    cube_face_map.insert(0, cube_face_0);

    let cube_face_1 = CubeFace {
        x_range: cube_edge_x_2.clone(),
        y_range: cube_edge_y_0.clone(),
        down_face_id: 2,
        up_face_id: 5,
        left_face_id: 0,
        right_face_id: 4,
    };
    cube_face_map.insert(1, cube_face_1);

    let cube_face_2 = CubeFace {
        x_range: cube_edge_x_1.clone(),
        y_range: cube_edge_y_1.clone(),
        down_face_id: 4,
        up_face_id: 0,
        left_face_id: 3,
        right_face_id: 1,
    };
    cube_face_map.insert(2, cube_face_2);

    let cube_face_3 = CubeFace {
        x_range: cube_edge_x_0.clone(),
        y_range: cube_edge_y_2.clone(),
        down_face_id: 5,
        up_face_id: 2,
        left_face_id: 0,
        right_face_id: 4,
    };
    cube_face_map.insert(3, cube_face_3);

    let cube_face_4 = CubeFace {
        x_range: cube_edge_x_1.clone(),
        y_range: cube_edge_y_2.clone(),
        down_face_id: 5,
        up_face_id: 2,
        left_face_id: 3,
        right_face_id: 1,
    };
    cube_face_map.insert(4, cube_face_4);

    let cube_face_5 = CubeFace {
        x_range: cube_edge_x_0.clone(),
        y_range: cube_edge_y_3.clone(),
        down_face_id: 1,
        up_face_id: 3,
        left_face_id: 0,
        right_face_id: 4,
    };
    cube_face_map.insert(5, cube_face_5);

    cube_face_edge_map.insert((0, 1), Direction::Right);
    cube_face_edge_map.insert((0, 2), Direction::Down);
    cube_face_edge_map.insert((0, 3), Direction::Right);
    cube_face_edge_map.insert((0, 5), Direction::Right);

    cube_face_edge_map.insert((1, 0), Direction::Left);
    cube_face_edge_map.insert((1, 5), Direction::Up);
    cube_face_edge_map.insert((1, 2), Direction::Left);
    cube_face_edge_map.insert((1, 4), Direction::Left);

    cube_face_edge_map.insert((2, 0), Direction::Up);
    cube_face_edge_map.insert((2, 3), Direction::Down);
    cube_face_edge_map.insert((2, 4), Direction::Down);
    cube_face_edge_map.insert((2, 1), Direction::Up);

    cube_face_edge_map.insert((3, 2), Direction::Right);
    cube_face_edge_map.insert((3, 0), Direction::Right);
    cube_face_edge_map.insert((3, 5), Direction::Down);
    cube_face_edge_map.insert((3, 4), Direction::Right);

    cube_face_edge_map.insert((4, 2), Direction::Up);
    cube_face_edge_map.insert((4, 3), Direction::Left);
    cube_face_edge_map.insert((4, 5), Direction::Left);
    cube_face_edge_map.insert((4, 1), Direction::Left);

    cube_face_edge_map.insert((5, 3), Direction::Up);
    cube_face_edge_map.insert((5, 0), Direction::Down);
    cube_face_edge_map.insert((5, 1), Direction::Down);
    cube_face_edge_map.insert((5, 4), Direction::Up);
}

fn define_test_input_cube(
    max_x: usize,
    max_y: usize,
    cube_face_map: &mut HashMap<usize, CubeFace>,
    cube_face_edge_map: &mut HashMap<(usize, usize), Direction>,
) {
    let cube_edge_x_length = (max_x + 1) / 4;
    let cube_edge_x_0 = 0..cube_edge_x_length;
    let cube_edge_x_1 = cube_edge_x_length..cube_edge_x_length * 2;
    let cube_edge_x_2 = cube_edge_x_length * 2..cube_edge_x_length * 3;
    let cube_edge_x_3 = cube_edge_x_length * 3..cube_edge_x_length * 4;

    let cube_edge_y_length = (max_y + 1) / 3;
    let cube_edge_y_0 = 0..cube_edge_y_length;
    let cube_edge_y_1 = cube_edge_y_length..cube_edge_y_length * 2;
    let cube_edge_y_2 = cube_edge_y_length * 2..cube_edge_y_length * 3;

    // all of this is hard coded specific to my puzzle input
    // this will NOR work with example input

    let cube_face_0 = CubeFace {
        x_range: cube_edge_x_2.clone(),
        y_range: cube_edge_y_0.clone(),
        down_face_id: 3,
        up_face_id: 1,
        left_face_id: 2,
        right_face_id: 5,
    };
    cube_face_map.insert(0, cube_face_0);

    let cube_face_1 = CubeFace {
        x_range: cube_edge_x_0.clone(),
        y_range: cube_edge_y_1.clone(),
        down_face_id: 4,
        up_face_id: 0,
        left_face_id: 5,
        right_face_id: 2,
    };
    cube_face_map.insert(1, cube_face_1);

    let cube_face_2 = CubeFace {
        x_range: cube_edge_x_1.clone(),
        y_range: cube_edge_y_1.clone(),
        down_face_id: 4,
        up_face_id: 0,
        left_face_id: 1,
        right_face_id: 3,
    };
    cube_face_map.insert(2, cube_face_2);

    let cube_face_3 = CubeFace {
        x_range: cube_edge_x_2.clone(),
        y_range: cube_edge_y_1.clone(),
        down_face_id: 4,
        up_face_id: 0,
        left_face_id: 2,
        right_face_id: 5,
    };
    cube_face_map.insert(3, cube_face_3);

    let cube_face_4 = CubeFace {
        x_range: cube_edge_x_2.clone(),
        y_range: cube_edge_y_2.clone(),
        down_face_id: 1,
        up_face_id: 3,
        left_face_id: 2,
        right_face_id: 5,
    };
    cube_face_map.insert(4, cube_face_4);

    let cube_face_5 = CubeFace {
        x_range: cube_edge_x_3.clone(),
        y_range: cube_edge_y_2.clone(),
        down_face_id: 1,
        up_face_id: 3,
        left_face_id: 4,
        right_face_id: 0,
    };
    cube_face_map.insert(5, cube_face_5);

    cube_face_edge_map.insert((0, 1), Direction::Down);
    cube_face_edge_map.insert((0, 2), Direction::Down);
    cube_face_edge_map.insert((0, 3), Direction::Down);
    cube_face_edge_map.insert((0, 5), Direction::Left);

    cube_face_edge_map.insert((1, 0), Direction::Down);
    cube_face_edge_map.insert((1, 5), Direction::Up);
    cube_face_edge_map.insert((1, 2), Direction::Right);
    cube_face_edge_map.insert((1, 4), Direction::Up);

    cube_face_edge_map.insert((2, 0), Direction::Right);
    cube_face_edge_map.insert((2, 3), Direction::Right);
    cube_face_edge_map.insert((2, 4), Direction::Right);
    cube_face_edge_map.insert((2, 1), Direction::Left);

    cube_face_edge_map.insert((3, 2), Direction::Left);
    cube_face_edge_map.insert((3, 0), Direction::Up);
    cube_face_edge_map.insert((3, 5), Direction::Down);
    cube_face_edge_map.insert((3, 4), Direction::Down);

    cube_face_edge_map.insert((4, 2), Direction::Up);
    cube_face_edge_map.insert((4, 3), Direction::Up);
    cube_face_edge_map.insert((4, 5), Direction::Right);
    cube_face_edge_map.insert((4, 1), Direction::Up);

    cube_face_edge_map.insert((5, 3), Direction::Left);
    cube_face_edge_map.insert((5, 0), Direction::Left);
    cube_face_edge_map.insert((5, 1), Direction::Right);
    cube_face_edge_map.insert((5, 4), Direction::Left);
}

fn part2(lines: &[String], test: bool) -> usize {
    //  182170
    let mut path_list: Vec<Order> = vec![];
    let mut grid: HashMap<(usize, usize), Tile> = HashMap::new();
    let mut grid_lines: HashMap<usize, GridLine> = HashMap::new();
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
    let max_y = grid_lines.len() - 1;

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
    // println!("{} {}", max_x, max_y);

    // all of this is hard coded specific to my puzzle input
    // this will NOR work with example input
    let mut cube_face_map: HashMap<usize, CubeFace> = HashMap::new();
    let mut cube_face_edge_map: HashMap<(usize, usize), Direction> = HashMap::new();
    if test {
        define_test_input_cube(
            max_x as usize,
            max_y,
            &mut cube_face_map,
            &mut cube_face_edge_map,
        );
    } else {
        define_input_cube(
            max_x as usize,
            max_y,
            &mut cube_face_map,
            &mut cube_face_edge_map,
        );
    }

    let mut current_pos = ((grid_lines.get(&0).unwrap().start, 0), Direction::Right);

    for order in path_list.iter() {
        // println!("{:?} {:?}", order, current_pos);
        match order {
            Order::Move(steps) => {
                let new_pos = get_possible_position_on_face(
                    &grid,
                    &cube_face_map,
                    &cube_face_edge_map,
                    *steps,
                    current_pos.0,
                    &current_pos.1,
                );
                current_pos = new_pos;
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
    // println!("{:?}", current_pos);
    (current_pos.0 .0 + 1) * 4 + (current_pos.0 .1 + 1) * 1000 + get_direction_value(&current_pos.1)
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
    println!("{}", part2(&lines, false));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec![
            "        ...#",
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
            "10R5L5R10L4R5L5",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 6032);
        let result = part2(&lines, true);
        assert_eq!(result, 5031);
    }
}
