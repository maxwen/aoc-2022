use itertools::Itertools;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet, VecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use aoc_2022::read_lines_as_vec;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Ground,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
enum BlizzardDirection {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for BlizzardDirection {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(BlizzardDirection::Up),
            'v' => Ok(BlizzardDirection::Down),
            '>' => Ok(BlizzardDirection::Right),
            '<' => Ok(BlizzardDirection::Left),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Hash)]
enum Step {
    Up,
    Down,
    Left,
    Right,
    Wait,
}

fn get_possible_steps(grid: &Grid, free_cache_set: &FreeState, pos: (u16, u16)) -> Vec<Step> {
    let mut possible_steps: Vec<Step> = vec![];
    // IMPORTANT!!! we can only wait if current spot stays free
    if free_cache_set.data.contains(&pos) {
        possible_steps.push(Step::Wait);
    }

    for d in Step::iter() {
        if d == Step::Wait {
            continue;
        }
        if grid.start_pos.1 == 0 && pos == grid.start_pos && d != Step::Down {
            // just to prevent -1 overflow below
            continue;
        }

        if grid.start_pos.1 == grid.grid_height - 1 && pos == grid.start_pos && d != Step::Up {
            // just to prevent -1 overflow below
            continue;
        }

        let pos = get_step_position(pos, &d);
        if !free_cache_set.data.contains(&pos) {
            continue;
        }
        possible_steps.push(d)
    }

    possible_steps
}

fn get_step_position(pos: (u16, u16), step: &Step) -> (u16, u16) {
    match step {
        Step::Up => (pos.0, pos.1 - 1),
        Step::Down => (pos.0, pos.1 + 1),
        Step::Left => (pos.0 - 1, pos.1),
        Step::Right => (pos.0 + 1, pos.1),
        Step::Wait => pos,
    }
}

#[derive(Debug)]
struct Blizzard {
    id: u16,
    pos: (u16, u16),
    direction: BlizzardDirection,
}

impl Blizzard {
    fn move_blizzard(&mut self, grid: &Grid) {
        self.pos = self.get_next_blizzard_positions(grid);
    }

