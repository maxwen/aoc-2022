use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use aoc_2022::read_lines_as_vec;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
enum Direction {
    North,
    NorthWest,
    NorthEast,
    South,
    SouthWest,
    SouthEast,
    West,
    East,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Space,
    Elve,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Elve {
    id: usize,
    pos: (i32, i32),
}

impl Elve {
    fn get_positions(&self) -> Vec<(i32, i32)> {
        let mut pos_list = vec![];
        for d in Direction::iter() {
            pos_list.push(get_positions_at_direction(self.pos, &d));
        }
        pos_list
    }

    fn move_elve(&mut self, move_pos: (i32, i32)) {
        self.pos = move_pos;
    }
    fn move_elve_wish(
        &self,
        grid: &HashMap<(i32, i32), Tile>,
        directions: &VecDeque<(Direction, Vec<Direction>)>,
    ) -> (i32, i32) {
        let all_free = self
            .get_positions()
            .iter()
            .filter(|pos| grid.get(&pos).is_none())
            .count()
            == 8;
        if !all_free {
            for d in directions.iter() {
                let move_possible =
                    d.1.iter()
                        .filter(|direction| {
                            grid.get(&get_positions_at_direction(self.pos, direction))
                                .is_none()
                        })
                        .count()
                        == 3;
                if move_possible {
                    return get_positions_at_direction(self.pos, &d.0);
                }
            }
        }
        self.pos
    }
}

fn get_positions_at_direction(pos: (i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::North => (pos.0, pos.1 - 1),
        Direction::NorthWest => (pos.0 - 1, pos.1 - 1),
        Direction::NorthEast => (pos.0 + 1, pos.1 - 1),
        Direction::South => (pos.0, pos.1 + 1),
        Direction::SouthWest => (pos.0 - 1, pos.1 + 1),
        Direction::SouthEast => (pos.0 + 1, pos.1 + 1),
        Direction::West => (pos.0 - 1, pos.1),
        Direction::East => (pos.0 + 1, pos.1),
    }
}

fn get_elves_area_space_tiles(grid: &HashMap<(i32, i32), Tile>) -> usize {
    let mut min_x: i32 = i32::MAX;
    let mut max_x: i32 = i32::MIN;
    let mut min_y: i32 = i32::MAX;
    let mut max_y: i32 = i32::MIN;

    for pos in grid.keys() {
        min_x = min(min_x, pos.0);
        min_y = min(min_y, pos.1);
        max_x = max(max_x, pos.0);
        max_y = max(max_y, pos.1);
    }

    let mut space_tiles = 0;
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            let pos = (x, y);
            if grid.get(&pos).is_none() {
                space_tiles += 1;
            }
        }
    }
    space_tiles
}

#[allow(dead_code)]
fn print_grid(grid: &HashMap<(i32, i32), Tile>) {
    let mut min_x: i32 = i32::MAX;
    let mut max_x: i32 = i32::MIN;
    let mut min_y: i32 = i32::MAX;
    let mut max_y: i32 = i32::MIN;

    for pos in grid.keys() {
        min_x = min(min_x, pos.0);
        min_y = min(min_y, pos.1);
        max_x = max(max_x, pos.0);
        max_y = max(max_y, pos.1);
    }

    println!("{}-{} {}-{}", min_x, max_x, min_y, max_y);
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            let pos = (x, y);
            match grid.get(&pos).unwrap_or(&Tile::Space) {
                Tile::Space => {
                    print!(".")
                }
                Tile::Elve => {
                    print!("#")
                }
            }
        }
        println!();
    }
}

fn update_grid(elves_map: &HashMap<usize, Elve>, grid: &mut HashMap<(i32, i32), Tile>) {
    grid.clear();
    for elve in elves_map.values() {
        grid.insert(elve.pos, Tile::Elve);
    }
}

fn part1(lines: &[String]) -> usize {
    // 3871
    let mut grid: HashMap<(i32, i32), Tile> = HashMap::new();
    let mut elves_map: HashMap<usize, Elve> = HashMap::new();
    let mut elve_id = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i32, y as i32);
            if c == '#' {
                let e = Elve { id: elve_id, pos };
                elves_map.insert(elve_id, e);
                elve_id += 1;
            }
        }
    }
    update_grid(&elves_map, &mut grid);

    let mut wish_direction_list: VecDeque<(Direction, Vec<Direction>)> = VecDeque::new();
    wish_direction_list.push_back((
        Direction::North,
        vec![Direction::North, Direction::NorthEast, Direction::NorthWest],
    ));
    wish_direction_list.push_back((
        Direction::South,
        vec![Direction::South, Direction::SouthEast, Direction::SouthWest],
    ));
    wish_direction_list.push_back((
        Direction::West,
        vec![Direction::West, Direction::NorthWest, Direction::SouthWest],
    ));
    wish_direction_list.push_back((
        Direction::East,
        vec![Direction::East, Direction::NorthEast, Direction::SouthEast],
    ));
    // println!("{:?}", wish_direction_list);

    for _ in 0..10 {
        // print_grid(&grid);
        let mut wishes: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
        for e in elves_map.values() {
            let move_pos = e.move_elve_wish(&grid, &wish_direction_list);
            if move_pos != e.pos {
                wishes
                    .entry(move_pos)
                    .and_modify(|list| list.push(e.id))
                    .or_insert(vec![e.id]);
            }
        }

        let moves = wishes
            .iter()
            .filter(|entry| entry.1.len() == 1)
            .collect::<Vec<_>>();

        for e in moves.iter() {
            let move_pos = e.0;
            let move_elve_id = e.1.first().unwrap();
            let elve = elves_map.get_mut(move_elve_id).unwrap();
            elve.move_elve(*move_pos);
        }
        wish_direction_list.rotate_left(1);
        update_grid(&elves_map, &mut grid);
    }
    get_elves_area_space_tiles(&grid)
}

fn part2(lines: &[String]) -> usize {
    0usize
}

fn main() {
    let lines = read_lines_as_vec("input/input_day23.txt").unwrap();

    // let lines = vec![
    //     "....#..", "..###.#", "#...#.#", ".#...##", "#.###..", "##.#.##", ".#..#..",
    // ]
    // .iter()
    // .map(|s| s.to_string())
    // .collect::<Vec<_>>();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let lines = vec![
            "....#..", "..###.#", "#...#.#", ".#...##", "#.###..", "##.#.##", ".#..#..",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 110);
        // let result = part2(&lines);
        // assert_eq!(result, 5031);
    }
}