    fn get_next_blizzard_positions(&self, grid: &Grid) -> (u16, u16) {
        match self.direction {
            BlizzardDirection::Up => {
                if self.pos.1 == 1 {
                    (self.pos.0, grid.grid_height - 2)
                } else {
                    (self.pos.0, self.pos.1 - 1)
                }
            }
            BlizzardDirection::Down => {
                if self.pos.1 == grid.grid_height - 2 {
                    (self.pos.0, 1)
                } else {
                    (self.pos.0, self.pos.1 + 1)
                }
            }
            BlizzardDirection::Left => {
                if self.pos.0 == 1 {
                    (grid.grid_width - 2, self.pos.1)
                } else {
                    (self.pos.0 - 1, self.pos.1)
                }
            }
            BlizzardDirection::Right => {
                if self.pos.0 == grid.grid_width - 2 {
                    (1, self.pos.1)
                } else {
                    (self.pos.0 + 1, self.pos.1)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FreeState {
    data: HashSet<(u16, u16)>,
}

#[derive(Debug)]
struct Grid {
    grid_data: HashMap<(u16, u16), Tile>,
    grid_width: u16,
    grid_height: u16,
    blizzard_list: HashMap<u16, RefCell<Blizzard>>,
    start_pos: (u16, u16),
    end_pos: (u16, u16),
    start_time: u16,
}

impl Grid {
    fn get_blizzard_map(&self) -> HashMap<(u16, u16), u16> {
        let mut blizzard_map = HashMap::new();
        self.blizzard_list.iter().for_each(|b| {
            let blizzard = b.1.borrow();
            blizzard_map.insert(blizzard.pos, blizzard.id);
        });
        blizzard_map
    }

    fn get_free_pos_state(&self) -> FreeState {
        let mut state = FreeState {
            data: HashSet::new(),
        };
        let blizzard_state = self.get_blizzard_map();
        for y in 1..self.grid_height - 1 {
            for x in 1..self.grid_width - 1 {
                let pos = (x, y);
                if !blizzard_state.contains_key(&pos) {
                    state.data.insert(pos);
                }
            }
        }
        // start and end must always be free
        state.data.insert(self.start_pos);
        state.data.insert(self.end_pos);
        state
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Grid, expedition: (u16, u16)) {
    let blizzard_map = grid.get_blizzard_map();
    for y in 0..grid.grid_height {
        for x in 0..grid.grid_width {
            let pos = (x, y);

            if let Some(blizzard_id) = blizzard_map.get(&pos) {
                let blizzard = grid.blizzard_list.get(blizzard_id).unwrap().borrow();
                match blizzard.direction {
                    BlizzardDirection::Up => print!("^"),
                    BlizzardDirection::Down => print!("v"),
                    BlizzardDirection::Left => print!("<"),
                    BlizzardDirection::Right => print!(">"),
                }
            } else {
                if expedition == pos {
                    print!("E")
                } else {
                    match grid.grid_data.get(&pos).unwrap() {
                        Tile::Ground => {
                            print!(".")
                        }
                        Tile::Wall => {
                            print!("#")
                        }
                    }
                }
            }
        }
        println!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: (u16, u16),
    steps: u16,
}

// fn gcd(n1: u16, n2: u16) -> u16 {
//     let mut res = min(n1, n2);
//     while res > 0 {
//         if n1 % res == 0 && n2 % res == 0 {
//             break;
//         }
//         res -= 1;
//     }
//     res
// }
//
// fn lcm(n1: u16, n2: u16) -> u16 {
//     println!("lcm {} {}", n1, n2);
//     n1 * n2 / gcd(n1, n2)
// }
fn bfs(grid: &Grid, free_state_cache: &HashMap<u16, FreeState>) -> u16 {
    let mut stack: VecDeque<State> = VecDeque::new();
    let s = State {
        pos: grid.start_pos,
        steps: grid.start_time,
    };
    stack.push_back(s.clone());

    let mut seen: HashSet<State> = HashSet::new();

    let min = u16::MAX;

    while let Some(current) = stack.pop_front() {
        let steps = current.steps;
        let current_pos = current.pos;

        if current_pos == grid.end_pos {
            return steps;
        }

        // just so simple
        if seen.contains(&current) {
            continue;
        }
        seen.insert(current.clone());

        let current_free = free_state_cache
            .get(&(steps % free_state_cache.len() as u16))
            .unwrap();
        let possible_steps = get_possible_steps(grid, &current_free, current_pos);

        for step in possible_steps {
            let move_pos = get_step_position(current_pos, &step);

            let s_new = State {
                pos: move_pos,
                steps: steps + 1,
            };
            stack.push_back(s_new);
        }
    }
    min
}

fn init_grid(lines: &[String]) -> Grid {
    let mut grid = Grid {
        grid_data: HashMap::new(),
        grid_width: 0,
        grid_height: 0,
        blizzard_list: HashMap::new(),
        start_pos: (0, 0),
        end_pos: (0, 0),
        start_time: 0,
    };

    let mut blizzard_id = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as u16, y as u16);
            if c == '#' {
                grid.grid_data.insert(pos, Tile::Wall);
            } else if c == '.' {
                if y == 0 {
                    grid.start_pos = pos
                }
                if y == lines.len() - 1 {
                    grid.end_pos = pos
                }
                grid.grid_data.insert(pos, Tile::Ground);
            } else {
                grid.grid_data.insert(pos, Tile::Ground);
                let b = Blizzard {
                    id: blizzard_id,
                    pos,
                    direction: BlizzardDirection::try_from(c).unwrap(),
                };
                grid.blizzard_list.insert(blizzard_id, RefCell::new(b));
                blizzard_id += 1;
            }
            if y == 0 {
                grid.grid_width += 1;
            }
        }
        if line.len() != 0 {
            grid.grid_height += 1;
        }
    }
    grid
}

// creates a list of free pos for a specific time
fn create_free_space_map(grid: &Grid) -> HashMap<u16, FreeState> {
    // https://github.com/ritesh-singh/aoc-2022-kotlin/blob/main/src/day24/Day24.kt
    // let lcm = lcm(grid.grid_width - 2, grid.grid_height - 2);
    // println!("lcm = {}", lcm);

    // we where right - using lcm we could have calculated this too
    // puzzle input blizard pos repeats after 600 steps
    // test input blizzard pos repeats after 12 steps

    let mut steps: u16 = 0;
    let mut free_state_cache = HashMap::new();

    loop {
        grid.blizzard_list
            .iter()
            .for_each(|b| b.1.borrow_mut().move_blizzard(&grid));

        let free_state = grid.get_free_pos_state();
        if free_state_cache.values().contains(&free_state) {
            break;
        }
        free_state_cache.insert(steps, free_state);
        steps += 1;
    }
    free_state_cache
}

fn part1(lines: &[String]) -> u16 {
    // 253
    let grid = init_grid(lines);
    let free_state_cache = create_free_space_map(&grid);

    bfs(&grid, &free_state_cache)
}

fn part2(lines: &[String]) -> u16 {
    // 794
    let mut grid = init_grid(lines);
    let free_state_cache = create_free_space_map(&grid);

    let down = bfs(&grid, &free_state_cache);

    grid.start_time = down;
    let temp = grid.end_pos;
    grid.end_pos = grid.start_pos;
    grid.start_pos = temp;

    let up = bfs(&grid, &free_state_cache);

    grid.start_time = up;
    let temp = grid.start_pos;
    grid.start_pos = grid.end_pos;
    grid.end_pos = temp;

    let down2 = bfs(&grid, &free_state_cache);

    down2
}

fn main() {
    let lines = read_lines_as_vec("input/input_day24.txt").unwrap();

    // let lines = vec![
    //     "#.######", "#>>.<^<#", "#.<..<<#", "#>v.><>#", "#<^v^^>#", "######.#",
    // ]
    // .iter()
    // .map(|s| s.to_string())
    // .collect::<Vec<_>>();

    let lines = lines
        .into_iter()
        .filter(|s| s.len() != 0)
        .collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec![
            "#.######", "#>>.<^<#", "#.<..<<#", "#>v.><>#", "#<^v^^>#", "######.#",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let lines = lines
            .into_iter()
            .filter(|s| s.len() != 0)
            .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 18);
        let result = part2(&lines);
        assert_eq!(result, 54);
    }
}
